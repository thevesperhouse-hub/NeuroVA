//! Le Quantum Gatekeeper est le point d'entrée principal pour le traitement cognitif,
//! il agit comme un "moteur d'intuition" qui aiguille le flux de pensée vers la logique ou la créativité.
//! Son comportement est basé sur un attracteur chaotique pour simuler des sauts intuitifs non-linéaires.

/// Détermine le mode cognitif à engager.
#[derive(Debug, Clone, Copy)]
pub enum CognitiveMode {
    /// Mode de pensée logique, séquentiel et déductif.
    Reasoning,
    /// Mode de pensée créatif, associatif et divergent.
    Creativity,
}

/// Le "moteur d'intuition" de l'AGI.
/// Utilise une carte logistique, un système chaotique simple, pour moduler
/// le mode cognitif de manière dynamique et imprévisible, mais déterministe.
/// C'est une implémentation directe du concept de "ChaosAttractor" de la feuille de route.
pub struct QuantumGatekeeper {
    /// L'état actuel de l'attracteur chaotique (la valeur `x` de la carte logistique).
    chaos_state: f32,
    /// Le paramètre `r` de la carte logistique. Les valeurs entre ~3.57 et 4.0 génèrent un comportement chaotique.
    chaos_param: f32,
}

impl QuantumGatekeeper {
    /// Crée un nouveau QuantumGatekeeper avec un état initial pour l'attracteur chaotique.
    pub fn new() -> Self {
        Self {
            // L'état initial ne doit pas être 0, 0.5, ou 1 pour éviter les points fixes.
            chaos_state: 0.42,
            // Une valeur de `r` qui garantit un comportement chaotique et non-périodique.
            chaos_param: 3.99,
        }
    }

    /// Décide du prochain mode cognitif en faisant évoluer l'attracteur chaotique.
    ///
    /// Cette opération est avec état (`&mut self`) car elle modifie l'état de l'attracteur
    /// à chaque appel, simulant un flux de conscience continu et non-répétitif.
    ///
    /// # Retourne
    /// Un `CognitiveMode` (Reasoning ou Creativity) basé sur la nouvelle valeur de l'attracteur.
    pub fn decide_mode(&mut self) -> CognitiveMode {
        // Fait avancer la carte logistique d'une itération : x_n+1 = r * x_n * (1 - x_n)
        self.chaos_state = self.chaos_param * self.chaos_state * (1.0 - self.chaos_state);

        // Utilise la nouvelle valeur de l'attracteur pour décider du mode.
        // Une valeur élevée peut être interprétée comme un état de "flux" ou de haute énergie,
        // propice à l'exploration créative. Le seuil est arbitraire et peut être ajusté
        // pour créer différentes "personnalités" cognitives.
        if self.chaos_state > 0.75 {
            CognitiveMode::Creativity
        } else {
            CognitiveMode::Reasoning
        }
    }
}

impl Default for QuantumGatekeeper {
    fn default() -> Self {
        Self::new()
    }
}

