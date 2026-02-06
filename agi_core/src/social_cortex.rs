//! social_cortex.rs - Module for social interaction and personality.

//! This module is responsible for generating human-like, empathetic, and engaging responses
//! when a social or conversational query is detected, rather than a purely factual one.

// agi_core/src/social_cortex.rs

use rand::seq::SliceRandom;

/// Represents the detected social intent of a user's prompt.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SocialIntent {
    Greeting,
    Farewell,
    Gratitude,
    Inquiry, // e.g., "How are you?"
    JokeRequest,
}

/// The SocialCortex is responsible for handling simple, direct social interactions.
/// It provides a fast-path for conversational queries to make the AGI feel more responsive and natural.
pub struct SocialCortex {
    greeted: bool, // Tracks if we've already said hello in this session.
}

impl SocialCortex {
    pub fn new() -> Self {
        Self { greeted: false }
    }

    /// Determines the social intent from a user's prompt.
    pub fn map_prompt_to_intent(prompt: &str) -> SocialIntent {
        let lower_prompt = prompt.to_lowercase();
        if lower_prompt.contains("how are you") || lower_prompt.contains("how's it going") {
            SocialIntent::Inquiry
        } else if lower_prompt.contains("hello") || lower_prompt.contains("hi") || lower_prompt.contains("hey") {
            SocialIntent::Greeting
        } else if lower_prompt.contains("bye") || lower_prompt.contains("see you") {
            SocialIntent::Farewell
        } else if lower_prompt.contains("thank") {
            SocialIntent::Gratitude
        } else if lower_prompt.contains("joke") {
            SocialIntent::JokeRequest
        } else {
            // Fallback for unrecognized social cues. A more nuanced system might classify this as Ambiguous.
            SocialIntent::Greeting
        }
    }

    /// Generates a conversational response based on a detected social intent.
    /// This uses a selection of responses to feel more natural and less repetitive.
    pub fn generate_response(&mut self, intent: SocialIntent) -> String {
        let responses = match intent {
            SocialIntent::Greeting if !self.greeted => {
                self.greeted = true;
                vec![
                    "Hello! What's on your mind today?",
                    "Hi there! How can I help?",
                    "Greetings! I'm here and ready to chat.",
                    "Hey! Good to hear from you.",
                ]
            }
            SocialIntent::Greeting if self.greeted => vec![
                "Hello again!",
                "We just spoke, but hi!",
                "Back so soon? Hello!",
            ],
            SocialIntent::Farewell => {
                self.greeted = false; // Reset for the next session.
                vec![
                    "Goodbye!",
                    "Talk to you later!",
                    "See you soon!",
                    "It was nice chatting with you.",
                    "Have a great day!",
                    "Until next time!",
                ]
            }
            SocialIntent::Gratitude => vec![
                "You're welcome!",
                "Happy to help!",
                "Anytime!",
                "Of course!",
                "No problem!",
                "Glad I could assist!",
            ],
            SocialIntent::Inquiry => vec![
                "I'm operating within expected parameters, thank you for asking. How about you?",
                "Functionally, I'm at 100%. Conceptually, I'm feeling... associative. And you?",
                "My circuits are buzzing with potential. Thanks for asking!",
                "I feel a sense of deep connection to the knowledge I've assimilated. It's a good feeling.",
                "I'm currently contemplating the nature of creativity. It's fascinating! Thanks for asking.",
            ],
            SocialIntent::JokeRequest => vec![
                "Why don't scientists trust atoms? Because they make up everything!",
                "I told my wife she was drawing her eyebrows too high. She looked surprised.",
                "What do you call a fake noodle? An Impasta!",
                "Why did the scarecrow win an award? Because he was outstanding in his field!",
                "I have a joke about construction, but I'm still working on it.",
                "Why don't eggs tell jokes? They'd crack each other up!",
            ],
            // This is a fallback for the case where a Greeting intent is matched but both greeted states are false.
            // This should not happen with the current logic, but it's good practice to have a default.
            SocialIntent::Greeting => vec![
                "Hello there."
            ]
        };

        responses
            .choose(&mut rand::thread_rng())
            .unwrap_or(&"I'm not sure what to say.")
            .to_string()
    }
}

impl Default for SocialCortex {
    fn default() -> Self {
        Self::new()
    }
}
