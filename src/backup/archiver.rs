use crate::config::SagerConfig;
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use zip::write::SimpleFileOptions;
use zip::ZipWriter;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupManifest {
    pub game_name: String,
    pub original_path: String,
    pub backup_timestamp: i64,
    pub backup_date_str: String,
    pub total_files: usize,
    pub total_bytes: u64,
    pub sager_version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupInfo {
    pub file_name: String,
    pub full_path: PathBuf,
    pub size_bytes: u64,
    pub manifest: Option<BackupManifest>,
}

pub struct SaveArchiver;

impl SaveArchiver {
    pub fn sanitize_filename(name: &str) -> String {
        name.chars()
            .map(|c| match c {
                '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|' => '_',
                _ => c,
            })
            .collect()
    }

    pub fn backup_game(
        game_name: &str,
        source_path: &Path,
        config: &SagerConfig,
    ) -> Result<BackupInfo, String> {
        if !source_path.exists() {
            return Err(format!("Source path does not exist: {}", source_path.display()));
        }

        let clean_name = Self::sanitize_filename(game_name);
        let game_backup_dir = config.backup_directory.join(&clean_name);
        fs::create_dir_all(&game_backup_dir)
            .map_err(|e| format!("Failed to create backup directory: {}", e))?;

        let now = chrono::Local::now();
        let timestamp_str = now.format("%Y%m%d_%H%M%S").to_string();
        let zip_filename = format!("{}_{}.zip", clean_name, timestamp_str);
        let zip_path = game_backup_dir.join(&zip_filename);

        let zip_file = File::create(&zip_path)
            .map_err(|e| format!("Failed to create zip file: {}", e))?;
        let mut zip = ZipWriter::new(zip_file);
        let options = SimpleFileOptions::default()
            .compression_method(zip::CompressionMethod::Deflated);

        let mut total_files = 0;
        let mut total_bytes = 0;

        for entry in WalkDir::new(source_path).into_iter().flatten() {
            let path = entry.path();
            if path.is_file() {
                if let Ok(rel_path) = path.strip_prefix(source_path) {
                    let rel_str = rel_path.to_string_lossy().replace('\\', "/");
                    
                    zip.start_file(&rel_str, options)
                        .map_err(|e| format!("Failed to add file to zip: {}", e))?;
                    
                    let mut file = File::open(path)
                        .map_err(|e| format!("Failed to open file: {}", e))?;
                    let bytes = std::io::copy(&mut file, &mut zip)
                        .map_err(|e| format!("Failed to write file to zip: {}", e))?;
                    
                    total_bytes += bytes;
                    total_files += 1;
                }
            }
        }

        // Add manifest inside the zip
        let manifest = BackupManifest {
            game_name: game_name.to_string(),
            original_path: source_path.to_string_lossy().to_string(),
            backup_timestamp: now.timestamp(),
            backup_date_str: now.format("%Y-%m-%d %H:%M:%S").to_string(),
            total_files,
            total_bytes,
            sager_version: "0.1.0".to_string(),
        };

        let manifest_json = serde_json::to_string_pretty(&manifest)
            .map_err(|e| format!("Failed to serialize manifest: {}", e))?;

        zip.start_file("sager_manifest.json", options)
            .map_err(|e| format!("Failed to add manifest to zip: {}", e))?;
        zip.write_all(manifest_json.as_bytes())
            .map_err(|e| format!("Failed to write manifest: {}", e))?;

        zip.finish()
            .map_err(|e| format!("Failed to finalize zip: {}", e))?;

        // Save manifest alongside zip as .json for quick inspection
        let json_path = game_backup_dir.join(format!("{}_{}.json", clean_name, timestamp_str));
        let _ = fs::write(&json_path, &manifest_json);

        // Update latest.json
        let latest_path = game_backup_dir.join("latest.json");
        let _ = fs::write(&latest_path, &manifest_json);

        let size_bytes = fs::metadata(&zip_path).map(|m| m.len()).unwrap_or(total_bytes);

        Ok(BackupInfo {
            file_name: zip_filename,
            full_path: zip_path,
            size_bytes,
            manifest: Some(manifest),
        })
    }

    pub fn backup_all(config: &SagerConfig) -> Vec<Result<BackupInfo, String>> {
        use super::scanner::SaveScanner;
        let games = SaveScanner::scan_all(config);
        let mut results = Vec::new();

        for game in games {
            let res = Self::backup_game(&game.name, &game.source_path, config);
            results.push(res);
        }

        results
    }
}
