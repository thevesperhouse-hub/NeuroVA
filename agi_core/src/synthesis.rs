//! Concept Synthesizer: The module responsible for advanced concept manipulation,
//! including learning new hierarchical relationships from direct commands.

use crate::Core;

pub struct ConceptSynthesizer;

impl ConceptSynthesizer {
    // The `new` method is no longer needed as we will use a static method.

    /// Processes a text prompt to check for special learning commands.
    /// If a command is found, it executes it and returns Some(response).
    /// Otherwise, it returns None, indicating normal processing should continue.
    /// This is a static method, so it doesn't take `&self`.
    pub fn process_command(text: &str, core: &mut Core) -> Option<String> {
        let trimmed = text.trim();
        if trimmed.starts_with("!learn") {
            // Expected format: "!learn child > parent"
            let parts: Vec<&str> = trimmed.split_whitespace().collect();
            if parts.len() == 4 && parts[2] == ">" {
                let child_name = parts[1];
                let parent_name = parts[3];

                // 1. Learn the structural relationship.
                core.learn_relationship(child_name, parent_name);

                // 2. Create and learn a semantic memory to make the concept retrievable.
                let semantic_memory = format!("le {} est un {}.", child_name, parent_name);
                core.learn_and_assimilate(&semantic_memory, false);

                // 3. Return a more comprehensive confirmation message.
                Some(format!("Understood. I've linked '{}' to '{}' and created a memory: '{}'", child_name, parent_name, semantic_memory))
            } else {
                Some("Invalid !learn command format. Please use: !learn <child> > <parent>".to_string())
            }
        } else {
            None
        }
    }
}
