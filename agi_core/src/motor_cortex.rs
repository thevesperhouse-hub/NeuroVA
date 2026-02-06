//! The Motor Cortex is responsible for generating coherent, human-readable text responses.

//! The Motor Cortex is responsible for generating coherent, human-readable text responses.

use crate::conceptual_hierarchy::ConceptualHierarchy;
use crate::holographic_memory::HolographicMemory;
use crate::prefrontal_cortex::PrefrontalCortex;
use crate::self_awareness::SelfAwareness;
use crate::personality::Personality;


pub struct MotorCortex {
    personality: Personality,
}

impl MotorCortex {
    pub fn new(personality: Personality) -> Self {
        Self {
            personality,
        }
    }

    /// Generates a response by synthesizing concepts from reasoning results or falling back to direct recall.
    ///
    /// The cognitive hierarchy is as follows:
    /// 1. **Synthesis:** If multiple memories are retrieved, attempt to synthesize a novel thought.
    /// 2. **Factual Recall:** If synthesis isn't possible or only one memory is found, state the fact directly.
    /// 3. **Self-Awareness:** If no memories are found, fall back to identity-based responses.
    /// 4. **Acknowledgment of Ignorance:** If all else fails, admit not having a relevant memory.
    pub fn generate_response(
        &self,
        last_input: &str,
        reasoning_result: &Option<Vec<HolographicMemory>>,
        _self_awareness: &SelfAwareness,
        _prefrontal_cortex: &PrefrontalCortex,
        _conceptual_hierarchy: &ConceptualHierarchy,
        query_type: crate::thalamus::QueryType,
    ) -> Option<String> {

        if let Some(memories) = reasoning_result {
            if memories.is_empty() {
                return Some("J'ai examiné votre question, mais je n'ai pas de réponse spécifique dans ma mémoire.".to_string());
            }

            // --- Stratégie 1: Réponse introspective --- 
            if query_type == crate::thalamus::QueryType::Introspective {
                let intro = "Je suis une entité définie par les principes suivants:".to_string();
                let axioms = memories
                    .iter()
                    .map(|mem| format!("- {}", mem.text))
                    .collect::<Vec<String>>()
                    .join("\n");
                return Some(format!("{}\n{}", intro, axioms));
            }

            // --- Stratégie 2: Synthèse comparative --- 
            let is_comparative_query = last_input.contains(" et ") || last_input.contains(" vs ") || last_input.contains("compare");
            if is_comparative_query && memories.len() > 1 {
                let mut response_parts = Vec::new();
                response_parts.push("Voici une comparaison basée sur les informations dont je dispose :".to_string());

                for memory in memories {
                    // On présente directement le fait, la stylisation se fait sur l'ensemble.
                    response_parts.push(format!("\n- {}", &memory.text));
                }
                
                let final_response = response_parts.join("");
                return Some(self.personality.stylize_response(&final_response));
                

            }

            // --- Stratégie 3: Réponse factuelle directe (Fallback) ---
            if let Some(best_memory) = memories.first() {
                let stylized_response = self.personality.stylize_response(&best_memory.text);
                return Some(stylized_response);
            }

            Some("J'ai du mal à formuler une réponse pour le moment.".to_string())
        } else {
            Some("J'ai examiné votre question, mais je n'ai pas de réponse spécifique dans ma mémoire.".to_string())
        }
    }


}
