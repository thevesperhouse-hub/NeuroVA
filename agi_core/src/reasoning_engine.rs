//! The Reasoning Engine is responsible for logical thought, deduction,
//! and retrieving relevant information from memory based on a specific prompt.

use crate::holographic_memory::{HolographicEncoder, HolographicMemory};
use crate::hippocampus::Hippocampus;
use crate::conceptual_hierarchy::ConceptualHierarchy;
use std::sync::{Arc, RwLock};

pub struct ReasoningEngine;

impl ReasoningEngine {
    pub fn new() -> Self {
        Self
    }

    /// Scores the plausibility of a given assertion against the knowledge in the hippocampus.
    ///
    /// # Returns
    /// A plausibility score between 0.0 and 1.0.
    pub fn score_assertion(
        &self,
        assertion: &str,
        hippocampus: &Hippocampus,
        encoder: &Arc<RwLock<HolographicEncoder>>,
    ) -> f32 {
        if assertion.is_empty() {
            return 0.0;
        }

        // Encode the assertion into a holographic trace.
        let assertion_trace = encoder.read().unwrap().encode(assertion);

        // Find the most similar memory in the hippocampus.
        let search_results = hippocampus.find_similar_memories(&assertion_trace, 1, false);

        // The score is the distance (lower is better), so we convert it to similarity (higher is better).
        // A distance of 0.0 is a perfect match (similarity 1.0).
        // A distance > 1.0 is considered no similarity.
        search_results.get(0).map_or(0.0, |(_, distance)| (1.0 - distance).max(0.0))
    }

    pub fn process(
        &self,
        prompt: &str,
        hippocampus: &Hippocampus,
        _conceptual_hierarchy: &ConceptualHierarchy,
        holographic_encoder: &Arc<RwLock<HolographicEncoder>>,
        is_introspective: bool,
        distance_threshold: f32, // Le seuil est maintenant dynamique
    ) -> Option<Vec<HolographicMemory>> {
        let prompt_trace = holographic_encoder.read().unwrap().encode(prompt);

        // Search for the top 5 most relevant memories to get a richer context.
        let search_results = hippocampus.find_similar_memories(&prompt_trace, 5, is_introspective);

        // Filter and sort the results.
        let mut relevant_memories: Vec<(HolographicMemory, f32)> = search_results
            .into_iter()
            .filter(|(_, distance)| {
                // For introspective queries, we are searching a very small, curated set of axioms.
                // The exact distance is less important than the fact they are axioms.
                // We bypass the distance check for these queries.
                if is_introspective {
                    true
                } else {
                    *distance < distance_threshold
                }
            })
            .map(|(mem, dist)| (mem.clone(), dist)) // Clone the memory to take ownership
            .collect();

        // Sort by distance (ascending) to ensure the most relevant memory is first.
        relevant_memories.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));

        // Map to just the memories, discarding the distance.
        let final_memories: Vec<HolographicMemory> = relevant_memories
            .into_iter()
            .map(|(mem, _)| mem)
            .collect();

        if final_memories.is_empty() {
            None
        } else {
            Some(final_memories)
        }
    }
}
