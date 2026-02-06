// agi_core/src/prefrontal_cortex.rs

use crate::holographic_memory::{ConceptFocuser, HolographicMemory};
use std::collections::HashSet;

/// The PrefrontalCortex is responsible for higher-order cognitive functions:
/// - Executive decision-making
/// - Synthesizing information from various sources (like the hippocampus)
/// - Maintaining conversational context
/// - Generating final, coherent responses
#[derive(Debug)]
pub struct PrefrontalCortex {
    _concept_focuser: ConceptFocuser,
    conversation_context: Vec<String>,
}

impl PrefrontalCortex {
    pub fn new(concept_focuser: ConceptFocuser) -> Self {
        Self {
            _concept_focuser: concept_focuser,
            conversation_context: Vec::new(),
        }
    }

    /// Updates the conversational context with the latest prompt.
    pub fn update_context(&mut self, prompt: &str) {
        self.conversation_context.push(prompt.to_string());
        // Limit the context size to avoid infinite growth
        if self.conversation_context.len() > 20 {
            self.conversation_context.remove(0);
        }
    }

    /// Checks if the recent conversation history contains a given keyword.
    pub fn context_contains(&self, keyword: &str) -> bool {
        self.conversation_context.iter().any(|prompt| prompt.to_lowercase().contains(keyword))
    }

    /// Synthesizes a coherent response from a collection of relevant memories.
    /// This is a crucial step up from simply returning the top-ranked memory.
    pub fn synthesize_response(&self, _original_prompt: &str, memories: &[HolographicMemory]) -> String {
        if memories.is_empty() {
            return "Je ne parviens pas à trouver d'informations pertinentes pour répondre.".to_string();
        }

        // If all memories are axioms (likely an introspective query), combine them for a full self-description.
        if !memories.is_empty() && memories.iter().all(|m| m.is_axiom) {
            return memories.iter()
                .map(|m| m.text.trim())
                .collect::<Vec<&str>>().join("\n");
        }

        // For general queries, combine the most relevant parts of each memory.
        let mut combined_text = String::new();
        let mut used_sentences = HashSet::new();

        for memory in memories {
            // Simple synthesis: just take the first sentence of each memory if it's not a duplicate.
            if let Some(first_sentence) = memory.text.split('.').next() {
                let sentence = first_sentence.trim();
                if !sentence.is_empty() && used_sentences.insert(sentence.to_string()) {
                    combined_text.push_str(sentence);
                    combined_text.push_str(". ");
                }
            }
        }

        if combined_text.is_empty() {
            // Fallback to the most relevant memory if synthesis fails
            memories[0].text.clone()
        } else {
            combined_text.trim().to_string()
        }
    }

    /// Synthesizes a thought from a collection of relevant memories and a query.
    /// This function now acts as the primary synthesis engine.
    pub fn synthesize_thought(&self, memories: &[HolographicMemory], query: &str) -> Option<String> {
        Some(self.synthesize_response(query, memories))
    }
}

