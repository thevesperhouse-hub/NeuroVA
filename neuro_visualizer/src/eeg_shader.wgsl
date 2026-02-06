// EEG line graph shader that generates vertices on the fly.

@group(0) @binding(1) var<storage, read> eeg_data: array<f32>;

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
};

@vertex
fn vs_main(@builtin(vertex_index) in_vertex_index: u32) -> VertexOutput {
    let num_points = arrayLength(&eeg_data);
    if (in_vertex_index >= num_points) {
        // Prevent out-of-bounds access if the buffer is not yet full.
        return VertexOutput(vec4<f32>(-2.0, -2.0, 0.0, 1.0));
    }

    // Map the vertex index to an x-coordinate from -1.0 to 1.0
    let x = -1.0 + (f32(in_vertex_index) / f32(num_points - 1u)) * 2.0;
    
    // Use the EEG data from the storage buffer as the y-coordinate, with some scaling.
    let y = eeg_data[in_vertex_index] * 0.5;

    var out: VertexOutput;
    out.clip_position = vec4<f32>(x, y, 0.0, 1.0);
    return out;
}

@fragment
fn fs_main() -> @location(0) vec4<f32> {
    return vec4<f32>(0.1, 0.8, 0.4, 1.0); // Cyan-green for a more modern look
}
