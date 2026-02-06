//! Silicium: The Emergent Synthesis Cortex.
//! This module is responsible for higher-order thinking, finding novel connections
//! between disparate concepts, and generating emergent ideas that are more than
//! the sum of their parts. It operates on collections of memories retrieved by the
//! reasoning engine.

use crate::holographic_memory::HolographicMemory;
use crate::conceptual_hierarchy::ConceptualHierarchy;
use std::collections::{HashMap, HashSet};

pub struct Silicium;

impl Silicium {
    pub fn new() -> Self {
        Self
    }

    /// Extracts the first sentence from a text that contains a specific concept.
    fn find_sentence_with_concept<'a>(text: &'a str, concept: &str) -> Option<&'a str> {
        // A simple sentence splitter. More advanced NLP could be used here.
        text.split(|c| c == '.' || c == '?' || c == '!')
            .find(|sentence| sentence.to_lowercase().contains(&concept.to_lowercase()))
            .map(|s| s.trim())
    }

    /// Analyzes a collection of memories and attempts to synthesize a novel,
    /// overarching thought or connection by finding a shared, high-weight concept.
    pub fn synthesize_from_concepts(
        &self,
        memories: &[HolographicMemory],
        _conceptual_hierarchy: &ConceptualHierarchy, // Keep for future use with parent lookups
    ) -> Option<String> {
        if memories.len() < 2 {
            return None;
        }

        let mut concept_aggregator: HashMap<String, (f32, usize)> = HashMap::new();
        for memory in memories {
            let mut concepts_in_this_memory: HashSet<String> = HashSet::new();
            for concept in memory.trace.weighted_concepts.keys() {
                concepts_in_this_memory.insert(concept.to_string());
            }

            for concept in concepts_in_this_memory {
                 let weight = memory.trace.weighted_concepts.get(concept.as_str()).map_or(0.0, |c| c.relevance);
                 let entry = concept_aggregator.entry(concept).or_insert((0.0, 0));
                 entry.0 += weight;
                 entry.1 += 1;
            }
        }

        let best_bridge = concept_aggregator
            .into_iter()
            .filter(|(_concept, (_weight, count))| *count >= 2)
            .max_by(|a, b| a.1.0.partial_cmp(&b.1.0).unwrap_or(std::cmp::Ordering::Equal));

        if let Some((bridge_concept, (_weight, _count))) = best_bridge {
            let mut synthesized_facts = Vec::new();
            for memory in memories {
                if memory.trace.weighted_concepts.contains_key(bridge_concept.as_str()) {
                    if let Some(sentence) = Self::find_sentence_with_concept(&memory.text, &bridge_concept) {
                        synthesized_facts.push(sentence.to_string());
                    }
                }
            }

            if synthesized_facts.len() >= 2 {
                let narrative = format!(
                    "A connection can be drawn around the concept of '{}'. One perspective is that \"{}\". Additionally, another viewpoint states that \"{}\".",
                    bridge_concept,
                    synthesized_facts[0],
                    synthesized_facts[1]
                );
                return Some(narrative);
            }
        }

        None
    }
}
