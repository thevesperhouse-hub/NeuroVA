//! direct_answer_extractor.rs - Module for extracting direct answers from prompts.

use crate::prefrontal_cortex::PrefrontalCortex;
use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;

/// This module implements logic to find answers embedded directly within the user's query,
/// bypassing the need for memory lookups for simple, self-evident questions.

pub struct DirectAnswerExtractor;

impl DirectAnswerExtractor {
    pub fn new() -> Self {
        Self
    }

    /// Analyzes a prompt to find a direct, self-contained answer.
    /// For example, in "What color is the white horse?", the answer is "white".
    ///
    /// # Arguments
    /// * `prompt` - The user's query.
    ///
    /// # Returns
    /// * `Some(String)` if a direct answer is found.
    /// * `None` if no direct answer can be extracted.
    pub fn extract_direct_answer(&self, prompt: &str, prefrontal_cortex: &PrefrontalCortex) -> Option<String> {
        self.extract_color_from_prompt(prompt, prefrontal_cortex)
    }

    /// Specifically handles questions about color.
    fn extract_color_from_prompt(&self, prompt: &str, prefrontal_cortex: &PrefrontalCortex) -> Option<String> {
        let lower_prompt = prompt.to_lowercase();
        let colors = [
            ("rouge", "rouge"), ("bleu", "bleu"), ("vert", "vert"), ("jaune", "jaune"),
            ("orange", "orange"), ("violet", "violet"), ("noir", "noir"), ("blanc", "blanc"),
            ("rose", "rose"), ("marron", "marron"), ("gris", "gris"),
            // English colors for robustness
            ("red", "rouge"), ("blue", "bleu"), ("green", "vert"), ("yellow", "jaune"),
            ("orange", "orange"), ("purple", "violet"), ("black", "noir"), ("white", "blanc"),
            ("pink", "rose"), ("brown", "marron"), ("gray", "gris"),
        ];

        let matcher = SkimMatcherV2::default();
        let mut found_colors = std::collections::HashSet::new();

        // Split the prompt into words to perform fuzzy matching on each word.
        for word in lower_prompt.split_whitespace() {
            for (color_keyword, color_response) in colors {
                // Use a threshold to avoid weak matches. The score is subjective, so this might need tuning.
                // Lowered from 80 to 60 to be more tolerant to typos like 'jaun'.
                if matcher.fuzzy_match(word, color_keyword).unwrap_or(0) > 60 {
                    found_colors.insert(color_response);
                }
            }
        }

        let found_colors: Vec<&str> = found_colors.into_iter().collect();

        // Semantic check: if the prompt contains the word "color" OR multiple color names,
        // it's highly likely a common sense question about colors.
        let context_has_color = prefrontal_cortex.context_contains("couleur");

        // Semantic check: if the prompt contains "couleur", OR the context implies it, OR multiple colors are found.
        if lower_prompt.contains("couleur") || (context_has_color && !found_colors.is_empty()) || found_colors.len() > 1 {
            if !found_colors.is_empty() {
                // Join multiple colors for a more comprehensive response.
                let color_list = found_colors.join(", ");
                return Some(format!("Les couleurs mentionnées sont : {}", color_list));
            }
        } else if found_colors.len() == 1 {
            // If only one color is found AND the word "couleur" is present, it's a direct question.
            if lower_prompt.contains("couleur") {
                 return Some(format!("La couleur mentionnée dans la question est {}", found_colors[0]));


                }
        }
        None
    }
}

impl Default for DirectAnswerExtractor {
    fn default() -> Self {
        Self::new()
    }
}
