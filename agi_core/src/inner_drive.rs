//! The Inner Drive module is responsible for generating autonomous thoughts,
//! goals, and internal stimuli, driving the AGI to think even without external prompts.

use crate::holographic_memory::HolographicMemory;
use rand::seq::SliceRandom;
use std::time::{Duration, Instant};

/// Represents the source of the AGI's autonomous motivation.
pub struct InnerDrive {
    last_thought_instant: Instant,
    thought_interval: Duration,
    is_contextual_turn: bool, // To alternate between contextual and isolation thoughts
}

impl InnerDrive {
    pub fn new(thought_interval_seconds: u64) -> Self {
        Self {
            last_thought_instant: Instant::now(),
            thought_interval: Duration::from_secs(thought_interval_seconds),
            is_contextual_turn: true,
        }
    }

    /// Called on each AGI core tick. If enough time has passed, it generates an internal stimulus.
    pub fn tick(&mut self, last_reasoning_result: Option<&str>, memories: &Vec<HolographicMemory>) -> Option<String> {
        if self.last_thought_instant.elapsed() < self.thought_interval {
            return None;
        }
        self.last_thought_instant = Instant::now();

        let thought = if self.is_contextual_turn {
            // On a contextual turn, try to use the last reasoning result.
            last_reasoning_result
                .and_then(|context| self.generate_contextual_prompt(context))
                .or_else(|| {
                    // Fallback to a random memory if context is not useful
                    println!("--- Inner Drive (Contextual Fallback) ---");
                    self.generate_isolation_prompt(memories)
                })
        } else {
            // On an isolation turn, always use a random memory.
            println!("--- Inner Drive (Isolation) ---");
            self.generate_isolation_prompt(memories)
        };

        self.is_contextual_turn = !self.is_contextual_turn; // Flip the turn for next time

        if let Some(ref prompt) = thought {
            println!("--- Inner Drive generated a thought: '{}' ---", prompt);
        }
        thought
    }

    /// Generates a prompt from a random memory, acting as an 'isolation' thought.
    fn generate_isolation_prompt(&self, memories: &Vec<HolographicMemory>) -> Option<String> {
        let mut rng = rand::thread_rng();
        memories.choose(&mut rng).and_then(|mem| self.generate_contextual_prompt(&mem.text))
    }

    /// Generates a prompt based on a given context (last reasoning result or a random memory).
    fn generate_contextual_prompt(&self, context: &str) -> Option<String> {
        const STOP_WORDS: &[&str] = &[
            // French
            "le", "la", "les", "un", "une", "des", "ce", "cet", "cette", "ces", "mon", "ton", "son", "ma", "ta", "sa", "mes", "tes", "ses",
            "quel", "quelle", "quels", "quelles", "qui", "que", "quoi", "dont", "où", "je", "tu", "il", "elle", "nous", "vous", "ils", "elles",
            "au", "aux", "avec", "dans", "de", "du", "en", "et", "est", "pour", "par", "sur", "ne", "pas", "plus", "comme", "mais", "si",
            "cela", "ça", "ici", "ont", "été", "lui", "eux", "moi", "toi", "sommes", "êtes", "sont", "absolument", "c'est", "d'un", "d'une",
            // English
            "the", "a", "an", "i", "it", "is", "in", "on", "at", "for", "with", "from", "by", "to", "of", "and", "are", "was", "were",
            "he", "she", "they", "we", "you", "me", "him", "her", "us", "them", "my", "your", "his", "its", "our", "their",
            "what", "which", "who", "whom", "this", "that", "these", "those", "am", "be", "been", "being", "have", "has", "had", "having",
            "do", "does", "did", "doing", "will", "would", "should", "can", "could", "not"
        ];

        let keywords: Vec<&str> = context.split_whitespace()
            .map(|word| word.trim_matches(|c: char| !c.is_alphanumeric()))
            .filter(|word| {
                if word.is_empty() || word.len() <= 2 { return false; }
                let first_char_is_upper = word.chars().next().unwrap().is_uppercase();
                let lower_word = word.to_lowercase();
                let is_not_stop_word = !STOP_WORDS.contains(&lower_word.as_str());
                first_char_is_upper && is_not_stop_word
            })
            .collect();

        let mut rng = rand::thread_rng();
        if let Some(keyword) = keywords.choose(&mut rng) {
            let templates = [
                "Comment le concept de '{}' pourrait-il s'appliquer à un autre domaine, comme l'art ?",
                "Quelles sont les implications éthiques de '{}' ?",
                "Existe-t-il une analogie historique pour '{}' ?",
                "Si '{}' est la réponse, quelle pourrait être la question ?",
                "Quel est le principe opposé à '{}' ?",
                "Comment pourrais-je expliquer '{}' à un enfant ?"
            ];
            templates.choose(&mut rng).map(|template| template.replace("{}", keyword))
        } else {
            None
        }
    }
}
