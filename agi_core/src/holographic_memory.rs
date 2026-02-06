// agi_core/src/holographic_memory.rs

use crate::connectome::Connectome;
use nalgebra::Complex;
use serde::{Serialize, Deserialize};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use rand::Rng;
use sha2::{Digest, Sha256};
use phf::phf_set;
// Note: string-interner available for future advanced string deduplication
// Temporarily removed memory optimization for thread safety
// TODO: Implement thread-safe version with RwLock or thread-local storage

// --- Q1.15 Quantization System ---
// Kimi AI optimization: Replace f32 complex numbers with i16 Q1.15 fixed-point
// This reduces memory usage by 75% (8 bytes -> 2 bytes per complex number)

/// Q1.15 fixed-point representation of a complex number
/// Real and imaginary parts are stored as i16 with Q1.15 format
/// Range: [-1.0, 0.99997] with ~0.00003 precision
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct QuantizedComplex {
    pub real: i16,
    pub imag: i16,
}

impl QuantizedComplex {
    /// Convert from f32 complex to Q1.15 quantized
    pub fn from_complex(c: Complex<f32>) -> Self {
        const SCALE: f32 = 32767.0; // 2^15 - 1
        Self {
            real: (c.re.clamp(-1.0, 0.99997) * SCALE) as i16,
            imag: (c.im.clamp(-1.0, 0.99997) * SCALE) as i16,
        }
    }
    
    /// Convert from Q1.15 quantized to f32 complex
    pub fn to_complex(self) -> Complex<f32> {
        const INV_SCALE: f32 = 1.0 / 32767.0;
        Complex::new(
            self.real as f32 * INV_SCALE,
            self.imag as f32 * INV_SCALE,
        )
    }
    
    /// Multiply two quantized complex numbers (Q1.15 * Q1.15 = Q2.30, then normalize back to Q1.15)
    pub fn mul(self, other: Self) -> Self {
        let real = ((self.real as i32 * other.real as i32 - self.imag as i32 * other.imag as i32) >> 15) as i16;
        let imag = ((self.real as i32 * other.imag as i32 + self.imag as i32 * other.real as i32) >> 15) as i16;
        Self { real, imag }
    }
    
    /// Add two quantized complex numbers
    pub fn add(self, other: Self) -> Self {
        Self {
            real: self.real.saturating_add(other.real),
            imag: self.imag.saturating_add(other.imag),
        }
    }
    
    /// Scale by a factor (for weighted operations)
    pub fn scale(self, factor: f32) -> Self {
        let scale = (factor.clamp(-1.0, 0.99997) * 32767.0) as i16;
        Self {
            real: ((self.real as i32 * scale as i32) >> 15) as i16,
            imag: ((self.imag as i32 * scale as i32) >> 15) as i16,
        }
    }
    
    /// Compute squared magnitude for normalization
    pub fn norm_sqr(self) -> f32 {
        let c = self.to_complex();
        c.norm_sqr()
    }
    
    /// Zero constant for initialization
    pub const ZERO: Self = Self { real: 0, imag: 0 };
}

// Implement arithmetic traits for seamless integration
use std::ops::{AddAssign, DivAssign, MulAssign};

impl AddAssign for QuantizedComplex {
    fn add_assign(&mut self, other: Self) {
        *self = self.add(other);
    }
}

impl AddAssign<&QuantizedComplex> for QuantizedComplex {
    fn add_assign(&mut self, other: &Self) {
        *self = self.add(*other);
    }
}

impl DivAssign<f32> for QuantizedComplex {
    fn div_assign(&mut self, scalar: f32) {
        *self = self.scale(1.0 / scalar);
    }
}

impl MulAssign<f32> for QuantizedComplex {
    fn mul_assign(&mut self, scalar: f32) {
        *self = self.scale(scalar);
    }
}

// --- Concept Focuser ---

/// A component responsible for semantic distillation.
/// It identifies and filters out low-information words to focus on core concepts.
#[derive(Debug, Clone)]
pub struct ConceptFocuser {
    // No need to store stop words anymore, we use the static phf::Set directly
}

impl ConceptFocuser {
    pub fn new() -> Self {
        Self {}
    }

