// agi_core/src/neuron.rs

/// Represents the state of a single neuron.
#[derive(Debug, Clone)]
pub struct Neuron {
    /// Unique identifier for the neuron.
    pub id: u64,
    /// The current membrane potential.
    pub potential: f32,
    /// The firing threshold.
    pub threshold: f32,
    /// Is the neuron currently firing?
    pub firing: bool,
    /// Rate at which the potential leaks, returning to a resting state.
    pub leak_factor: f32,
}

impl Neuron {
    pub fn new(id: u64) -> Self {
        Self {
            id,
            potential: 0.0, // Start at rest
            threshold: 1.0, // Example threshold
            firing: false,
            leak_factor: 0.01, // Reduced leak to encourage cascades
        }
    }

    /// Updates the neuron's state for one time step using a leaky integrate-and-fire model.
    pub fn update(&mut self) {
        // 1. If the neuron was firing on the last tick, reset it now.
        // This happens *before* we check for a new firing event in the current tick.
        // This gives the 'firing' state a full tick to be observed by the rest of the system.
        if self.firing {
            self.potential = 0.0; // Reset potential to resting state.
            self.firing = false;   // End the firing state.
        }

        // 2. Check if the current potential exceeds the firing threshold.
        if self.potential >= self.threshold {
            self.firing = true;
        }

        // 3. Apply the 'leak' to the potential.
        if self.potential > 0.0 {
            self.potential *= 1.0 - self.leak_factor;
            if self.potential < 1e-6 {
                self.potential = 0.0;
            }
        }
    }
}
