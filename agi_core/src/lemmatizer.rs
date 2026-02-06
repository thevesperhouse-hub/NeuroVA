//! A simple rule-based French lemmatizer.

// A basic set of rules for French lemmatization.
// This is a starting point and can be expanded significantly.
const RULES: &[(&str, &str)] = &[
    // Noun and Adjective Endings (plurals, feminine, etc.) - Longest first
    ("euses", "eux"),     // e.g., "heureuses" -> "heureux"
    ("eaux", "eau"),      // e.g., "bateaux" -> "bateau"
    ("elles", "el"),      // e.g., "nouvelles" -> "nouvel"
    ("aux", "al"),        // e.g., "journaux" -> "journal"
    ("euse", "eux"),      // e.g., "heureuse" -> "heureux"
    ("ives", "if"),       // e.g., "sportives" -> "sportif"
    ("elle", "el"),       // e.g., "nouvelle" -> "nouvel"
    ("ive", "if"),        // e.g., "sportive" -> "sportif"

    // Verb Endings (ordered by suffix length to avoid conflicts)

    // Subjonctif Imparfait
    ("assent", "er"),     // e.g., "parlassent" -> "parler"
    ("assiez", "er"),     // e.g., "parlassiez" -> "parler"
    ("assions", "er"),    // e.g., "parlassions" -> "parler"
    ("issent", "ir"),     // e.g., "finissent" -> "finir"
    ("ussiez", "re"),     // e.g., "vendussiez" -> "vendre"
    ("ussions", "re"),    // e.g., "vendussions" -> "vendre"
    ("asses", "er"),      // e.g., "parlasses" -> "parler"
    ("isse", "ir"),       // e.g., "finisse" -> "finir"
    ("usse", "re"),       // e.g., "vendusse" -> "vendre"
    ("ât", "er"),         // e.g., "parlât" -> "parler"
    ("ît", "ir"),         // e.g., "finît" -> "finir"
    ("ût", "re"),         // e.g., "vendût" -> "vendre"

    // Imparfait / Conditionnel
    ("issaient", "ir"),   // e.g., "finissaient" -> "finir"
    ("eraient", "er"),     // e.g., "parleraient" -> "parler"
    ("issions", "ir"),     // e.g., "finissions" -> "finir"
    ("issiez", "ir"),      // e.g., "finissiez" -> "finir"
    ("erions", "er"),      // e.g., "parlerions" -> "parler"
    ("eriez", "er"),       // e.g., "parleriez" -> "parler"
    ("aient", "er"),       // e.g., "parlaient" -> "parler"
    ("issait", "ir"),      // e.g., "finissait" -> "finir"
    ("issais", "ir"),      // e.g., "finissais" -> "finir"
    ("erait", "er"),       // e.g., "parlerait" -> "parler"
    ("erais", "er"),       // e.g., "parlerais" -> "parler"
    ("ions", "er"),        // e.g., "parlions" -> "parler"
    ("iez", "er"),         // e.g., "parliez" -> "parler"
    ("ait", "er"),         // e.g., "parlait" -> "parler"
    ("ais", "er"),         // e.g., "parlais" -> "parler"

    // Futur
    ("eront", "er"),       // e.g., "parleront" -> "parler"
    ("erons", "er"),       // e.g., "parlerons" -> "parler"
    ("erez", "er"),        // e.g., "parlerez" -> "parler"
    ("erai", "er"),        // e.g., "parlerai" -> "parler"
    ("eras", "er"),        // e.g., "parleras" -> "parler"
    ("era", "er"),         // e.g., "parlera" -> "parler"

    // Passé Simple
    ("èrent", "er"),       // e.g., "parlèrent" -> "parler"
    ("irent", "ir"),       // e.g., "finirent" -> "finir"
    ("urent", "re"),       // e.g., "vendurent" -> "vendre"
    ("âmes", "er"),        // e.g., "parlâmes" -> "parler"
    ("îmes", "ir"),        // e.g., "finîmes" -> "finir"
    ("ûmes", "re"),        // e.g., "vendûmes" -> "vendre"
    ("âtes", "er"),        // e.g., "parlâtes" -> "parler"
    ("îtes", "ir"),        // e.g., "finîtes" -> "finir"
    ("ûtes", "re"),        // e.g., "vendûtes" -> "vendre"

    // Présent
    ("issant", "ir"),     // e.g., "finissant" -> "finir"
    ("ons", "er"),         // e.g., "parlons" -> "parler"
    ("ez", "er"),          // e.g., "parlez" -> "parler"
    ("ent", "er"),         // e.g., "parlent" -> "parler"

    // Participe Passé
    ("ées", "er"),         // e.g., "parlées" -> "parler"
    ("ée", "er"),          // e.g., "parlée" -> "parler"
    ("és", "er"),          // e.g., "parlés" -> "parler"
    ("é", "er"),           // e.g., "parlé" -> "parler"
    ("is", "ir"),          // e.g., "finis" -> "finir"
    ("it", "ir"),          // e.g., "finit" -> "finir"
    ("u", "re"),           // e.g., "vendu" -> "vendre"

    // General plural 's' (lowest priority)
    ("s", ""),             // e.g., "chats" -> "chat"
];

/// Lemmatizes a French word based on a simple set of suffix-replacement rules.
pub fn lemmatize(word: &str) -> String {
    if word.len() <= 3 { // Avoid lemmatizing very short words
        return word.to_string();
    }

    for (suffix, replacement) in RULES.iter() {
        if word.ends_with(suffix) {
            // A very basic check to avoid over-lemmatization like "bus" -> "bu"
            if *suffix == "s" && word.ends_with("ss") {
                continue;
            }
            return format!("{}{}", &word[..word.len() - suffix.len()], replacement);
        }
    }

    word.to_string()
}
