// agi_core/src/connectome.rs

use crate::neuron::Neuron;
use rand::Rng;
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;
use std::collections::{HashMap, HashSet};

/// Represents a connection between two neurons, using stable u64 IDs.
#[derive(Debug, Clone, Copy)]
pub struct Synapse {
    /// ID of the presynaptic (source) neuron.
    pub from: u64,
    /// ID of the postsynaptic (target) neuron.
    pub to: u64,
    /// The weight of the connection.
    pub weight: f32,
}

/// Represents the entire neural network, loaded from a binary file.
#[derive(Debug, Default)]
pub struct Connectome {
    pub neurons: Vec<Neuron>,
    pub synapses: Vec<Synapse>,
    pub outgoing_synapses: HashMap<u64, Vec<(u64, f32)>>,
    
    // --- Performance Optimization ---
    // A set of neurons whose potential is > 0. Only these are processed in the update loop.
    pub active_neurons: HashSet<u64>,

    // A rolling log of recent firing activity (neuron_id, tick).
    pub firing_history: Vec<(u64, u64)>,
}

impl Connectome {
    /// Updates the state of all neurons in the connectome.
    /// This includes decaying potential and checking for firing conditions.
    /// Returns a list of IDs for neurons that are currently firing.
    pub fn update(&mut self, current_tick: u64) -> Vec<u64> {
        // --- Spontaneous Activity ---
        // Add a small chance for any neuron to get a random potential boost,
        // simulating background noise and preventing the network from dying.
        let mut rng = rand::thread_rng();
        

        const SPONTANEOUS_BOOST_AMOUNT: f32 = 0.75;
        let num_to_boost = 2; // Boost a couple of random neurons each tick to ensure activity.

        if !self.neurons.is_empty() {
            for _ in 0..num_to_boost {
                let neuron_id = rng.gen_range(0..self.neurons.len());
                if let Some(neuron) = self.neurons.get_mut(neuron_id) {
                    neuron.potential += SPONTANEOUS_BOOST_AMOUNT;
                    if neuron.potential > 0.0 {
                        self.active_neurons.insert(neuron.id);
                    }
                }
            }
        }

        let mut firing_ids = Vec::new();
        let mut dormant_ids = Vec::new();

        // Iterate over a clone of the active set because we'll be modifying it.
        for &neuron_id in &self.active_neurons.clone() {
            if let Some(neuron) = self.neurons.get_mut(neuron_id as usize) {
                neuron.update(); // Handles decay and firing state change

                if neuron.firing {
                    firing_ids.push(neuron.id);
                }

                // If potential has decayed to zero, mark it for removal from the active list.
                if neuron.potential <= 0.0 {
                    dormant_ids.push(neuron.id);
                }
            }
        }

        // Remove dormant neurons from the active set.
        for id in dormant_ids {
            self.active_neurons.remove(&id);
        }

        // --- Update Firing History ---
        if !firing_ids.is_empty() {
            for &id in &firing_ids {
                self.firing_history.push((id, current_tick));
            }

            // Prune the history to keep it from growing indefinitely.
            let history_max_len = 500;
            if self.firing_history.len() > history_max_len {
                let to_remove = self.firing_history.len() - history_max_len;
                self.firing_history.drain(0..to_remove);
            }
        }

        firing_ids
    }

    /// Propagates a signal from a single firing neuron to its connected neurons using the optimized map.
    pub fn propagate_signal_from(&mut self, firing_neuron_id: u64) {
        // Use the pre-computed map for a fast lookup.
        if let Some(connections) = self.outgoing_synapses.get(&firing_neuron_id) {
            for &(to_id, weight) in connections {
                if let Some(neuron) = self.neurons.get_mut(to_id as usize) {
                    neuron.potential += weight;
                    // If the neuron is now active, add it to the list for the next update tick.
                    if neuron.potential > 0.0 {
                        self.active_neurons.insert(to_id);
                    }
                }
            }
        }
    }

    /// Creates a new Connectome by loading a quantized binary file.
    pub fn from_binary<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let mut file = File::open(path)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;

        if buffer.len() < 16 { // 2 * u64
            return Err(io::Error::new(io::ErrorKind::InvalidData, "File is too small to be a valid connectome."));
        }

