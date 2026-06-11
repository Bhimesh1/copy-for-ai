use anyhow::{bail, Context, Result};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct ConvertedDocument {
    pub source_path: PathBuf,
    pub file_name: String,
    pub file_extension: Option<String>,
    pub content: String,
}

pub fn convert_file(path: impl AsRef<Path>) -> Result<ConvertedDocument> {
    let path = path.as_ref();

    if !path.exists() {
        bail!("File does not exist: {}", path.display());
    }

    if !path.is_file() {
        bail!("Path is not a file: {}", path.display());
    }

    let extension = path
        .extension()
        .and_then(|value| value.to_str())
        .map(|value| value.to_lowercase());

    match extension.as_deref() {
        Some("txt") | Some("md") | Some("markdown") => convert_text_file(path),
        Some(other) => bail!("Unsupported file type for now: .{}", other),
        None => bail!("File has no extension: {}", path.display()),
    }
}

fn convert_text_file(path: &Path) -> Result<ConvertedDocument> {
    let content = fs::read_to_string(path)
        .with_context(|| format!("Failed to read file: {}", path.display()))?;

    let file_name = path
        .file_name()
        .and_then(|value| value.to_str())
        .unwrap_or("unknown")
        .to_string();

    let file_extension = path
        .extension()
        .and_then(|value| value.to_str())
        .map(|value| value.to_lowercase());

    Ok(ConvertedDocument {
        source_path: path.to_path_buf(),
        file_name,
        file_extension,
        content,
    })
}
