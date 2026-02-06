//! Module responsable du scan et de l'échantillonnage de sources de connaissances distantes ou locales.
//! Il ne télécharge jamais l'intégralité des données, mais en extrait une "signature informationnelle"
//! pour un apprentissage holographique efficace et sans encombrement.

//! Module responsable du scan et de l'échantillonnage de sources de connaissances distantes ou locales.
//! Il ne télécharge jamais l'intégralité des données, mais en extrait une "signature informationnelle"
//! pour un apprentissage holographique efficace et sans encombrement.

use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use reqwest::Client;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use thiserror::Error;

/// Définit les types de sources de données que le scanner peut traiter.
#[derive(Debug)]
pub enum DataSource {
    Http { url: String },
    LocalFile { path: String },
}

#[derive(Error, Debug)]
pub enum ScannerError {
    #[error("Erreur réseau ou HTTP: {0}")]
    Network(#[from] reqwest::Error),
    #[error("Erreur de lecture du fichier local: {0}")]
    Io(#[from] std::io::Error),
    #[error("La source de données est vide ou inaccessible.")]
    EmptySource,
    #[error("La taille de la source de données n'a pas pu être déterminée.")]
    UnknownSize,
}

/// Le scanner de connaissances.
pub struct KnowledgeScanner {
    client: Client,
}

impl KnowledgeScanner {
    pub fn new() -> Self {
        Self { client: Client::new() }
    }

    /// Scanne une source de données, en extrait des fragments et retourne une signature concaténée.
    pub async fn scan(
        &self,
        source: &DataSource,
        num_fragments: u32,
        fragment_size: u64,
    ) -> Result<String, ScannerError> {
        match source {
            DataSource::Http { url } => self.scan_http(url, num_fragments, fragment_size).await,
            DataSource::LocalFile { path } => self.scan_local(path, num_fragments, fragment_size),
        }
    }

    async fn scan_http(&self, url: &str, num_fragments: u32, fragment_size: u64) -> Result<String, ScannerError> {
        // 1. Envoyer une requête HEAD pour obtenir la taille totale du contenu.
        let head_res = self.client.head(url).send().await?;
        let total_size = head_res
            .headers()
            .get(reqwest::header::CONTENT_LENGTH)
            .and_then(|val| val.to_str().ok())
            .and_then(|s| s.parse::<u64>().ok())
            .ok_or(ScannerError::UnknownSize)?;

        if total_size <= fragment_size * num_fragments as u64 {
            // Si le fichier est trop petit, on le télécharge en entier.
            let text = self.client.get(url).send().await?.text().await?;
            return Ok(text);
        }

        // 2. Générer des positions de départ aléatoires et uniques.
        let mut signature = String::new();
        let mut rng = StdRng::from_entropy();
        for _ in 0..num_fragments {
            let max_pos = total_size - fragment_size;
            let random_pos = rng.gen_range(0..=max_pos);

            // 3. Envoyer une requête GET avec un en-tête Range.
            let range_header = format!("bytes={}-{}", random_pos, random_pos + fragment_size - 1);
            let fragment_res = self.client.get(url).header("Range", range_header).send().await?;
            let fragment_text = fragment_res.text().await?;
            signature.push_str(&fragment_text);
            signature.push_str("\n\n...\n\n"); // Séparateur pour marquer la discontinuité
        }

        Ok(signature)
    }

    fn scan_local(&self, path: &str, num_fragments: u32, fragment_size: u64) -> Result<String, ScannerError> {
        let mut file = File::open(path)?;
        let total_size = file.metadata()?.len();

        if total_size <= fragment_size * num_fragments as u64 {
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;
            return Ok(contents);
        }

        let mut signature = String::new();
        let mut rng = StdRng::from_entropy();
        let mut buffer = vec![0; fragment_size as usize];

        for _ in 0..num_fragments {
            let max_pos = total_size - fragment_size;
            let random_pos = rng.gen_range(0..=max_pos);

            file.seek(SeekFrom::Start(random_pos))?;
            let bytes_read = file.read(&mut buffer)?;
            
            // Tenter de convertir le fragment en UTF-8, en ignorant les erreurs.
            let fragment_text = String::from_utf8_lossy(&buffer[..bytes_read]);
            signature.push_str(&fragment_text);
            signature.push_str("\n\n...\n\n");
        }

        Ok(signature)
    }
}

impl Default for KnowledgeScanner {
    fn default() -> Self {
        Self::new()
    }
}
