//! # Neurochemical Modulator
//! 
//! Ce module simule les effets fonctionnels de haut niveau des principaux neuromodulateurs
//! sur le comportement cognitif de l'AGI. Il ne simule pas la chimie elle-même,
//! mais plutôt ses conséquences sur des paramètres comme la motivation, l'attention,
//! la patience et la vigilance.

/// Représente l'état chimique global du "cerveau" de l'AGI.
/// Chaque valeur est typiquement normalisée entre 0.0 et 1.0.
#[derive(Debug, Clone)]
pub struct NeurochemicalState {
    /// **Dopamine** : Associée à la motivation, la récompense, l'apprentissage par renforcement
    /// et la flexibilité cognitive. Un niveau élevé peut encourager l'exploration et la
    /// répétition de stratégies gagnantes.
    pub dopamine: f32,

    /// **Sérotonine** : Associée à la régulation de l'humeur, la patience et le contrôle
    /// des impulsions. Un niveau élevé peut rendre l'AGI plus "tolérante" dans ses
    /// recherches sémantiques ou moins prompte à changer de sujet.
    pub serotonin: f32,

    /// **Acétylcholine** : Associée à l'attention, la concentration et la précision de la mémoire.
    /// Un niveau élevé peut affiner la recherche de souvenirs pour être plus stricte et
    /// pertinente, améliorant le "focus".
    pub acetylcholine: f32,

    /// **Noradrénaline** : Associée à la vigilance, l'alerte et la réponse à la nouveauté
    /// ou au stress. Un niveau élevé peut augmenter la réactivité globale du système
    /// neuronal.
    pub noradrenaline: f32,
}

/// Le modulateur lui-même, qui contient l'état et les méthodes pour le mettre à jour.
#[derive(Debug, Clone)]
pub struct NeurochemicalModulator {
    pub state: NeurochemicalState,
}

impl NeurochemicalModulator {
    /// Crée un nouvel modulateur avec un état de base équilibré.
    pub fn new() -> Self {
        Self {
            state: NeurochemicalState {
                dopamine: 0.5,
                serotonin: 0.5,
                acetylcholine: 0.5,
                noradrenaline: 0.5,
            },
        }
    }

    /// Augmente le niveau de dopamine suite à un succès cognitif.
    /// Cela simule une boucle de renforcement positif.
    pub fn reward_successful_reasoning(&mut self) {
        const DOPAMINE_REWARD: f32 = 0.05;
        self.state.dopamine = (self.state.dopamine + DOPAMINE_REWARD).min(1.0);
        println!("--- Neuro-Modulation: Dopamine rewarded. New level: {:.2} ---", self.state.dopamine);
    }


    /// Calcule un seuil de distance pour le raisonnement qui est modulé par la dopamine.
    /// Un niveau de dopamine plus élevé augmente légèrement le seuil, ce qui rend l'AGI plus "ouverte"
    /// à considérer des souvenirs sémantiquement plus distants (flexibilité cognitive).
    /// 
    /// # Arguments
    /// * `base_threshold` - Le seuil de distance de base avant modulation.
    pub fn get_reasoning_distance_threshold(&self, base_threshold: f32) -> f32 {
        // La modulation est centrée autour de 0.5 (état de base).
        // L'influence de la dopamine est un facteur (par exemple, 20% du seuil de base).
        let modulation_factor = (self.state.dopamine - 0.5) * (base_threshold * 0.2);
        let dynamic_threshold = base_threshold + modulation_factor;
        // S'assure que le seuil ne devient pas négatif ou absurdement élevé.
        dynamic_threshold.max(0.1).min(1.5)
    }

    /// Simule la dégradation naturelle ou la recapture des neuromodulateurs,
    /// les faisant revenir lentement à leur état de base (0.5).
    pub fn decay(&mut self) {
        const DECAY_RATE: f32 = 0.005; // Taux de dégradation très lent
        
        // Ramène la dopamine vers 0.5
        if self.state.dopamine > 0.5 {
            self.state.dopamine = (self.state.dopamine - DECAY_RATE).max(0.5);
        } else {
            self.state.dopamine = (self.state.dopamine + DECAY_RATE).min(0.5);
        }

        // TODO: Appliquer la même logique pour les autres neuromodulateurs quand ils seront utilisés.
    }

}
