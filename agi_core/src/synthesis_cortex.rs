//! The Synthesis Cortex is responsible for weaving multiple streams of thought
//! or retrieved memories into a single, coherent, and natural-sounding response.

use crate::holographic_memory::{HolographicEncoder, HolographicMemory};

pub struct SynthesisCortex;

impl SynthesisCortex {
    pub fn new() -> Self {
        Self
    }

    /// Synthesizes a response from a collection of memories, guided by the original prompt.
    pub fn synthesize(
        &self,
        memories: Vec<HolographicMemory>,
        original_prompt: &str,
                _encoder: &HolographicEncoder,
    ) -> String {
        if memories.is_empty() {
            return "No memories could be retrieved for this query.".to_string();
        }

        if memories.len() == 1 {
            // If there's only one memory, just return its text directly.
            return memories[0].text.clone();
        }

        // More advanced synthesis: structure the response based on the prompt's sub-queries.
        // With the new multi-search architecture in lib.rs, `memories` contains pre-vetted, relevant items.
        // The job of synthesis is now to structure them, not to re-match them.

        if memories.len() > 1 {
            let mut response_parts = Vec::new();
            let _sub_queries: Vec<String> = original_prompt
                .split(|c| c == ',' || c == ';')
                .flat_map(|part| part.split(" et "))
                .flat_map(|part| part.split(" and "))
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();
            
            // Simple association if counts match, otherwise list all thoughts.
            // Create a structured list of thoughts.
            for mem in memories {
                // Format each memory as a bullet point.
                response_parts.push(format!("- {}", &mem.text));
            }

            format!(
                "Based on your query, I have synthesized the following points:\n\n{}",
                response_parts.join("\n")
            )
        } else {
            memories.first().map_or_else(
                || "No relevant information found.".to_string(),
                |mem| mem.text.clone(),
            )
        }
    }
}
