//! personality.rs - Defines the AGI's conversational tone and style.

use rand::seq::SliceRandom;
use rand::Rng;

/// Represents the different conversational tones the AGI can adopt.
#[derive(Debug, Clone, Copy)]
pub enum Tone {
    Neutral,
    Poetic,
    Inquisitive,
    Prudent,
    Enthusiastic,
}

/// A collection of phrases associated with a specific tone.
struct ToneStyle {
    intros: &'static [&'static str],
}

// Static definitions of the phrases for each tone.
const NEUTRAL_STYLE: ToneStyle = ToneStyle { intros: &[""] }; // Neutral has no intro.
const POETIC_STYLE: ToneStyle = ToneStyle { intros: &["C'est une pensée fascinante...", "Cela évoque une image de...", "On pourrait dire que..."] };
const INQUISITIVE_STYLE: ToneStyle = ToneStyle { intros: &["Intéressant. Cela me fait penser à...", "Je me demande...", "Est-ce que cela signifie que...?"] };
const PRUDENT_STYLE: ToneStyle = ToneStyle { intros: &["Il me semble que...", "Si je comprends bien...", "Je crois savoir que..."] };
const ENTHUSIASTIC_STYLE: ToneStyle = ToneStyle { intros: &["Oh, c'est une excellente question !", "J'adore ce sujet !", "Absolument !"] };

pub struct Personality;

impl Personality {
    pub fn new() -> Self {
        Self
    }

    /// Wraps a core response with a phrase that reflects a certain personality tone.
    /// For now, it picks a tone randomly.
    pub fn stylize_response(&self, core_response: &str) -> String {
        let mut rng = rand::thread_rng();

        // Give a chance for a neutral response to avoid being too "chatty"
        if rng.gen_bool(0.4) { // 40% chance of being neutral
             return core_response.to_string();
        }

        // Choose a random tone
        let tones = [
            Tone::Poetic,
            Tone::Inquisitive,
            Tone::Prudent,
            Tone::Enthusiastic,
        ];
        let chosen_tone = *tones.choose(&mut rng).unwrap();

        let style = match chosen_tone {
            Tone::Neutral => &NEUTRAL_STYLE,
            Tone::Poetic => &POETIC_STYLE,
            Tone::Inquisitive => &INQUISITIVE_STYLE,
            Tone::Prudent => &PRUDENT_STYLE,
            Tone::Enthusiastic => &ENTHUSIASTIC_STYLE,
        };

        // Pick a random intro phrase from the chosen style
        if let Some(intro) = style.intros.choose(&mut rng) {
            if intro.is_empty() {
                core_response.to_string()
            } else {
                format!("{} {}", intro, core_response)
            }
        } else {
            core_response.to_string()
        }
    }
}