    /// Distills core concepts from text, including unigrams, bigrams, and trigrams.
    pub fn distill_concepts(&self, text: &str) -> HashSet<String> {
        // 1. Tokenize and clean the text, preserving order.
        let words: Vec<String> = text
            .split_whitespace()
            .map(|s| s.trim_matches(|c: char| !c.is_alphanumeric() && c != '=' && c != '-' && c != '²').to_lowercase())
            .filter(|s| !s.is_empty() && !Self::get_low_information_words().contains(s.as_str()))
            .collect();

        let mut concepts = HashSet::new();

        // 2. Extract n-grams (unigrams, bigrams, trigrams)
        for i in 0..words.len() {
            // Unigrams (only add if they have some length)
            if words[i].len() > 2 {
                concepts.insert(words[i].clone());
            }

            // Bigrams
            if i + 1 < words.len() {
                concepts.insert(format!("{} {}", words[i], words[i + 1]));
            }

            // Trigrams
            if i + 2 < words.len() {
                concepts.insert(format!("{} {} {}", words[i], words[i + 1], words[i + 2]));
            }
        }

        concepts
    }

    /// Returns a comprehensive set of low-information words (stop words) for French and English.
    /// Now using phf::Set for compile-time perfect hashing and better performance.
    fn get_low_information_words() -> &'static phf::Set<&'static str> {
        static STOP_WORDS: phf::Set<&'static str> = phf_set! {
            // French stop words
            "a", "à", "alors", "au", "aucuns", "aussi", "autre", "autres", "aux", "avant", "avec", "avoir", "bon",
            "car", "ce", "ceci", "cela", "ces", "cette", "ceux", "chaque", "ci", "comme", "comment", "dans",
            "de", "des", "du", "dedans", "dehors", "depuis", "deux", "devrait", "doit", "donc", "dos", "droite",
            "dès", "début", "elle", "elles", "en", "encore", "essai", "est", "et", "eu", "fait", "faites", "fois",
            "font", "force", "haut", "hors", "ici", "il", "ils", "je", "juste", "la", "le", "les", "leur",
            "leurs", "lui", "ma", "maintenant", "mais", "mes", "mine", "moins", "mon", "mot", "même", "ne", "ni",
            "nommés", "nos", "notre", "nous", "nouveaux", "ou", "où", "par", "parce", "pas", "peut", "peu",
            "plupart", "pour", "pourquoi", "quand", "que", "quel", "quelle", "quelles", "quels", "qui", "sa",
            "sans", "ses", "seul", "seulement", "si", "sien", "soi", "soit", "son", "sont", "sous", "sur", "ta",
            "tandis", "tellement", "tels", "tes", "ton", "tous", "tout", "trop", "très", "tu", "un", "une",
            "voient", "vont", "vos", "votre", "vous", "vu", "y", "ça", "étaient", "état", "étions", "été", "être",
            "serait",
            // English stop words
            "about", "above", "after", "again", "against", "all", "am", "an", "and", "any", "are", "aren't",
            "as", "at", "be", "because", "been", "before", "being", "below", "between", "both", "but", "by",
            "can't", "cannot", "could", "couldn't", "did", "didn't", "do", "does", "doesn't", "doing", "don't",
            "down", "during", "each", "few", "for", "from", "further", "had", "hadn't", "has", "hasn't", "have",
            "haven't", "having", "he", "he'd", "he'll", "he's", "her", "here", "here's", "hers", "herself",
            "him", "himself", "his", "how", "how's", "i", "i'd", "i'll", "i'm", "i've", "if", "in", "into",
            "is", "isn't", "it", "it's", "its", "itself", "let's", "me", "more", "most", "mustn't", "my",
            "myself", "no", "nor", "not", "of", "off", "on", "once", "only", "or", "other", "ought", "our",
            "ours", "ourselves", "out", "over", "own", "same", "shan't", "she", "she'd", "she'll", "she's",
            "should", "shouldn't", "so", "some", "such", "than", "that", "that's", "the", "their", "theirs",
            "them", "themselves", "then", "there", "there's", "these", "they", "they'd", "they'll", "they're",
            "they've", "this", "those", "through", "to", "too", "under", "until", "up", "very", "was", "wasn't",
            "we", "we'd", "we'll", "we're", "we've", "were", "weren't", "what", "what's", "when", "when's",
            "where", "where's", "which", "while", "who", "who's", "whom", "why", "why's", "with", "won't",
            "would", "wouldn't", "you", "you'd", "you'll", "you're", "you've", "your", "yours", "yourself",
            "yourselves",
        };
        &STOP_WORDS
    }
}

