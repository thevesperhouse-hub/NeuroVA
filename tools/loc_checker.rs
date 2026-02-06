use std::fs::{self, File};
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};

fn main() -> io::Result<()> {
    let current_dir = std::env::current_dir()?;
    let project_root = current_dir.parent().unwrap_or(&current_dir);
    println!("Scanning project in: {}\n", project_root.display());

    let mut rust_files = Vec::new();
    visit_dirs(project_root, &mut rust_files, "rs")?;

    println!("{} rust files detected:", rust_files.len());

    let mut results = Vec::new();
    for path in rust_files {
        if let Ok(lines) = count_lines(&path) {
            let relative_path = path.strip_prefix(project_root).unwrap_or(&path).to_path_buf();
            results.push((relative_path, lines));
        }
    }

    // Sort results alphabetically by path
    results.sort_by(|a, b| a.0.cmp(&b.0));

    for (path, lines) in results {
        println!("{} -> {} LoC", path.display(), lines);
    }

    Ok(())
}

fn visit_dirs(dir: &Path, files: &mut Vec<PathBuf>, extension: &str) -> io::Result<()> {
    if dir.is_dir() {
        // Ignore target and .git directories
        if dir.ends_with("target") || dir.ends_with(".git") {
            return Ok(());
        }

        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, files, extension)?;
            } else if let Some(ext) = path.extension() {
                if ext == extension {
                    files.push(path);
                }
            }
        }
    }
    Ok(())
}

fn count_lines(path: &Path) -> io::Result<usize> {
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);
    Ok(reader.lines().count())
}
