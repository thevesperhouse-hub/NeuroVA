//! The engine for creative, analogical, and associative reasoning.

use crate::conceptual_hierarchy::ConceptualHierarchy;

pub struct CuriosityEngine;

impl CuriosityEngine {
    pub fn new() -> Self {
        Self
    }

    /// Finds concepts in different domains that share structural similarities.
    /// This is the core of analogical reasoning.
    ///
    /// # Arguments
    /// * `concept_id` - The ID of the concept to start the search from.
    /// * `hierarchy` - A reference to the conceptual hierarchy to search within.
    ///
    /// # Returns
    /// A list of tuples, where each tuple contains the ID of an analogous concept
    /// and a string describing the nature of the analogy.
    pub fn find_analogies(&self, concept_id: u64, hierarchy: &ConceptualHierarchy) -> Vec<(u64, String)> {
        let mut analogies = Vec::new();

        let source_concept = match hierarchy.get_concept(concept_id) {
            Some(c) => c,
            None => return analogies, // Source concept doesn't exist.
        };

        if source_concept.domains.is_empty() {
            return analogies; // Can't find analogies without at least one domain.
        }

        // 1. Iterate through the domains of the source concept.
        for &domain_id in &source_concept.domains {
            let domain_name = hierarchy.get_concept(domain_id).map_or("unknown domain", |d| &d.name);

            // 2. Iterate through ALL concepts to find others in the same domain.
            for other_concept in hierarchy.get_all_concepts() {
                // Skip self and concepts that are not in the current domain.
                if other_concept.id == source_concept.id || !other_concept.domains.contains(&domain_id) {
                    continue;
                }

                // 3. We found a pair! Generate an analogy.
                let analogy_text = format!(
                    "Analogy in '{}': How might the principles of '{}' apply to '{}'?",
                    domain_name, source_concept.name, other_concept.name
                );

                analogies.push((other_concept.id, analogy_text));
            }
        }

        analogies
    }
}