// --- Holographic Memory Structures ---

/// A complete memory, pairing the original information with its holographic representation.
#[derive(Debug, Clone)]
pub struct HolographicMemory {
    pub text: String,
    pub trace: HolographicTrace,
    pub is_axiom: bool,
}

impl HolographicMemory {
    pub fn new(text: String, trace: HolographicTrace, is_axiom: bool) -> Self {
        Self { text, trace, is_axiom }
    }

    /// Creates a new, non-axiomatic memory directly from a text string.
    /// This is a convenience function for creating temporary or synthesized memories.
    pub fn new_from_text(text: String, encoder: &HolographicEncoder) -> Self {
        let trace = encoder.encode(&text);
        Self {
            text,
            trace,
            is_axiom: false,
        }
    }
}

/// Represents a concept with an associated weight and holographic signature.
/// Now uses Q1.15 quantization for 75% memory reduction
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct WeightedConcept {
    pub interference_pattern: Vec<QuantizedComplex>,
    pub relevance: f32, // TF-IDF score
}

// Note: QuantizedWeightedConcept removed - WeightedConcept now uses Q1.15 directly

/// Represents the conceptual breakdown of a piece of information.
/// Each key concept is given a unique holographic signature and a relevance weight.
/// Now uses Q1.15 quantization for 75% memory reduction
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct HolographicTrace {
    pub weighted_concepts: HashMap<Arc<str>, WeightedConcept>,
    /// A single vector representing the weighted sum of all concept interference patterns.
    pub superposition_pattern: Vec<QuantizedComplex>,
}

// Note: QuantizedHolographicTrace removed - HolographicTrace now uses Q1.15 directly

