// Struct for screen resolution and time uniforms
struct Uniforms {
    time: f32,
    resolution_x: f32,
    resolution_y: f32,
    // This field is not used in this shader, but it's needed to match the memory layout
    // of the Rust Uniforms struct, which is shared between both shaders.
    num_columns: u32, 
};

@group(0) @binding(0)
var<uniform> uniforms: Uniforms;

// Storage buffer for EEG data points (values between -1.0 and 1.0)
@group(0) @binding(1)
var<storage, read> eeg_data: array<f32>;

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) frag_coord: vec2<f32>,
};

// Standard full-screen triangle vertex shader
@vertex
fn vs_main(@builtin(vertex_index) in_vertex_index: u32) -> VertexOutput {
    var out: VertexOutput;

    // A single, large triangle that covers the entire clip space.
    // This is more reliable than bit-twiddling tricks.
    let pos = array<vec2<f32>, 3>(
        vec2<f32>(-1.0, -1.0),
        vec2<f32>( 3.0, -1.0),
        vec2<f32>(-1.0,  3.0)
    );

    let p = pos[in_vertex_index];
    out.clip_position = vec4<f32>(p, 0.0, 1.0);
    out.frag_coord = p;
    return out;
}

// Function to calculate the minimum distance from a point `p` to a line segment `a` -> `b`
fn dist_to_segment(p: vec2<f32>, a: vec2<f32>, b: vec2<f32>) -> f32 {
    let pa = p - a;
    let ba = b - a;
    let h = clamp(dot(pa, ba) / dot(ba, ba), 0.0, 1.0);
    return length(pa - ba * h);
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let uv = (in.frag_coord + 1.0) / 2.0; // Convert frag_coord from [-1,1] to [0,1]
    let frag_coord_px = uv * vec2<f32>(uniforms.resolution_x, uniforms.resolution_y);

    let num_points = arrayLength(&eeg_data);
    var min_dist = 1e9; // Initialize with a large value

    // Loop through all line segments to find the closest one to the current fragment
    for (var i = 0u; i < num_points - 1u; i = i + 1u) {
        // Convert data points from normalized coordinates to pixel coordinates
        let p1_norm = vec2<f32>((f32(i) / f32(num_points - 1u)), (eeg_data[i] + 1.0) / 2.0);
        let p2_norm = vec2<f32>((f32(i + 1u) / f32(num_points - 1u)), (eeg_data[i + 1u] + 1.0) / 2.0);

        let p1 = p1_norm * vec2<f32>(uniforms.resolution_x, uniforms.resolution_y);
        let p2 = p2_norm * vec2<f32>(uniforms.resolution_x, uniforms.resolution_y);

        min_dist = min(min_dist, dist_to_segment(frag_coord_px, p1, p2));
    }

    let line_color = vec3<f32>(0.0, 1.0, 0.8); // Bright cyan-green
    let glow_radius_px = 10.0;
    let line_width_px = 1.5;

    // Calculate glow intensity based on distance
    let intensity = smoothstep(glow_radius_px, 0.0, min_dist);

    // Sharpen the core line
    let core_line = smoothstep(line_width_px, 0.0, min_dist);

    let final_color = line_color * (intensity * 0.5 + core_line * 0.5);
    let final_alpha = intensity;

    return vec4<f32>(final_color, final_alpha);
}
