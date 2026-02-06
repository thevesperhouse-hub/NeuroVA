// agi_core/src/hippocampus.rs
use crate::holographic_memory::{HolographicMemory, HolographicTrace};
use crate::quantum::Qubit;
use rand::Rng;
use std::collections::HashSet;

/// Represents a memory pattern as a set of associated qubit indices.
#[derive(Debug, Clone)]
pub struct MemoryPattern {
    pub qubit_indices: Vec<usize>,
}

/// Represents the Hippocampus, responsible for memory encoding and retrieval.
#[derive(Debug)]
pub struct Hippocampus {
    core_memories: Vec<MemoryPattern>,
    pub holographic_memory: Vec<HolographicMemory>,
}

impl Hippocampus {
    pub fn new() -> Self {
        let patterns = vec![
            MemoryPattern { qubit_indices: vec![0, 3, 5, 7] },
            MemoryPattern { qubit_indices: vec![1, 2, 6] },
            MemoryPattern { qubit_indices: vec![0, 4] },
        ];

        Hippocampus {
            core_memories: patterns,
            holographic_memory: Vec::new(),
        }
    }

    pub fn add_holographic_memory(&mut self, text: String, trace: HolographicTrace, is_axiom: bool) {
        let new_memory = HolographicMemory {
            text,
            trace,
            is_axiom,
        };
        if is_axiom {
            println!("--- Foundational Axiom Encoded: '{}' ---", new_memory.text);
        } else {
            println!("--- New Holographic Memory Encoded: '{}' ---", new_memory.text);
        }
        self.holographic_memory.push(new_memory);
    }

    /// Finds the top_k most similar holographic memories to a given query trace.
    pub fn find_similar_memories<'a>(
        &'a self,
        query_trace: &HolographicTrace,
        top_k: usize,
        is_introspective: bool,
    ) -> Vec<(&'a HolographicMemory, f32)> {
        if self.holographic_memory.is_empty() {
            return Vec::new();
        }

        let memories_to_search: Vec<_> = if is_introspective {
            // For introspective queries, we perform a targeted search ONLY on foundational axioms.
            println!("--- Introspective query: Searching foundational axioms... ---");
            self.holographic_memory.iter().filter(|mem| mem.is_axiom).collect()
        } else {
            // For all other queries, proceed with the normal semantic distance search.
            println!("--- Factual/Creative query: Searching full knowledge base... ---");
            self.holographic_memory.iter().collect()
        };

        let mut scored_memories: Vec<(&'a HolographicMemory, f32)> = memories_to_search
            .into_iter()
            .filter_map(|memory| {
                let distance = query_trace.distance(&memory.trace);
                if distance.is_nan() {
                    None
                } else {
                    Some((memory, distance))
                }
            })
            .collect();

        // Sort by distance, ascending (smallest distance is most similar)
        scored_memories.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));

        // --- Diagnostic Logging ---
        println!("--- Top 5 Raw Search Results (Distance): ---");
        for (memory, distance) in scored_memories.iter().take(5) {
            println!("  - Distance: {:.4}, Text: '{}'", distance, memory.text);
        }
        // --- End Diagnostic Logging ---


        // --- Deduplication Step ---
        // Ensures that the AGI doesn't repeat itself by returning memories with the exact same text.
        let mut unique_memories = Vec::with_capacity(top_k);
        let mut seen_texts = HashSet::new();

        for (memory, score) in scored_memories {
            if seen_texts.insert(&memory.text) { // Check for uniqueness based on text content
                unique_memories.push((memory, score));
                if unique_memories.len() >= top_k {
                    break;
                }
            }
        }

        unique_memories
    }

    pub fn get_random_pattern(&self) -> Option<&MemoryPattern> {
        if self.core_memories.is_empty() {
            None
        } else {
            let mut rng = rand::thread_rng();
            self.core_memories.get(rng.gen_range(0..self.core_memories.len()))
        }
    }

    pub fn replay_core_memories(&self, quantum_core: &mut [Qubit]) {
        println!("\n--- Hippocampal Replay Initiated ---");
        let priming_strength = 0.1;

        for pattern in &self.core_memories {
            println!("Replaying memory pattern: {:?}", pattern.qubit_indices);
            for &qubit_index in &pattern.qubit_indices {
                if let Some(qubit) = quantum_core.get_mut(qubit_index) {
                    qubit.beta.re += priming_strength;
                    qubit.alpha.re -= priming_strength;
                    qubit.alpha.re = qubit.alpha.re.max(0.0);
                    qubit.beta.re = qubit.beta.re.max(0.0);
                    qubit.normalize();
                }
            }
        }
        println!("--- Hippocampal Replay Complete ---\n");
    }
}