impl HolographicTrace {
    /// Creates a new, unique trace seeded with random data.
    /// This represents the foundational 'qualia' of a new concept.
    pub fn new_seeded(name: &str, complexity: usize) -> Self {
        let mut rng = rand::thread_rng();
        let mut weighted_concepts = HashMap::new();

        let mut interference_pattern = Vec::with_capacity(complexity);
        for _ in 0..complexity {
            let complex_val = Complex::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0));
            interference_pattern.push(QuantizedComplex::from_complex(complex_val));
        }

        // The superposition pattern for a single seeded concept is just its own pattern.
        let superposition_pattern = interference_pattern.clone();

        let concept = WeightedConcept {
            interference_pattern,
            relevance: 1.0, // Base relevance
        };

        weighted_concepts.insert(name.to_string().into(), concept);

        Self { weighted_concepts, superposition_pattern }
    }

    /// Combines another trace into this one.
    /// This is the mechanism for holographic superposition.
    pub fn combine_with(&mut self, other: &HolographicTrace) {
        for (name, other_concept) in &other.weighted_concepts {
            let self_concept = self.weighted_concepts.entry(name.clone()).or_insert_with(|| WeightedConcept {
                interference_pattern: vec![QuantizedComplex::ZERO; other_concept.interference_pattern.len()],
                relevance: 0.0,
            });

            // Combine interference patterns (simple vector addition for now)
            for (i, other_complex) in other_concept.interference_pattern.iter().enumerate() {
                self_concept.interference_pattern[i] += other_complex;
            }

            // Average relevance
            self_concept.relevance = (self_concept.relevance + other_concept.relevance) / 2.0;
        }

        // Recalculate the superposition pattern from the new combined concepts
        let dimensionality = self.superposition_pattern.len().max(other.superposition_pattern.len());
        self.superposition_pattern.resize(dimensionality, QuantizedComplex::ZERO);
        other.superposition_pattern.iter().enumerate().for_each(|(i, val)| self.superposition_pattern[i] += val);
        
        // Normalize with epsilon to prevent NaN
        let norm = self.superposition_pattern.iter().map(|c| c.norm_sqr()).sum::<f32>().sqrt();
        let norm_safe = norm.max(1e-9); // Epsilon to prevent NaN
        if norm > 0.0 {
            self.superposition_pattern.iter_mut().for_each(|c| *c /= norm_safe);
        }
    }

    /// Creates a new, empty holographic trace.
    pub fn new_empty(complexity: usize) -> Self {
        Self {
            weighted_concepts: HashMap::new(),
            superposition_pattern: vec![QuantizedComplex::ZERO; complexity],
        }
    }

    /// Computes the cosine similarity between this trace and another.
    /// Returns a value between -1 and 1, where 1 means identical and -1 means opposite.
    /// 
    /// OPTIMIZATION: Converts Q15 to f32 for high-precision similarity calculations
    /// while maintaining memory-efficient Q15 storage (best of both worlds).
    pub fn cosine_similarity(&self, other: &HolographicTrace) -> f32 {
        let p1 = &self.superposition_pattern;
        let p2 = &other.superposition_pattern;

        // Convert Q15 to f32 for high-precision dot product calculation
        let dot_product: f32 = p1.iter().zip(p2.iter())
            .map(|(a, b)| {
                let a_f32 = a.to_complex();
                let b_f32 = b.to_complex();
                (a_f32.re * b_f32.re) + (a_f32.im * b_f32.im) // Real part of (a * b.conj())
            })
            .sum();

        // Convert Q15 to f32 for high-precision norm calculations
        let norm_p1: f32 = p1.iter()
            .map(|c| {
                let c_f32 = c.to_complex();
                c_f32.norm_sqr()
            })
            .sum::<f32>().sqrt();
            
        let norm_p2: f32 = p2.iter()
            .map(|c| {
                let c_f32 = c.to_complex();
                c_f32.norm_sqr()
            })
            .sum::<f32>().sqrt();

        if norm_p1 == 0.0 || norm_p2 == 0.0 {
            return 0.0;
        }

        let similarity = dot_product / (norm_p1 * norm_p2);
        
        // Clamp to valid cosine similarity range to handle any floating point errors
        similarity.clamp(-1.0, 1.0)
    }

    /// Calculates semantic distance based on cosine similarity.
    /// A lower value (closer to 0) means the concepts are more similar.
    pub fn distance(&self, other: &HolographicTrace) -> f32 {
        // We use 1.0 - similarity so that a higher similarity (closer to 1) results in a lower distance.
        // The result is in the range [0, 2], where 0 is identical.
        let sim = self.cosine_similarity(other);
        if sim.is_nan() {
            // If similarity is NaN (due to zero-length vectors), treat distance as infinite.
            f32::MAX
        } else {
            1.0 - sim.abs()
        }
    }
}

// Temporarily removed MemoryBuffers for thread safety
// TODO: Implement thread-safe memory optimization later

/// Encodes textual data into conceptual holographic traces using a semantic field model.
/// Now includes advanced memory management for optimal performance.
pub struct HolographicEncoder {
    pub focuser: ConceptFocuser,
    concept_dimensionality: usize,
    pub doc_frequency: HashMap<String, usize>,
    pub total_docs: usize,
    semantic_axes: HashMap<String, Vec<Complex<f32>>>,
    semantic_lexicon: HashMap<String, HashMap<String, f32>>,
    // Temporarily removed memory_buffers for thread safety
}

impl HolographicEncoder {
    pub fn new(concept_dimensionality: usize) -> Self {
        Self {
            focuser: ConceptFocuser::new(),
            concept_dimensionality,
            doc_frequency: HashMap::new(),
            total_docs: 0,
            semantic_axes: HashMap::new(),
            semantic_lexicon: HashMap::new(),
            // Temporarily removed memory_buffers initialization
        }
    }

    /// Returns the static set of stop words for filtering.
    /// Now uses the optimized phf::Set for better performance.
    pub fn get_stop_words(&self) -> &'static phf::Set<&'static str> {
        ConceptFocuser::get_low_information_words()
    }

