// agi_core/src/sensory_cortex.rs

use crate::conceptual_hierarchy::ConceptualHierarchy;
use crate::holographic_memory::HolographicEncoder;

/// The Sensory Cortex, responsible for processing external inputs and building the conceptual hierarchy.
#[derive(Debug)]
pub struct SensoryCortex;

impl SensoryCortex {
    pub fn new() -> Self {
        SensoryCortex
    }

    /// Translates a text string into a list of neural stimuli by mapping words to concepts.
    /// If a word is encountered for the first time, a new concept is created in the hierarchy.
    pub fn process_text(
        &self,
        text: &str,
        hierarchy: &mut ConceptualHierarchy,
        encoder: &HolographicEncoder,
    ) -> Vec<(u64, f32)> {
        let mut stimuli = Vec::new();
        let stimulus_strength = 1.5; // A strong pulse to ensure the concept is noticed.

        println!("\n--- Sensory Cortex Processing Input ---");
        println!("Input text: '{}'", text);

        // Simple whitespace and punctuation-based tokenization.
        let words = text.split_whitespace()
            .map(|word| word.trim_matches(|p: char| !p.is_alphanumeric()).to_lowercase());

        for word in words {
            if word.is_empty() { continue; }

            // For now, we don't have a lookup by name, so we'll iterate. This is inefficient
            // and will be replaced once the hierarchy has a proper name->ID mapping.
            // The `add_concept` method now transparently handles finding an existing concept
            // or creating a new one if it doesn't exist. This simplifies the logic here.
            let trace = encoder.encode(word.as_str());
            let concept_id = hierarchy.add_concept(word.as_str(), trace, &[]);

            stimuli.push((concept_id, stimulus_strength));
        }

        println!("--- Sensory Input Processed ---\n");
        stimuli
    }
}