        let num_neurons = u64::from_le_bytes(buffer[0..8].try_into().unwrap());
        let num_synapses = u64::from_le_bytes(buffer[8..16].try_into().unwrap());

        let mut neurons = Vec::with_capacity(num_neurons as usize);
        for i in 0..num_neurons {
            neurons.push(Neuron::new(i));
        }

        let mut synapses = Vec::with_capacity(num_synapses as usize);
        let mut cursor = 16;
        let synapse_size = std::mem::size_of::<u32>() * 2 + std::mem::size_of::<f32>(); // 4 + 4 + 4 = 12 bytes

        for _ in 0..num_synapses {
            if cursor + synapse_size > buffer.len() {
                 return Err(io::Error::new(io::ErrorKind::InvalidData, "Unexpected end of file while reading synapses."));
            }
            let from = u32::from_le_bytes(buffer[cursor..cursor+4].try_into().unwrap()) as u64;
            cursor += 4;
            let to = u32::from_le_bytes(buffer[cursor..cursor+4].try_into().unwrap()) as u64;
            cursor += 4;
            let weight = f32::from_le_bytes(buffer[cursor..cursor+4].try_into().unwrap());
            cursor += 4;

            synapses.push(Synapse { from, to, weight });
        }

        println!("Successfully loaded connectome: {} neurons, {} synapses.", neurons.len(), synapses.len());

        // --- Optimization Step: Pre-compute the outgoing synapse map ---
        let mut outgoing_synapses = HashMap::new();
        for synapse in &synapses {
            outgoing_synapses.entry(synapse.from)
                .or_insert_with(Vec::new)
                .push((synapse.to, synapse.weight));
        }

        Ok(Self { 
            neurons, 
            synapses, 
            outgoing_synapses, 
            firing_history: Vec::new(),
            active_neurons: HashSet::new(), // Initialize the active list
        })
    }

    /// Returns the IDs of neurons that have fired within a given recent window of ticks.
        /// Applies Long-Term Potentiation (LTP) to the synapses between a set of active neurons.
    /// This strengthens the connections within a pathway that just fired, making it easier to activate in the future.
    /// Applies a powerful, deep potentiation to synapses between a set of active neurons.
    /// This is used to engrave foundational, axiomatic memories into the connectome.
    pub fn deeply_engrave_pathway(&mut self, active_neuron_ids: &HashSet<u64>) {
        let potentiation_factor = 1.8; // e.g., 80% increase - much stronger than normal LTP
        let max_weight = 3.5; // Allow axioms to have a higher maximum weight

        for from_id in active_neuron_ids {
            if let Some(connections) = self.outgoing_synapses.get_mut(from_id) {
                for (to_id, weight) in connections.iter_mut() {
                    // If the target neuron was also part of the same firing event, strengthen the connection.
                    if active_neuron_ids.contains(to_id) {
                        *weight *= potentiation_factor;
                        if *weight > max_weight {
                            *weight = max_weight;
                        }
                    }
                }
            }
        }
    }

    pub fn potentiate_pathway(&mut self, active_neuron_ids: &HashSet<u64>) {
        let potentiation_factor = 1.1; // e.g., 10% increase
        let max_weight = 2.5; // Prevent runaway weights

        for from_id in active_neuron_ids {
            if let Some(connections) = self.outgoing_synapses.get_mut(from_id) {
                for (to_id, weight) in connections.iter_mut() {
                    // If the target neuron was also part of the same firing event, strengthen the connection.
                    if active_neuron_ids.contains(to_id) {
                        *weight *= potentiation_factor;
                        if *weight > max_weight {
                            *weight = max_weight;
                        }
                    }
                }
            }
        }
    }

    /// Returns the IDs of neurons that have fired within a given recent window of ticks.
    pub fn get_recent_firings(&self, current_tick: u64, window_size: u64) -> Vec<u64> {
        self.firing_history
            .iter()
            .filter_map(|&(neuron_id, tick)| {
                if current_tick.saturating_sub(tick) < window_size {
                    Some(neuron_id)
                } else {
                    None
                }
            })
            .collect()
    }
}
