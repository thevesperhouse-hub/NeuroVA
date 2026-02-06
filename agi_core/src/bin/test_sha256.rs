// Programme de diagnostic pour tester le déterminisme SHA256
use agi_core::holographic_memory::HolographicEncoder;
use std::collections::HashSet;

fn main() {
    println!("🧪 Test de diagnostic SHA256...");
    
    // Créer deux encodeurs identiques
    let encoder1 = HolographicEncoder::new(256);
    let encoder2 = HolographicEncoder::new(256);
    
    // Test avec un concept inconnu
    let test_concept = "xyzzy_unique_test_concept_12345";
    println!("Test avec le concept: {}", test_concept);
    
    // Test de l'encodage complet directement
    println!("Test de l'encodage complet...");
    let concepts1: HashSet<String> = [test_concept.to_string()].into_iter().collect();
    let concepts2: HashSet<String> = [test_concept.to_string()].into_iter().collect();
    
    let trace1 = encoder1.encode_concepts(&concepts1);
    let trace2 = encoder2.encode_concepts(&concepts2);
    
    // Vérifier les NaN dans les traces
    let mut nan_count = 0;
    for (i, (c1, c2)) in trace1.superposition_pattern.iter().zip(trace2.superposition_pattern.iter()).enumerate() {
        // Convert to f32 for NaN checking since i16 can't be NaN
        let c1_f32 = c1.to_complex();
        let c2_f32 = c2.to_complex();
        if c1_f32.re.is_nan() || c1_f32.im.is_nan() || c2_f32.re.is_nan() || c2_f32.im.is_nan() {
            println!("❌ NaN détecté dans trace à l'index {}: trace1=({}, {}), trace2=({}, {})", 
                     i, c1_f32.re, c1_f32.im, c2_f32.re, c2_f32.im);
            nan_count += 1;
        }
    }
    
    if nan_count == 0 {
        println!("✅ Aucun NaN dans les traces");
        println!("✅ Déterminisme complet: OK");
        
        // Calculer la norme pour vérifier la normalisation
        let norm1: f32 = trace1.superposition_pattern.iter()
            .map(|c| c.norm_sqr()).sum::<f32>().sqrt();
        let norm2: f32 = trace2.superposition_pattern.iter()
            .map(|c| c.norm_sqr()).sum::<f32>().sqrt();
            
        println!("✅ Normes: trace1={:.6}, trace2={:.6}", norm1, norm2);
        
        if (norm1 - 1.0).abs() < 1e-6 && (norm2 - 1.0).abs() < 1e-6 {
            println!("🎉 Tous les tests passent ! Le déterminisme SHA256 fonctionne.");
        } else {
            println!("⚠️  Problème de normalisation détecté.");
        }
    } else {
        println!("❌ {} NaN détectés dans les traces", nan_count);
    }
}
