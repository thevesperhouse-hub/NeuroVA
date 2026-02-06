//! `prompt_segmenter` - Module pour la segmentation biomimétique des prompts.
//!
//! Ce module a pour but de décomposer une requête utilisateur complexe en plusieurs sous-requêtes
//! ou intentions distinctes. Plutôt que de se baser sur de simples séparateurs comme "et",
//! il utilise une approche heuristique pour simuler une compréhension plus naturelle du langage,
//! en accord avec les principes biomimétiques du projet.

/// Segmente un prompt en plusieurs sous-prompts basés sur des heuristiques.
///
/// # Arguments
/// * `prompt` - La chaîne de caractères représentant la requête de l'utilisateur.
///
/// # Retourne
/// Un `Vec<String>` contenant les sous-prompts identifiés.
pub fn segment_prompt(prompt: &str) -> Vec<String> {
    let mut final_segments = Vec::new();

    // 1. Première passe : découpage par la ponctuation forte (phrases).
    let sentence_delimiters = ['.', '?', '!'];
    let mut last_cut = 0;
    for (i, char) in prompt.char_indices() {
        if sentence_delimiters.contains(&char) {
            let sentence = prompt[last_cut..=i].trim();
            if !sentence.is_empty() {
                final_segments.push(sentence.to_string());
            }
            last_cut = i + 1;
        }
    }
    // Ajouter le reste de la chaîne s'il n'y a pas de ponctuation finale
    let remainder = prompt[last_cut..].trim();
    if !remainder.is_empty() {
        final_segments.push(remainder.to_string());
    }

    // Si aucun délimiteur n'a été trouvé, on traite le prompt entier.
    if final_segments.is_empty() {
        final_segments.push(prompt.to_string());
    }

    // 2. Deuxième passe : affiner les segments en les divisant par des conjonctions.
    let conjunctions = [" et ", " puis ", " ensuite "];
    let mut refined_segments = Vec::new();

    for segment in final_segments {
        // Pour l'instant, nous ne gérons qu'une seule conjonction pour la simplicité.
        // Une future amélioration pourrait gérer des découpages multiples.
        let mut split_done = false;
        for conj in &conjunctions {
            if let Some(index) = segment.find(conj) {
                let first_part = &segment[..index];
                let second_part = &segment[index + conj.len()..];
                refined_segments.push(first_part.trim().to_string());
                refined_segments.push(second_part.trim().to_string());
                split_done = true;
                break; // On ne traite qu'une seule conjonction par segment
            }
        }
        if !split_done {
            refined_segments.push(segment);
        }
    }

    refined_segments.into_iter().filter(|s| !s.is_empty()).collect()
}
