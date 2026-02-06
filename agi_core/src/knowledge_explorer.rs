use std::fs;
use std::io;
use std::path::Path;


/// Explores and processes large volumes of text data for the AGI.
/// This module is designed to read knowledge from files, break it down into
/// manageable concepts (e.g., sentences), and prepare it for holographic encoding,
/// avoiding massive storage like traditional LLMs.
#[derive(Debug, Default)]
pub struct KnowledgeExplorer {
    pub concepts: Vec<String>,
}

impl KnowledgeExplorer {
    /// Creates a new, empty KnowledgeExplorer.
    pub fn new() -> Self {
        Self::default()
    }

    /// Loads a text file and processes it into a list of concepts (sentences).
    /// This is the first step in the non-traditional learning pipeline.
    pub fn load_and_process_file<P: AsRef<Path>>(&mut self, path: P) -> io::Result<()> {
        let file = fs::File::open(path)?;
        let reader = io::BufReader::new(file);
        use io::BufRead;

        println!("--- KnowledgeExplorer: Processing file line-by-line... ---");

        // Process the file line by line to handle massive files without high memory usage.
        // We assume one concept per line for this scalable approach.
        self.concepts = reader.lines()
            .filter_map(io::Result::ok)
            .map(|line| line.trim().to_string())
            .filter(|line| !line.is_empty())
            .collect();

        println!("   -> Extracted {} concepts.", self.concepts.len());
        Ok(())
    }

    /// Returns a clone of the concepts discovered by the explorer.
    pub fn get_discovered_concepts(&self) -> Vec<String> {
        self.concepts.clone()
    }

    /// Clears the list of discovered concepts.
    /// This is called after the concepts have been assimilated by the core.
    pub fn clear_discovered_concepts(&mut self) {
        self.concepts.clear();
    }
}
