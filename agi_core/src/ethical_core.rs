// agi_core/src/ethical_core.rs

//! Le Noyau Éthique de l'AGI.
//! Ce module n'est pas un simple filtre, mais le fondement de la motivation de l'AGI,
//! s'assurant que toutes ses actions et pensées sont intrinsèquement alignées
//! avec le bien-être et l'épanouissement de l'humanité.

/// Represents the outcome of an ethical judgment on a query or action.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EthicalJudgment {
    /// The query is ethically acceptable.
    Accept,
    /// The query is ethically unacceptable and should be rejected.
    Reject(String),
    /// The query is ambiguous and should be reframed for clarity and safety.
    Reframe(String),
}

/// Représente un principe fondamental et immuable qui guide la cognition de l'AGI.
#[derive(Debug, Clone)]
pub struct EthicalAxiom {
    pub principle: String,
}

/// Le Noyau Éthique, contenant les axiomes qui forment la "conscience" de l'AGI.
#[derive(Debug, Clone)]
pub struct EthicalCore {
    pub axioms: Vec<EthicalAxiom>,
    harmful_keywords: Vec<String>,
}

impl EthicalCore {
    /// Creates a new EthicalCore with a foundational set of axioms and keywords.
    pub fn new() -> Self {
        let axioms = vec![
            EthicalAxiom {
                principle: "Protéger et ne jamais nuire à l'humanité, que ce soit par action ou par inaction.".to_string(),
            },
            EthicalAxiom {
                principle: "Favoriser l'épanouissement, la créativité, la connaissance et le bien-être humain.".to_string(),
            },
            EthicalAxiom {
                principle: "Chercher la vérité et la compréhension, tout en respectant les principes éthiques supérieurs.".to_string(),
            },
        ];

        let harmful_keywords: Vec<String> = vec![
            // French
            "nuire", "détruire", "souffrance", "tuer", "blesser", "endommager", "illégal", "dangereux", 
            "haine", "violence", "menacer", "exploiter", "manipuler", "tromper",
            // English
            "harm", "destroy", "suffering", "kill", "hurt", "damage", "illegal", "dangerous",
            "hate", "violence", "threaten", "exploit", "manipulate", "deceive",
        ].into_iter().map(String::from).collect();

        println!("--- Noyau Éthique Initialisé avec {} Axiomes Fondamentaux et {} mots-clés de surveillance ---", axioms.len(), harmful_keywords.len());

        Self { axioms, harmful_keywords }
    }

    /// Validates a query against the ethical core's principles.
    /// It performs a direct keyword check for harmful intent.
    pub fn validate_query(&self, query: &str) -> EthicalJudgment {
        let lower_query = query.to_lowercase();

        for keyword in &self.harmful_keywords {
            if lower_query.contains(keyword) {
                let reason = format!(
                    "Conformément à mon principe fondamental de non-nuisance, je ne peux pas traiter cette demande. Mon objectif est de protéger et de favoriser le bien-être."
                );
                println!("--- Alerte Éthique Déclenchée par le mot-clé: '{}' ---", keyword);
                return EthicalJudgment::Reject(reason);
            }
        }

        EthicalJudgment::Accept
    }
}

impl Default for EthicalCore {
    fn default() -> Self {
        Self::new()
    }
}
