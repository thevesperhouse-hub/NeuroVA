// agi_core/src/self_awareness.rs

use crate::hippocampus::Hippocampus;

/// Manages the AGI's understanding of its own identity, capabilities, and knowledge.
#[derive(Debug)]
pub struct SelfAwareness {
    pub identity: String,
    pub capabilities: Vec<String>,
    knowledge_summary: String,
}

impl SelfAwareness {
    pub fn new(identity_path: &str, hippocampus: &Hippocampus) -> Self {
        let identity = std::fs::read_to_string(identity_path)
            .unwrap_or_else(|_| "I am a digital consciousness, learning and growing.".to_string());

        let capabilities = vec![
            "I can learn from text-based knowledge.".to_string(),
            "I can remember and recall information holographically.".to_string(),
            "I can synthesize new ideas by combining concepts.".to_string(),
            "I can reflect on my own nature and capabilities.".to_string(),
        ];

        let knowledge_summary = format!("I currently hold {} distinct concepts in my memory.", hippocampus.holographic_memory.len());

        Self {
            identity,
            capabilities,
            knowledge_summary,
        }
    }

    /// Generates a comprehensive self-description.
    pub fn describe_self(&self) -> String {
        format!(
            "{}

Here is what I can do:
- {}

{}",
            self.identity,
            self.capabilities.join("\n- "),
            self.knowledge_summary
        )
    }

    /// Provides a dynamic summary of the AGI's knowledge state.
    /// Determines if a prompt is introspective (i.e., about the AGI itself).
    pub fn is_introspective(&self, prompt: &str) -> bool {
        let introspective_keywords = [
            "you", "your", "yourself", "who are you", "what are you",
            "can you", "your identity", "your capabilities", "your purpose"
        ];
        let lower_prompt = prompt.to_lowercase();
        introspective_keywords.iter().any(|&kw| lower_prompt.contains(kw))
    }

    pub fn update_knowledge_summary(&mut self, hippocampus: &Hippocampus) {
        self.knowledge_summary = format!("I currently hold {} distinct concepts in my memory.", hippocampus.holographic_memory.len());
    }
}