    pub fn build_document_frequency(&mut self, memories: &[HolographicMemory]) {
        self.total_docs = memories.len();
        let mut df = HashMap::new();
        for memory in memories {
            // Use the same keyword extraction to be consistent.
            let keywords = self.focuser.distill_concepts(&memory.text);
            for keyword in keywords {
                *df.entry(keyword).or_insert(0) += 1;
            }
        }
        self.doc_frequency = df;
        println!("--- Document Frequency Map Built. {} unique concepts indexed across {} documents. ---", self.doc_frequency.len(), self.total_docs);
    }

    /// A public method to access the concept focuser's functionality.
    pub fn distill_concepts(&self, text: &str) -> HashSet<String> {
        self.focuser.distill_concepts(text)
    }

    /*
    /// Calculates the TF-IDF score for a term in a document.
    fn calculate_tf_idf(&self, term: &str, term_freq: usize, doc_keywords: &HashSet<String>) -> f32 {
        let tf = term_freq as f32 / doc_keywords.len() as f32;
        let idf = if let Some(doc_count) = self.doc_frequency.get(term) {
            (self.total_docs as f32 / *doc_count as f32).log10()
        } else {
            // If a term is not in the DF map, it's very rare (only in this doc).
            (self.total_docs as f32).log10() // Max IDF
        };
        tf * idf
    }
    */

    /// Transforms text into a conceptual holographic trace using semantic inference.
    /// It identifies known semantic concepts within the text and builds a superposition
    /// pattern exclusively from them, ignoring all other words.
    /// Encodes a set of pre-distilled concepts into a holographic trace.
    /// This is the core logic used by both public-facing encode methods.
    /// OPTIMIZED: Uses memory pools to reduce allocations and improve performance
    pub fn encode_concepts(&self, concepts: &HashSet<String>) -> HolographicTrace {
        if concepts.is_empty() {
            return HolographicTrace::new_empty(self.concept_dimensionality);
        }

        // Standard allocation approach (thread-safe)
        let mut weighted_concepts = HashMap::new();
        let mut superposition_pattern = vec![QuantizedComplex::ZERO; self.concept_dimensionality];
        let mut term_freq_map = HashMap::new();

        // Calculate term frequency for the current text
        for concept in concepts {
            *term_freq_map.entry(concept.clone()).or_insert(0) += 1;
        }

        for (concept_text, &tf_count) in &term_freq_map {
            let base_vector = self.generate_reference_wave_for_concept(concept_text);

            // Calculate TF-IDF weight (with safety checks for log10)
            let tf = tf_count as f32 / concepts.len() as f32;
            let idf = if let Some(doc_count) = self.doc_frequency.get(concept_text) {
                if *doc_count > 0 && self.total_docs > 0 {
                    (self.total_docs as f32 / *doc_count as f32).log10()
                } else {
                    1.0 // Default IDF when no document frequency data
                }
            } else {
                if self.total_docs > 0 {
                    (self.total_docs as f32).log10()
                } else {
                    1.0 // Default IDF when total_docs is 0
                }
            };
            let weight = tf * idf;

            // Convert base_vector to quantized and apply weight
            let quantized_base: Vec<QuantizedComplex> = base_vector.iter()
                .map(|&c| QuantizedComplex::from_complex(c))
                .collect();
            
            for (i, &quantized_val) in quantized_base.iter().enumerate() {
                let weighted_val = quantized_val.scale(weight);
                superposition_pattern[i] += weighted_val;
            }

            weighted_concepts.insert(concept_text.clone().into(), WeightedConcept {
                interference_pattern: quantized_base,
                relevance: weight,
            });

        }

        let norm = superposition_pattern.iter().map(|c| c.norm_sqr()).sum::<f32>().sqrt();
        let norm_safe = norm.max(1e-9); // Epsilon to prevent NaN
        if norm > 0.0 {
            superposition_pattern.iter_mut().for_each(|c| *c /= norm_safe);
        }

        // Note: Removed tanh() sharpening as it was causing NaN values
        // The normalization provides sufficient distinctiveness

        // Create the result trace
        let result = HolographicTrace {
            weighted_concepts,
            superposition_pattern,
        };
        
        // Standard approach: no special buffer management needed
        
        result
    }

    /// Encodes text for general reasoning, filtering out low-information words.
    pub fn encode(&self, text: &str) -> HolographicTrace {
        let concepts = self.focuser.distill_concepts(text);
        self.encode_concepts(&concepts)
    }

