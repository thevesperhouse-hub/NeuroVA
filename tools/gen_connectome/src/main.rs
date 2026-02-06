use rand::Rng;
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

const NUM_NEURONS: u64 = 1_000;
const NUM_SYNAPSES: u64 = 100_000;

fn main() -> io::Result<()> {
    println!("Generating connectome with {} neurons and {} synapses...", NUM_NEURONS, NUM_SYNAPSES);

    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let workspace_root = Path::new(manifest_dir).parent().unwrap().parent().unwrap(); // Go up two levels from tools/gen_connectome
    let output_file = workspace_root.join("quantized_connectome.bin");
    let mut file = File::create(&output_file)?;
    let mut rng = rand::thread_rng();

    // 1. Write number of neurons (u64)
    file.write_all(&NUM_NEURONS.to_le_bytes())?;

    // 2. Write number of synapses (u64)
    file.write_all(&NUM_SYNAPSES.to_le_bytes())?;

    // 3. Write synapse data
    for _ in 0..NUM_SYNAPSES {
        let source_index: u32 = rng.gen_range(0..NUM_NEURONS as u32);
        let target_index: u32 = rng.gen_range(0..NUM_NEURONS as u32);
        let weight: f32 = rng.gen_range(-1.0..1.0);

        file.write_all(&source_index.to_le_bytes())?;
        file.write_all(&target_index.to_le_bytes())?;
        file.write_all(&weight.to_le_bytes())?;
    }

    println!("Successfully generated {}", output_file.display());

    Ok(())
}
