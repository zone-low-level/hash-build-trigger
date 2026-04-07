// src/main.rs
use anyhow::{Context, Result};
use clap::Parser;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use walkdir::WalkDir;

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about = "Hash files in directories (by extension) and rebuild if changed"
)]
struct Args {
    /// Directories to scan (can be repeated)
    #[arg(short = 'd', long, required = true, num_args = 1..)]
    dirs: Vec<String>,

    /// File extensions to include (e.g. rs toml json). If omitted, all files are included.
    #[arg(short = 'e', long)]
    extensions: Vec<String>,

    /// Build command to run if hash changed (default: "zig build")
    #[arg(short = 'b', long, default_value = "zig build")]
    build_cmd: String,

    /// Cache file that stores the last hash
    #[arg(long, default_value = "zig-out/.last-source-hash")]
    cache_file: String,
}

fn main() -> Result<()> {
    let args = Args::parse();

    // Compute current hash of all matching files
    let current_hash = compute_combined_hash(&args.dirs, &args.extensions)
        .context("Failed to compute hash of source files")?;

    // Load previous hash (if exists)
    let previous_hash = fs::read_to_string(&args.cache_file).ok();

    if previous_hash.as_deref() == Some(&current_hash) {
        println!("No changes detected in source files. Skipping build.");
        return Ok(());
    }

    println!("🔄 Source files changed (or first run). Running build...");

    // Run the build command
    // let status = Command::new("sh")
    //     .arg("-c")
    //     .arg(&args.build_cmd)
    //     .status()
    //     .context("Failed to execute build command")?;

    let status = Command::new("zig")
        .arg("build")
        .status()
        .context("Failed to execute")?;
    if !status.success() {
        anyhow::bail!("Build command failed with exit code: {}", status);
    }

    // Save the new hash
    fs::write(&args.cache_file, &current_hash).context("Failed to write hash cache file")?;

    println!("✅ Build completed. New hash cached.");
    Ok(())
}

/// Computes a deterministic BLAKE3 hash of all matching files.
/// Files are processed in sorted path order for reproducibility.
fn compute_combined_hash(dirs: &[String], extensions: &[String]) -> Result<String> {
    let mut hasher = blake3::Hasher::new();

    let mut files: Vec<PathBuf> = Vec::new();

    for dir in dirs {
        for entry in WalkDir::new(dir)
            .follow_links(false)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if entry.file_type().is_file() {
                let path = entry.path();

                // Filter by extension if any were provided
                if !extensions.is_empty() {
                    let ext = path
                        .extension()
                        .and_then(|e| e.to_str())
                        .unwrap_or("")
                        .to_lowercase();

                    if !extensions.iter().any(|e| e.to_lowercase() == ext) {
                        continue;
                    }
                }

                files.push(path.to_path_buf());
            }
        }
    }

    // Sort for deterministic ordering
    files.sort();

    for path in files {
        // Hash the relative path (makes hash sensitive to file location)
        let rel_path = path
            .strip_prefix(std::env::current_dir()?)
            .unwrap_or(&path)
            .to_string_lossy();
        hasher.update(rel_path.as_bytes());
        hasher.update(b"\0"); // separator

        // Hash the file content
        let content =
            fs::read(&path).with_context(|| format!("Failed to read file: {}", path.display()))?;
        hasher.update(&content);
    }

    Ok(hasher.finalize().to_hex().to_string())
}
