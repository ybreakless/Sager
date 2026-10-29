use super::archiver::{BackupInfo, BackupManifest, SaveArchiver};
use crate::config::SagerConfig;
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::Read;
use std::path::{Path, PathBuf};
use zip::ZipArchive;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RestoreResult {
    pub game_name: String,
    pub target_path: PathBuf,
    pub restored_files: usize,
    pub safety_backup_path: Option<PathBuf>,
}

pub struct SaveRestore;

impl SaveRestore {
    pub fn list_backups_for_game(game_name: &str, config: &SagerConfig) -> Vec<BackupInfo> {
        let clean_name = SaveArchiver::sanitize_filename(game_name);
        let game_backup_dir = config.backup_directory.join(&clean_name);

        if !game_backup_dir.exists() || !game_backup_dir.is_dir() {
            return Vec::new();
        }

        let mut list = Vec::new();

        if let Ok(entries) = fs::read_dir(&game_backup_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() && path.extension().map_or(false, |e| e == "zip") {
                    let file_name = path.file_name().unwrap_or_default().to_string_lossy().to_string();
                    let size_bytes = fs::metadata(&path).map(|m| m.len()).unwrap_or(0);
                    let manifest = Self::read_manifest_from_zip(&path);

                    list.push(BackupInfo {
                        file_name,
                        full_path: path,
                        size_bytes,
                        manifest,
                    });
                }
            }
        }

        list.sort_by(|a, b| {
            let time_a = a.manifest.as_ref().map(|m| m.backup_timestamp).unwrap_or(0);
            let time_b = b.manifest.as_ref().map(|m| m.backup_timestamp).unwrap_or(0);
            time_b.cmp(&time_a) // newest first
        });

        list
    }

    pub fn list_all_backups(config: &SagerConfig) -> Vec<(String, Vec<BackupInfo>)> {
        let mut results = Vec::new();
        if !config.backup_directory.exists() {
            return results;
        }

        if let Ok(entries) = fs::read_dir(&config.backup_directory) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    let game_name = path.file_name().unwrap_or_default().to_string_lossy().to_string();
                    let backups = Self::list_backups_for_game(&game_name, config);
                    if !backups.is_empty() {
                        results.push((game_name, backups));
                    }
                }
            }
        }

        results.sort_by(|a, b| a.0.cmp(&b.0));
        results
    }

    pub fn read_manifest_from_zip(zip_path: &Path) -> Option<BackupManifest> {
        let file = File::open(zip_path).ok()?;
        let mut archive = ZipArchive::new(file).ok()?;

        if let Ok(mut manifest_file) = archive.by_name("sager_manifest.json") {
            let mut content = String::new();
            if manifest_file.read_to_string(&mut content).is_ok() {
                return serde_json::from_str(&content).ok();
            }
        }

        // Check alongside zip file (e.g. name_timestamp.json)
        let json_candidate = zip_path.with_extension("json");
        if json_candidate.exists() {
            if let Ok(content) = fs::read_to_string(&json_candidate) {
                return serde_json::from_str(&content).ok();
            }
        }

        None
    }

    pub fn restore_backup(
        zip_path: &Path,
        target_override: Option<&Path>,
        config: &SagerConfig,
    ) -> Result<RestoreResult, String> {
        if !zip_path.exists() {
            return Err(format!("Backup archive does not exist: {}", zip_path.display()));
        }

        let manifest = Self::read_manifest_from_zip(zip_path)
            .ok_or_else(|| "Could not read sager_manifest.json from backup archive".to_string())?;

        let target_path = if let Some(custom) = target_override {
            custom.to_path_buf()
        } else {
            PathBuf::from(&manifest.original_path)
        };

        // Safety backup of existing current save if it exists
        let mut safety_backup_path = None;
        if target_path.exists() {
            let safety_name = format!("{}_safety_before_restore", SaveArchiver::sanitize_filename(&manifest.game_name));
            if let Ok(safety) = SaveArchiver::backup_game(&safety_name, &target_path, config) {
                safety_backup_path = Some(safety.full_path);
            }
        }

        fs::create_dir_all(&target_path)
            .map_err(|e| format!("Failed to create restore target directory: {}", e))?;

        let file = File::open(zip_path)
            .map_err(|e| format!("Failed to open backup archive: {}", e))?;
        let mut archive = ZipArchive::new(file)
            .map_err(|e| format!("Failed to read zip archive: {}", e))?;

        let mut restored_files = 0;

        for i in 0..archive.len() {
            let mut file = archive.by_index(i)
                .map_err(|e| format!("Failed to read zip entry #{}: {}", i, e))?;
            
            let file_name = file.name().to_string();
            // Skip the metadata manifest
            if file_name == "sager_manifest.json" {
                continue;
            }

            let outpath = match file.enclosed_name() {
                Some(path) => target_path.join(path),
                None => continue,
            };

            if file.is_dir() {
                fs::create_dir_all(&outpath)
                    .map_err(|e| format!("Failed to create folder {:?}: {}", outpath, e))?;
            } else {
                if let Some(parent) = outpath.parent() {
                    if !parent.exists() {
                        fs::create_dir_all(parent)
                            .map_err(|e| format!("Failed to create folder {:?}: {}", parent, e))?;
                    }
                }
                let mut outfile = File::create(&outpath)
                    .map_err(|e| format!("Failed to create file {:?}: {}", outpath, e))?;
                std::io::copy(&mut file, &mut outfile)
                    .map_err(|e| format!("Failed to extract file {:?}: {}", outpath, e))?;
                restored_files += 1;
            }
        }

        Ok(RestoreResult {
            game_name: manifest.game_name,
            target_path,
            restored_files,
            safety_backup_path,
        })
    }
}
