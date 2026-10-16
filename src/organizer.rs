use crate::config::{classify_extension, is_temporary_download, Config};
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

pub fn resolve_target_path(dest_dir: &Path, file_name: &str) -> PathBuf {
    let target = dest_dir.join(file_name);
    if !target.exists() {
        return target;
    }

    let original_path = Path::new(file_name);
    let stem = original_path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or(file_name);
    let ext = original_path.extension().and_then(|e| e.to_str());

    let mut counter = 1;
    loop {
        let new_file_name = match ext {
            Some(e) => format!("{stem}_{counter}.{e}"),
            None => format!("{stem}_{counter}"),
        };

        let candidate = dest_dir.join(&new_file_name);
        if !candidate.exists() {
            return candidate;
        }
        counter += 1;
    }
}

pub fn safe_move(source: &Path, target: &Path, dry_run: bool) -> Result<(), io::Error> {
    if dry_run {
        return Ok(());
    }

    if let Some(parent) = target.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent)?;
        }
    }

    if let Err(rename_err) = fs::rename(source, target) {
        // Fallback para cópia e remoção caso a origem e o destino estejam em pontos de montagem diferentes (EXDEV).
        if rename_err.kind() == io::ErrorKind::CrossesDevices || rename_err.raw_os_error() == Some(18)
        {
            fs::copy(source, target)?;
            fs::remove_file(source)?;
        } else {
            return Err(rename_err);
        }
    }

    Ok(())
}

pub fn organize_directory(config: &Config) -> Result<usize, io::Error> {
    if !config.source_dir.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!(
                "Diretório de origem não encontrado: {}",
                config.source_dir.display()
            ),
        ));
    }

    let entries = fs::read_dir(&config.source_dir)?;
    let mut moved_count = 0;

    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            continue;
        }

        let file_name = match path.file_name().and_then(|n| n.to_str()) {
            Some(name) => {
                if name.starts_with('.') {
                    continue;
                }
                name
            }
            None => continue,
        };

        let extension = match path.extension().and_then(|e| e.to_str()) {
            Some(ext) => ext,
            None => continue,
        };

        if is_temporary_download(extension) {
            continue;
        }

        if let Some(category) = classify_extension(extension) {
            let dest_dir = config.destination_for(category);
            let target_path = resolve_target_path(dest_dir, file_name);

            let prefix = if config.dry_run {
                "[SIMULACAO]"
            } else {
                "[ORGANIZADO]"
            };

            println!(
                "{} [{}] {} -> {}",
                prefix,
                category.name(),
                path.file_name().unwrap_or_default().to_string_lossy(),
                target_path.display()
            );

            safe_move(&path, &target_path, config.dry_run)?;
            moved_count += 1;
        }
    }

    Ok(moved_count)
}
