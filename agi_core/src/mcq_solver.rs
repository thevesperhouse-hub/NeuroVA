//! `mcq_solver` - Module pour la détection et la résolution de Questions à Choix Multiples (QCM).
//!
//! Ce module identifie si un prompt est un QCM et utilise le moteur de raisonnement
//! pour évaluer les options et trouver la réponse la plus probable.

use crate::holographic_memory::{HolographicEncoder, HolographicMemory};
use crate::reasoning_engine::ReasoningEngine;
use crate::hippocampus::Hippocampus;
use regex::Regex;
use std::sync::{Arc, Mutex, RwLock};

/// Represents a parsed Multiple Choice Question.
struct ParsedMCQ {
    question: String,
    options: Vec<String>,
}

/// Structure principale pour le solveur de QCM.
pub struct McqSolver {
    reasoning_engine: Arc<Mutex<ReasoningEngine>>,
}

impl McqSolver {
    pub fn new(reasoning_engine: Arc<Mutex<ReasoningEngine>>) -> Self {
        Self { reasoning_engine }
    }

    /// Tente de détecter et de résoudre un QCM à partir d'un prompt.
    ///
    /// # Retourne
    /// `Some(HolographicMemory)` avec la réponse si le prompt est un QCM et qu'une réponse est trouvée.
    /// `None` si le prompt n'est pas identifié comme un QCM.
    pub fn solve(&self, prompt: &str, hippocampus: &Hippocampus, encoder: &Arc<RwLock<HolographicEncoder>>) -> Option<HolographicMemory> {
        let parsed_mcq = self.parse_mcq(prompt)?;

        let mut best_option: Option<String> = None;
        let mut max_score = -1.0_f32;

        let reasoning_engine = self.reasoning_engine.lock().unwrap();

        for option in &parsed_mcq.options {
            // --- Heuristique de bon sens : l'option est-elle dans la question ? ---
            // Extrait le texte pur de l'option (ex: "A. Blanc" -> "Blanc")
            let option_text = option.split_once('.').map_or(option.as_str(), |(_, text)| text).trim();
            
            // Normalise le texte pour la comparaison
            let normalized_question = parsed_mcq.question.to_lowercase();
            let normalized_option = option_text.to_lowercase();

            if normalized_question.contains(&normalized_option) {
                println!("[MCQ Solver] Heuristique de bon sens déclenchée pour l'option : {}", option);
                max_score = 1.0; // Score de confiance maximal
                best_option = Some(option.clone());
                break; // On a trouvé la réponse la plus logique, pas besoin de chercher plus loin.
            }

            // Formulate a complete assertion to be evaluated.
            let assertion = format!("{} {}", parsed_mcq.question, option);
            
            let score = reasoning_engine.score_assertion(&assertion, hippocampus, encoder);
            println!("[MCQ Solver] Evaluating: '{}' -> Score: {:.4}", assertion, score);

            if score > max_score {
                max_score = score;
                best_option = Some(option.clone());
            }
        }

        // If we found a plausible answer, return it as a memory.
        if let Some(chosen_option) = best_option {
            if max_score > 0.1 { // Confidence threshold
                let answer_content = format!("En réponse à la question '{}', l'option la plus plausible est : {}", parsed_mcq.question, chosen_option);
                
                let answer_trace = encoder.read().unwrap().encode(&answer_content);

                let answer_memory = HolographicMemory {
                    text: answer_content,
                    trace: answer_trace,
                    is_axiom: false,
                };
                return Some(answer_memory);
            }
        }

        None // No confident answer found
    }

    /// Parses a prompt to extract the question and a list of options.
    fn parse_mcq(&self, prompt: &str) -> Option<ParsedMCQ> {
        // Heuristique pour trouver le début des options (ex: " A. ", " 1) ")
        let options_marker = Regex::new(r"\s+[A-Da-d][.)]").unwrap();
        
        let (question, options_str) = match options_marker.find(prompt) {
            Some(marker_match) => prompt.split_at(marker_match.start()),
            None => return None, // Not an MCQ if no option markers are found.
        };

        let question = question.trim().to_string();
        let options: Vec<String> = options_marker.split(options_str)
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect();

        Some(ParsedMCQ { question, options })
    }
}
