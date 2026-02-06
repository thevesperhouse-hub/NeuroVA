

// agi_core/src/thalamus.rs
use crate::holographic_memory::{HolographicEncoder, HolographicTrace};
use std::sync::{Arc, RwLock};

/// Represents the classified intent of a user's prompt.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum QueryType {
    Introspective, // "Who are you?", "What can you do?"
    Factual,         // "What is...?", "Who was...?"
    Creative,        // "Write a poem...", "Imagine..."
    Social,          // "How are you?", "Hello."
    Ambiguous,       // Could not determine a clear intent.
}

/// Represents the Thalamus, a key structure for gating and relaying information
/// using semantic, holographic principles.
pub struct Thalamus {
    encoder: Arc<RwLock<HolographicEncoder>>,
    introspective_prototype: HolographicTrace,
    factual_prototype: HolographicTrace,
    creative_prototype: HolographicTrace,
    social_prototype: HolographicTrace,
}

// Manual implementation of Debug as HolographicTrace does not derive it.
impl std::fmt::Debug for Thalamus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Thalamus")
            .field("encoder", &"Arc<RwLock<HolographicEncoder>>")
            .field("introspective_prototype", &"HolographicTrace")
            .field("factual_prototype", &"HolographicTrace")
            .field("creative_prototype", &"HolographicTrace")
            .field("social_prototype", &"HolographicTrace")
            .finish()
    }
}

impl Thalamus {
    /// Creates a new Thalamus, pre-computing prototype traces for semantic query classification.
    pub fn new(encoder: Arc<RwLock<HolographicEncoder>>) -> Self {
        let encoder_lock = encoder.read().unwrap();

        // Define core concepts for each query type.
        // Use rich, representative phrases to create more nuanced holographic prototypes.
                let introspective_concepts = "Who are you? Tell me about yourself. What is your purpose? Describe your nature. What are your capabilities? What are you made of?";
        // Use a more robust "bag-of-words" prototype for factual queries.
                let factual_concepts = "what is who is where is when is why is how is what was who was tell me about explain define describe the history of the process of the meaning of facts information data E=mc2 speed of light socrates quoi qui où quand comment pourquoi est était étaient sont fait expliquer définir décrire dis-moi sur le fondateur l'histoire le processus la signification de les faits les informations";
        let creative_concepts = "Imagine a world where... Create a story about... Write a poem that captures the feeling of... Compose a song about... What if...? Invent a concept.";
        let social_concepts = "how are you comment vas-tu how's it going what's up hello hi hey salut bonjour good morning good afternoon good evening greetings farewell bye goodbye thank you thanks joke";

        // Create holographic prototypes.
        let introspective_prototype = encoder_lock.encode_raw(introspective_concepts);
        let factual_prototype = encoder_lock.encode_raw(factual_concepts);
        let creative_prototype = encoder_lock.encode_raw(creative_concepts);
        let social_prototype = encoder_lock.encode_raw(social_concepts);

        Self {
            encoder: Arc::clone(&encoder),
            introspective_prototype,
            factual_prototype,
            creative_prototype,
            social_prototype,
        }
    }

    /// Re-generates the holographic prototypes using the current state of the encoder.
    /// This should be called after the main knowledge base is loaded to ensure prototypes
    /// are created in a mature semantic space.
    pub fn rebuild_prototypes(&mut self) {
        let encoder_lock = self.encoder.read().unwrap();
        println!("--- Rebuilding Thalamus semantic prototypes... ---");

                let introspective_concepts = "Who are you? Tell me about yourself. What is your purpose? Describe your nature. What are your capabilities? What are you made of?";
                let factual_concepts = "what is who is where is when is why is how is what was who was tell me about explain define describe the history of the process of the meaning of facts information data E=mc2 speed of light socrates quoi qui où quand comment pourquoi est était étaient sont fait expliquer définir décrire dis-moi sur le fondateur l'histoire le processus la signification de les faits les informations";
        let creative_concepts = "Imagine a world where... Create a story about... Write a poem that captures the feeling of... Compose a song about... What if...? Invent a concept.";
        let social_concepts = "how are you comment vas-tu how's it going what's up hello hi hey salut bonjour good morning good afternoon good evening greetings farewell bye goodbye thank you thanks joke";

        self.introspective_prototype = encoder_lock.encode_raw(introspective_concepts);
        self.factual_prototype = encoder_lock.encode_raw(factual_concepts);
        self.creative_prototype = encoder_lock.encode_raw(creative_concepts);
        self.social_prototype = encoder_lock.encode_raw(social_concepts);
        println!("--- Thalamus prototypes rebuilt successfully. ---");
    }

