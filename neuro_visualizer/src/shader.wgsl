// Basic shader for synaptic activity visualization (circles)
// Vertex shader - simple pass-through for a full-screen triangle
@vertex
fn vs_main(@builtin(vertex_index) in_vertex_index: u32) -> @builtin(position) vec4<f32> {
    let x = f32(in_vertex_index / 2u) * 4.0 - 1.0;
    let y = f32(in_vertex_index % 2u) * 4.0 - 1.0;
    return vec4<f32>(x, y, 0.0, 1.0);
}

// We no longer need the Ripple struct, only the uniforms.
struct Uniforms {
    time: f32,
    resolution_x: f32,
    resolution_y: f32,
    awareness_level: f32,
}

@group(0) @binding(0)
var<uniform> uniforms: Uniforms;

// --- Simplex Noise Functions ---
// These functions create a procedural, organic-looking "noise" pattern.
// It's more complex than simple random noise, giving a flowing, liquid-like appearance.

fn mod289_v3(x: vec3<f32>) -> vec3<f32> {
    return x - floor(x * (1.0 / 289.0)) * 289.0;
}

fn mod289_v2(x: vec2<f32>) -> vec2<f32> {
    return x - floor(x * (1.0 / 289.0)) * 289.0;
}

fn permute(x: vec3<f32>) -> vec3<f32> {
    return mod289_v3(((x * 34.0) + 1.0) * x);
}

fn snoise(v: vec2<f32>) -> f32 {
    let C = vec4<f32>(0.2113248654, 0.3660254038, -0.5773502692, 0.0243902439);
    var i = floor(v + dot(v, C.yy));
    let x0 = v - i + dot(i, C.xx);
    var i1 = select(vec2<f32>(0.0, 1.0), vec2<f32>(1.0, 0.0), x0.x > x0.y);
    let x1 = x0 - i1 + C.xx;
    let x2 = x0 - 1.0 + 2.0 * C.xx;
    i = mod289_v2(i);
    let p = permute(permute(i.y + vec3<f32>(0.0, i1.y, 1.0)) + i.x + vec3<f32>(0.0, i1.x, 1.0));
    var m = max(0.5 - vec3<f32>(dot(x0, x0), dot(x1, x1), dot(x2, x2)), vec3<f32>(0.0));
    m = m * m;
    m = m * m;
    let x = 2.0 * fract(p * C.www) - 1.0;
    let h = abs(x) - 0.5;
    let ox = floor(x + 0.5);
    let a0 = x - ox;
    m = m * (1.79284291400159 - 0.85373472095314 * (a0 * a0 + h * h));
    let g = vec3<f32>(a0.x * x0.x + h.x * x0.y, a0.y * x1.x + h.y * x1.y, a0.z * x2.x + h.z * x2.y);
    return 130.0 * dot(m, g);
}


@fragment
fn fs_main(@builtin(position) frag_coord: vec4<f32>) -> @location(0) vec4<f32> {
    let resolution = vec2<f32>(uniforms.resolution_x, uniforms.resolution_y);
    var uv = frag_coord.xy / resolution.xy;

    // --- Control parameters based on awareness ---
    let awareness = uniforms.awareness_level;

    // As awareness increases, the animation slows down.
    let time_speed = mix(2.0, 0.1, awareness);
    // As awareness increases, the noise pattern becomes larger and less detailed.
    let zoom = mix(5.0, 2.0, awareness);
    // As awareness increases, the distortion effect lessens.
    let distortion_amount = mix(0.8, 0.0, awareness);

    // --- Noise calculation ---
    // We use multiple layers of noise ("octaves") to create a more detailed effect.
    let p = uv * zoom;
    let t = uniforms.time * time_speed;

    var f = 0.0;
    f += 0.5000 * snoise(p + t);
    f += 0.2500 * snoise(p * 2.0 + t * 1.5);
    f += 0.1250 * snoise(p * 4.0 + t * 2.0);
    f += 0.0625 * snoise(p * 8.0 + t * 2.5);

    // --- Distortion --- 
    // We use one layer of noise to distort the coordinates of another.
    // This creates the swirling, liquid-like effect.
    let q = vec2<f32>(f, f);
    let r = vec2<f32>(
        snoise(p + q * distortion_amount + vec2<f32>(1.7, 9.2) + t),
        snoise(p + q * distortion_amount + vec2<f32>(8.3, 2.8) + t)
    );

    // --- Coloring ---
    // The final color is a mix between a dark, chaotic color and a bright, calm color.
    let dark_color = vec3<f32>(0.0, 0.0, 0.05); // Deep, dark blue
    let mid_color = vec3<f32>(0.1, 0.3, 0.8);  // A vibrant blue
    let bright_color = vec3<f32>(0.8, 0.9, 1.0); // Bright, almost white-blue

    // The noise value `r.x` controls the mix between the dark and mid colors.
    var color = mix(dark_color, mid_color, smoothstep(-0.1, 0.4, r.x));

    // As awareness grows, we blend in the bright, calm color.
    color = mix(color, bright_color, awareness * awareness);

    return vec4<f32>(color, 1.0);
}
