//! La Creativity Forge est responsable de la pensée divergente, de l'intuition
//! et de la génération d'états quantiques nouveaux et inattendus.

use crate::quantum::{EntanglementGate, Qubit, TwoQubitGate, OneQubitGate};
use rand::{thread_rng, Rng};
use std::f32::consts::PI;

/// La Creativity Forge, qui explore de nouvelles voies cognitives.
pub struct CreativityForge {
    // Futurs champs : attracteurs étranges, paramètres de chaos, etc.
}

impl CreativityForge {
    pub fn new() -> Self {
        Self {}
    }

    /// Modifie l'état quantique pour encourager l'émergence de nouveaux motifs.
    /// C'est ici que la "pensée latérale" et les "sauts conceptuels" se produisent.
    pub fn process(&self, quantum_core: &mut [Qubit]) {
        println!("\n--- Creativity Forge Activated ---");
        let mut rng = thread_rng();
        let core_len = quantum_core.len();

        if core_len < 2 {
            println!("Not enough qubits for creative entanglement.");
            return;
        }

        // --- 1. Sauts Conceptuels via Intrication (Quantum Leaps) ---
        // On intrique un petit nombre de paires de qubits pour créer des liens
        // nouveaux et inattendus entre des concepts non-reliés.
        let entanglement_gate = EntanglementGate;
        // Intriquer ~5% du core, avec un minimum de 1.
        let num_entanglements = (core_len / 20).max(1); 
        println!("Attempting {} quantum leaps...", num_entanglements);

        for _ in 0..num_entanglements {
            // Choisir deux indices uniques
            let idx1 = rng.gen_range(0..core_len);
            let mut idx2 = rng.gen_range(0..core_len);
            while idx1 == idx2 {
                idx2 = rng.gen_range(0..core_len);
            }

            // Emprunter deux éléments mutables du slice de manière sûre
            let (lo_idx, hi_idx) = if idx1 < idx2 { (idx1, idx2) } else { (idx2, idx1) };
            let (slice1, slice2) = quantum_core.split_at_mut(hi_idx);
            let q1 = &mut slice1[lo_idx];
            let q2 = &mut slice2[0];

            println!("  -> Entangling Qubit {} and Qubit {}", lo_idx, hi_idx);
            // q1 est le contrôle, q2 est la cible
            entanglement_gate.apply(q1, q2);
        }

        // --- 2. Bruit d'Intuition (Background Creativity) ---
        // Une légère fluctuation aléatoire pour tous les qubits, simulant
        // une "intuition" de fond ou des idées spontanées mineures.
        println!("Applying background intuition noise...");
        for qubit in quantum_core.iter_mut() {
            // C'est l'équivalent d'une "intuition" ou d'une "idée spontanée".
            // Introduce a random phase shift for creative exploration.
            let random_phase_shift: f32 = rng.gen_range(-PI..PI);
            let phase_gate = crate::quantum::PhaseShiftGate::new(random_phase_shift);
            phase_gate.apply(qubit);
        }
        println!("--- Creativity Forge Process Complete ---\n");
    }
}

impl Default for CreativityForge {
    fn default() -> Self {
        Self::new()
    }
}

