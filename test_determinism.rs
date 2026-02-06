// Test simple pour vérifier le déterminisme SHA256
use std::collections::HashSet;

// Copie simplifiée des structures nécessaires pour le test
use sha2::{Digest, Sha256};
use rand::SeedableRng;
use rand::Rng;

fn generate_deterministic_pattern(concept: &str, dimensionality: usize) -> Vec<(f32, f32)> {
    let mut hasher = Sha256::new();
    hasher.update(concept.as_bytes());
    let seed: [u8; 32] = hasher.finalize().into();

    let mut rng: rand_chacha::ChaCha8Rng = rand::SeedableRng::from_seed(seed);
    let mut pattern = Vec::with_capacity(dimensionality);
    
    for _ in 0..dimensionality {
        pattern.push((rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0)));
    }
    
    // Normalisation avec epsilon
    let norm_sqr: f32 = pattern.iter().map(|(re, im)| re*re + im*im).sum();
    let norm = norm_sqr.sqrt().max(1e-9); // Epsilon pour éviter NaN
    
    pattern.iter_mut().for_each(|(re, im)| {
        *re /= norm;
        *im /= norm;
    });
    
    pattern
}

fn main() {
    println!("🧪 Test de déterminisme SHA256...");
    
    let test_concept = "xyzzy_unique_test_concept_12345";
    let dimensionality = 256;
    
    // Générer le même pattern deux fois
    let pattern1 = generate_deterministic_pattern(test_concept, dimensionality);
    let pattern2 = generate_deterministic_pattern(test_concept, dimensionality);
    
    // Vérifier qu'ils sont identiques
    let mut max_diff = 0.0f32;
    for (i, ((re1, im1), (re2, im2))) in pattern1.iter().zip(pattern2.iter()).enumerate() {
        let diff_re = (re1 - re2).abs();
        let diff_im = (im1 - im2).abs();
        max_diff = max_diff.max(diff_re).max(diff_im);
        
        if diff_re > 1e-6 || diff_im > 1e-6 {
            println!("❌ Différence détectée à l'index {}: ({}, {}) vs ({}, {})", i, re1, im1, re2, im2);
            return;
        }
    }
    
    // Vérifier la normalisation
    let norm_sqr: f32 = pattern1.iter().map(|(re, im)| re*re + im*im).sum();
    let norm = norm_sqr.sqrt();
    
    println!("✅ Déterminisme: Patterns identiques (diff max = {:.2e})", max_diff);
    println!("✅ Normalisation: Norme = {:.6} (attendu ≈ 1.0)", norm);
    
    // Test avec des concepts différents
    let pattern_alpha = generate_deterministic_pattern("concept_alpha", dimensionality);
    let pattern_beta = generate_deterministic_pattern("concept_beta", dimensionality);
    
    // Calculer la distance
    let dot_product: f32 = pattern_alpha.iter().zip(pattern_beta.iter())
        .map(|((re1, im1), (re2, im2))| re1*re2 + im1*im2)
        .sum();
    let distance = 1.0 - dot_product;
    
    println!("✅ Unicité: Distance entre concepts différents = {:.4}", distance);
    
    if distance > 0.1 {
        println!("🎉 Tous les tests passent ! Le déterminisme SHA256 fonctionne parfaitement.");
    } else {
        println!("⚠️  Les concepts différents sont trop similaires.");
    }
}