    /// Checks if the text matches common factual question patterns.
    fn is_factual_question(&self, text: &str) -> bool {
        let lower_text = text.to_lowercase();
        let factual_starters = [
            // English
            "what is", "what are", "what's", "what was", "what were",
            "who is", "who are", "who's", "who was", "who were",
            "where is", "where are", "where was", "where were",
            "when is", "when are", "when was", "when were",
            "why is", "why are", "why was", "why were",
            "how is", "how are", "how was", "how were",
            "explain", "define", "describe", "tell me about",

            // French
            "qu'est-ce que", "qu'est ce que", "c'est quoi", "qu'est-ce qu'est", "qu'est que c'est",
            "qui est", "qui était", "qui sont", "qui étaient",
            "où est", "où était", "où sont", "où étaient",
            "quand est-ce que", "quand était",
            "pourquoi est-ce que",
            "comment est",
            "explique", "définis", "décris", "parle-moi de"
        ];

        factual_starters.iter().any(|&starter| lower_text.starts_with(starter))
    }

    /// Analyzes the prompt to determine its nature (e.g., Factual, Introspective).
    pub fn analyze_prompt(&self, prompt: &str) -> QueryType {
        // --- Priority 1: Keyword-based classification for deterministic routing ---
                const IDENTITY_KEYWORDS: &[&str] = &["who are you", "what are you", "qui es-tu", "quel est ton nom", "who is neurova"];
        const INTROSPECTIVE_KEYWORDS: &[&str] = &["do you feel", "what do you think", "penses-tu", "ressens-tu"];
        const SOCIAL_KEYWORDS: &[&str] = &["hello", "how are you", "bonjour", "salut"];

        let lower_prompt = prompt.to_lowercase();

        if IDENTITY_KEYWORDS.iter().any(|&keyword| lower_prompt.contains(keyword)) {
            return QueryType::Introspective; // Crucially, identity questions are introspective.
        }
        if INTROSPECTIVE_KEYWORDS.iter().any(|&keyword| lower_prompt.contains(keyword)) {
            return QueryType::Introspective;
        }
        // Use the more robust starter check for factual questions.
        if self.is_factual_question(prompt) {
            return QueryType::Factual;
        }
        if SOCIAL_KEYWORDS.iter().any(|&keyword| lower_prompt.contains(keyword)) {
            return QueryType::Social;
        }

        // --- Priority 2: Fallback to semantic similarity analysis if no keywords match ---
        let prompt_trace = self.encoder.read().unwrap().encode_raw(prompt);

        let prototypes = [
            (QueryType::Introspective, &self.introspective_prototype),
            (QueryType::Factual, &self.factual_prototype),
            (QueryType::Creative, &self.creative_prototype),
            (QueryType::Social, &self.social_prototype),
        ];

        // Find the prototype with the highest cosine similarity.
        let scores: Vec<_> = prototypes
            .iter()
            .map(|(q_type, proto_trace)| {
                let similarity = prompt_trace.cosine_similarity(proto_trace);
                (q_type, similarity)
            })
            .collect();

        // For debugging, print the scores.
        println!("Thalamus Scores for '{}':", prompt);
        for (q_type, score) in &scores {
            println!("  - {:?}: {:.4}", q_type, score);
        }

        // If a score is significantly higher than others, choose it. Otherwise, ambiguous.
        const MINIMAL_CONFIDENCE_THRESHOLD: f32 = 0.05;
        let best_match = scores
            .iter()
            .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));

        match best_match {
            Some((query_type, similarity)) if *similarity > MINIMAL_CONFIDENCE_THRESHOLD => **query_type,
            _ => QueryType::Ambiguous,
        }
    }
}
