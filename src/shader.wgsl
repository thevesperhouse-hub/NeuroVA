@vertex
fn vs_main(@builtin(vertex_index) in_vertex_index: u32) -> @builtin(position) vec4<f32> {
    // Génère un triangle qui couvre tout l'écran. C'est une astuce courante
    // pour les effets de post-processing ou les fonds dynamiques.
    let x = f32(in_vertex_index / 2u) * 4.0 - 1.0;
    let y = f32(in_vertex_index % 2u) * 4.0 - 1.0;
    return vec4<f32>(x, y, 0.0, 1.0);
}

struct Column {
    pos: vec2<f32>,
    state: f32, // 0.0 = inactive, 1.0 = firing
    // padding is implicitly handled by WGSL's layout rules
};



struct Uniforms {
    time: f32,
    resolution_x: f32,
    resolution_y: f32,
    num_columns: u32,
}

@group(0) @binding(0)
var<uniform> uniforms: Uniforms;

@group(0) @binding(1)
var<storage, read> columns: array<Column>;

// Helper function to draw a blurred circle
fn circle(uv: vec2<f32>, center: vec2<f32>, radius: f32, blur: f32) -> f32 {
    let d = distance(uv, center);
    return smoothstep(radius, radius - blur, d);
}

@fragment
fn fs_main(@builtin(position) frag_coord: vec4<f32>) -> @location(0) vec4<f32> {
    let resolution = vec2<f32>(uniforms.resolution_x, uniforms.resolution_y);
    // Center coordinates and correct for aspect ratio.
    // This makes (0,0) the center of the screen.
    let uv = (frag_coord.xy - 0.5 * resolution) / resolution.y;
    
    var final_color = vec3<f32>(0.0, 0.0, 0.02); // Even darker blue background for more contrast

    let num_columns = arrayLength(&columns);
    for (var i: u32 = 0u; i < num_columns; i = i + 1u) {
        let col = columns[i];
        
        // The intensity is now directly driven by the neuron's state from the simulation
        // We use smoothstep to create a soft, non-linear glow that feels more organic
        let intensity = smoothstep(0.0, 0.8, col.state);

        if (intensity > 0.01) {
            // Define a more pleasant, cyan/blue color palette
            let base_color = vec3<f32>(0.1, 0.7, 0.9);
            
            // Draw the circle with a soft falloff for a glowy/bloom effect
            let c = circle(uv, col.pos, 0.03, 0.03 * 0.9) * intensity;
            
            // Additive blending creates the bloom where circles overlap
            final_color += c * base_color * 1.5;
        }
    }

    return vec4<f32>(final_color, 1.0);
}