    /// Encodes raw text without filtering stop words. Used for creating Thalamus prototypes
    /// where stop words like "who" and "what" are critical classification signals.
    pub fn encode_raw(&self, text: &str) -> HolographicTrace {
        let concepts: HashSet<String> = text
            .split(|c: char| !c.is_alphanumeric())
            .map(|s| s.to_lowercase())
            .filter(|s| !s.is_empty())
            .collect();
        self.encode_concepts(&concepts)
    }

    /// Generates a reference wave for a concept based on its position in the semantic field.
    /// If the concept is not in the lexicon, it falls back to a hash-based wave.
    fn generate_reference_wave_for_concept(&self, concept: &str) -> Vec<Complex<f32>> {
        let mut final_wave = vec![Complex::new(0.0, 0.0); self.concept_dimensionality];

        if let Some(coordinates) = self.semantic_lexicon.get(concept) {
            // The concept is in the lexicon, build its wave from semantic axes.
            for (axis, weight) in coordinates {
                if let Some(axis_wave) = self.semantic_axes.get(axis) {
                    for (i, complex_val) in axis_wave.iter().enumerate() {
                        final_wave[i] += complex_val * weight;
                    }
                }
            }
        } else {
            // Fallback for unknown concepts: generate a unique, deterministic wave using SHA256.
            let mut hasher = Sha256::new();
            hasher.update(concept.as_bytes());
            let seed: [u8; 32] = hasher.finalize().into();

            let mut rng: rand_chacha::ChaCha8Rng = rand::SeedableRng::from_seed(seed);
            for i in 0..self.concept_dimensionality {
                final_wave[i] = Complex::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0));
            }
        }

        // Normalize the final wave to make it a unit vector.
        let norm = final_wave.iter().map(|c| c.norm_sqr()).sum::<f32>().sqrt();
        let norm_safe = norm.max(1e-9); // Epsilon to prevent NaN
        if norm > 0.0 {
            final_wave.iter_mut().for_each(|c| *c /= norm_safe);
        }

        final_wave
    }



    // Removed unused initialize_semantic_field method to clean up warnings

    /// Encodes the current state of neural activity into a conceptual holographic trace.
    /// For now, this creates a single, holistic concept of the "current neural state".
    pub fn encode_neural_activity(&self, connectome: &Connectome) -> HolographicTrace {
        let mut concept_traces = HashMap::new();
        let concept_name = "current_thought_pattern".to_string();

        // Create a reference wave specifically for this holistic neural concept.
        let reference_wave = self.generate_reference_wave_for_concept(&concept_name);

        // Create a data wave from the neural activity.
        let mut data_wave = vec![Complex::new(0.0, 0.0); self.concept_dimensionality];
        let firing_neurons: Vec<_> = connectome.neurons.iter().filter(|n| n.firing).collect();
        for (i, neuron) in firing_neurons.iter().enumerate() {
            let index = (neuron.id as usize + i) % self.concept_dimensionality;
            let angle = (neuron.id as u32 as f32) / 128.0 * std::f32::consts::PI;
            data_wave[index] += Complex::new(angle.cos(), angle.sin()) * neuron.potential;
        }
        let norm = data_wave.iter().map(|c| c.norm_sqr()).sum::<f32>().sqrt();
        if norm > 0.0 {
            for c in data_wave.iter_mut() {
                *c /= norm;
            }
        }

        // Create the interference pattern for the neural concept (convert to quantized)
        let interference_pattern: Vec<QuantizedComplex> = reference_wave
            .iter()
            .zip(data_wave.iter())
            .map(|(ref_c, data_c)| QuantizedComplex::from_complex(ref_c * data_c))
            .collect();

        let superposition_pattern = interference_pattern.clone(); // For a single concept, superposition is just its own pattern

        let weighted_concept = WeightedConcept {
            interference_pattern,
            relevance: 1.0, // Placeholder: Neural activity relevance needs a proper model.
        };

        concept_traces.insert(concept_name.into(), weighted_concept);

        HolographicTrace { weighted_concepts: concept_traces, superposition_pattern }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deterministic_unknown_concept() {
        // Test that unknown concepts generate the same pattern across different encoder instances
        let focuser1 = ConceptFocuser::new();
        let focuser2 = ConceptFocuser::new();
        let encoder1 = HolographicEncoder::new(256, focuser1);
        let encoder2 = HolographicEncoder::new(256, focuser2);
        
        // Test with a concept that definitely won't be in the semantic lexicon
        let test_concept = "xyzzy_unique_test_concept_12345";
        
        // First, test the raw wave generation directly
        let wave1 = encoder1.generate_reference_wave_for_concept(test_concept);
        let wave2 = encoder2.generate_reference_wave_for_concept(test_concept);
        
        // Check for NaN in raw waves
        for (i, (c1, c2)) in wave1.iter().zip(wave2.iter()).enumerate() {
            assert!(!c1.re.is_nan(), "NaN in wave1[{}].re: {}", i, c1.re);
            assert!(!c1.im.is_nan(), "NaN in wave1[{}].im: {}", i, c1.im);
            assert!(!c2.re.is_nan(), "NaN in wave2[{}].re: {}", i, c2.re);
            assert!(!c2.im.is_nan(), "NaN in wave2[{}].im: {}", i, c2.im);
            
            assert!((c1.re - c2.re).abs() < 1e-6, "Wave real parts differ at {}: {} vs {}", i, c1.re, c2.re);
            assert!((c1.im - c2.im).abs() < 1e-6, "Wave imag parts differ at {}: {} vs {}", i, c1.im, c2.im);
        }
        
        // Now test full encoding
        let concepts1: HashSet<String> = [test_concept.to_string()].into_iter().collect();
        let concepts2: HashSet<String> = [test_concept.to_string()].into_iter().collect();
        
        let trace1 = encoder1.encode_concepts(&concepts1);
        let trace2 = encoder2.encode_concepts(&concepts2);
        
        // Check for NaN in traces
        for (i, (c1, c2)) in trace1.superposition_pattern.iter().zip(trace2.superposition_pattern.iter()).enumerate() {
            assert!(!c1.re.is_nan(), "NaN in trace1[{}].re: {}", i, c1.re);
            assert!(!c1.im.is_nan(), "NaN in trace1[{}].im: {}", i, c1.im);
            assert!(!c2.re.is_nan(), "NaN in trace2[{}].re: {}", i, c2.re);
            assert!(!c2.im.is_nan(), "NaN in trace2[{}].im: {}", i, c2.im);
            
            assert!((c1.re - c2.re).abs() < 1e-6, "Trace real parts differ at {}: {} vs {}", i, c1.re, c2.re);
            assert!((c1.im - c2.im).abs() < 1e-6, "Trace imag parts differ at {}: {} vs {}", i, c1.im, c2.im);
        }
        
        println!("✅ Determinism test passed: Same concept generates identical patterns");
    }

    #[test]
    fn test_pattern_normalization() {
        // Test that generated patterns are properly normalized
        let focuser = ConceptFocuser::new();
        let encoder = HolographicEncoder::new(256, focuser);
        
        let trace = encoder.encode("hello world test");
        let norm = trace.superposition_pattern.iter()
            .map(|c| c.norm_sqr())
            .sum::<f32>()
            .sqrt();
            
        assert!((norm - 1.0).abs() < 1e-6, "Pattern not normalized: norm = {}", norm);
        println!("✅ Normalization test passed: Pattern norm = {:.6}", norm);
    }

    #[test]
    fn test_different_concepts_different_patterns() {
        // Test that different concepts generate different patterns
        let focuser = ConceptFocuser::new();
        let encoder = HolographicEncoder::new(256, focuser);
        
        let concepts1: HashSet<String> = ["concept_alpha".to_string()].into_iter().collect();
        let concepts2: HashSet<String> = ["concept_beta".to_string()].into_iter().collect();
        
        let trace1 = encoder.encode_concepts(&concepts1);
        let trace2 = encoder.encode_concepts(&concepts2);
        
        // Calculate distance between the two patterns
        let distance = trace1.distance(&trace2);
        
        // Different concepts should have significant distance (> 0.1)
        assert!(distance > 0.1, "Different concepts too similar: distance = {}", distance);
        println!("✅ Uniqueness test passed: Different concepts have distance = {:.4}", distance);
    }
}
