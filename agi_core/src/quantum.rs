// agi_core/src/quantum.rs


use nalgebra::Complex;
use rand::Rng;
use std::f32::consts::FRAC_1_SQRT_2;

// --- Qubit Definition ---

/// Represents a qubit with complex amplitudes for |0> and |1> states.
#[derive(Debug, Clone)]
pub struct Qubit {
    pub alpha: Complex<f32>, // Amplitude for |0>
    pub beta: Complex<f32>,  // Amplitude for |1>
}

impl Qubit {
    /// Initializes a new qubit in the |0> state.
    pub fn new() -> Self {
        Self {
            alpha: Complex::new(1.0, 0.0),
            beta: Complex::new(0.0, 0.0),
        }
    }

    /// Measures the qubit, collapsing it to either |0> or |1>.
    /// Returns the classical outcome (0 or 1).
    pub fn measure(&mut self) -> u8 {
        // Probabilities are the squared magnitudes of the amplitudes.
        let prob_0 = self.alpha.norm_sqr();
        let prob_1 = self.beta.norm_sqr();
        let total_prob = prob_0 + prob_1;

        // Handle the case of a zero-norm state to avoid division by zero.
        if total_prob < 1e-9 {
            // Default to collapsing to |0> if the state is invalid/zero.
            self.alpha = Complex::new(1.0, 0.0);
            self.beta = Complex::new(0.0, 0.0);
            return 0;
        }

        let rand_val: f32 = rand::thread_rng().gen();
        if rand_val < prob_0 / total_prob {
            self.alpha = Complex::new(1.0, 0.0); // Collapse to |0>
            self.beta = Complex::new(0.0, 0.0);
            0
        } else {
            self.alpha = Complex::new(0.0, 0.0);
            self.beta = Complex::new(1.0, 0.0); // Collapse to |1>
            1
        }
    }

    /// Normalizes the qubit's amplitudes to ensure the total probability is 1.
    pub fn normalize(&mut self) {
        let norm = (self.alpha.norm_sqr() + self.beta.norm_sqr()).sqrt();
        if norm > 1e-9 {
            self.alpha /= norm;
            self.beta /= norm;
        } else {
            // Reset to a default |0> state if magnitude is zero.
            self.alpha = Complex::new(1.0, 0.0);
            self.beta = Complex::new(0.0, 0.0);
        }
    }
}

// --- Gate Traits ---

/// Represents a quantum gate that can be applied to a single qubit.
pub trait OneQubitGate {
    fn apply(&self, qubit: &mut Qubit);
}

/// A trait for gates that act on two qubits.
pub trait TwoQubitGate {
    fn apply(&self, control: &mut Qubit, target: &mut Qubit);
}

// --- One-Qubit Gates ---

/// The Hadamard gate, which creates a superposition.
pub struct HadamardGate;
impl OneQubitGate for HadamardGate {
    fn apply(&self, qubit: &mut Qubit) {
        let original_alpha = qubit.alpha;
        let original_beta = qubit.beta;

        qubit.alpha = (original_alpha + original_beta) * FRAC_1_SQRT_2;
        qubit.beta = (original_alpha - original_beta) * FRAC_1_SQRT_2;
    }
}

/// The Pauli-X gate, equivalent to a quantum NOT gate.
pub struct PauliXGate;
impl OneQubitGate for PauliXGate {
    fn apply(&self, qubit: &mut Qubit) {
        std::mem::swap(&mut qubit.alpha, &mut qubit.beta);
    }
}

/// A gate that applies a phase shift to the |1> state.
pub struct PhaseShiftGate {
    phase: f32,
}

impl PhaseShiftGate {
    pub fn new(phase: f32) -> Self {
        Self { phase }
    }
}

impl OneQubitGate for PhaseShiftGate {
    fn apply(&self, qubit: &mut Qubit) {
        // The phase factor is e^(i*phase) = cos(phase) + i*sin(phase)
        let phase_factor = Complex::new(self.phase.cos(), self.phase.sin());
        // Apply the phase shift only to the beta component (the |1> state)
        qubit.beta *= phase_factor;
    }
}

// --- Two-Qubit Gates ---

/// The CNOT (Controlled-NOT) gate.
pub struct CnotGate;
impl TwoQubitGate for CnotGate {
    fn apply(&self, control: &mut Qubit, target: &mut Qubit) {
        // Apply Pauli-X to target if control is in |1> state.
        // A simple way to check this is if the probability of |1> is high.
        if control.beta.norm_sqr() > 0.5 {
            let pauli_x = PauliXGate;
            pauli_x.apply(target);
        }
    }
}

/// Represents the Entanglement Gate (creates a Bell state).
pub struct EntanglementGate;
impl TwoQubitGate for EntanglementGate {
    fn apply(&self, control: &mut Qubit, target: &mut Qubit) {
        let hadamard = HadamardGate;
        hadamard.apply(control);
        let cnot = CnotGate;
        cnot.apply(control, target);
    }
}

// --- Holographic Functions ---


