// agi_core/src/trace_visualizer.rs

use crate::holographic_memory::HolographicTrace;
use image::{Rgb, RgbImage};
use nalgebra::ComplexField;
use std::f32::consts::PI;

/// Generates a unique visual representation (a "mandala") of a holographic trace.
///
/// # Arguments
/// * `trace` - The holographic trace to visualize.
/// * `width` - The width of the output image.
/// * `height` - The height of the output image.
///
/// # Returns
/// An `RgbImage` representing the trace.
pub fn generate_trace_image(trace: &HolographicTrace, width: u32, height: u32) -> RgbImage {
    let mut img = RgbImage::new(width, height);
    let center_x = width as f32 / 2.0;
    let center_y = height as f32 / 2.0;
    let max_radius = width.min(height) as f32 / 2.5;
    let num_symmetry_axes = 8;

    if trace.weighted_concepts.is_empty() {
        return img; // Return black image if trace is empty
    }

    // Normalize all magnitudes across the entire trace to get better color/brightness distribution
    let mut max_magnitude = 0.0;
    for concept in trace.weighted_concepts.values() {
        for c in &concept.interference_pattern {
            let mag = c.norm();
            if mag > max_magnitude {
                max_magnitude = mag;
            }
        }
    }
    if max_magnitude == 0.0 { max_magnitude = 1.0; } // Avoid division by zero

    for (concept_name, concept) in &trace.weighted_concepts {
        // Use a djb2-like hash function to get a well-distributed base hue from the concept name.
        let mut hash = 5381_u32;
        for byte in concept_name.bytes() {
            hash = (hash.wrapping_shl(5)).wrapping_add(hash).wrapping_add(byte as u32); // hash * 33 + byte
        }
        let base_hue = (hash % 360) as f32;

        let concept_relevance = concept.relevance;

        for (i, c) in concept.interference_pattern.iter().enumerate() {
            let magnitude = c.norm() / max_magnitude; // Normalized magnitude (0 to 1)
            let phase = c.argument(); // Phase (-PI to PI)

            // Use the base hue for the concept, and modulate S & V with the trace data
            let hue = base_hue;
            let saturation = 0.6 + (magnitude * 0.4); // From 0.6 to 1.0
            let value = 0.5 + ((phase + PI) / (2.0 * PI)) * 0.5; // From 0.5 to 1.0

            let color = hsv_to_rgb(hue, saturation, value);

            // Map vector index to a position. The concept's contribution is now color-coded.
            let radius = (i as f32 / concept.interference_pattern.len() as f32) * max_radius;
            let angle = phase;

            // Draw with symmetry
            for j in 0..num_symmetry_axes {
                let symmetry_angle = 2.0 * PI * (j as f32) / (num_symmetry_axes as f32);
                
                // Main point
                let x1 = center_x + radius * (angle + symmetry_angle).cos();
                let y1 = center_y + radius * (angle + symmetry_angle).sin();

                // Mirrored point
                let x2 = center_x + radius * (angle - symmetry_angle).cos();
                let y2 = center_y + radius * (angle - symmetry_angle).sin();

                let point_radius = (magnitude * 3.0 + concept_relevance * 3.0) as i32 + 1;
                draw_filled_circle(&mut img, x1 as i32, y1 as i32, point_radius, color);
                draw_filled_circle(&mut img, x2 as i32, y2 as i32, point_radius, color);
            }
        }
    }

    img
}

/// Helper function to convert HSV to RGB.
fn hsv_to_rgb(h: f32, s: f32, v: f32) -> Rgb<u8> {
    let c = v * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = v - c;

    let (r_prime, g_prime, b_prime) = if h >= 0.0 && h < 60.0 {
        (c, x, 0.0)
    } else if h >= 60.0 && h < 120.0 {
        (x, c, 0.0)
    } else if h >= 120.0 && h < 180.0 {
        (0.0, c, x)
    } else if h >= 180.0 && h < 240.0 {
        (0.0, x, c)
    } else if h >= 240.0 && h < 300.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };

    Rgb([
        ((r_prime + m) * 255.0) as u8,
        ((g_prime + m) * 255.0) as u8,
        ((b_prime + m) * 255.0) as u8,
    ])
}

/// Helper function to draw a filled circle on the image.
fn draw_filled_circle(img: &mut RgbImage, cx: i32, cy: i32, radius: i32, color: Rgb<u8>) {
    for x in (cx - radius)..=(cx + radius) {
        for y in (cy - radius)..=(cy + radius) {
            if (x - cx).pow(2) + (y - cy).pow(2) <= radius.pow(2) {
                if x >= 0 && x < img.width() as i32 && y >= 0 && y < img.height() as i32 {
                    img.put_pixel(x as u32, y as u32, color);
                }
            }
        }
    }
}
