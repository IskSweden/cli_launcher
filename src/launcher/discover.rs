// src/launcher/discover.rs
// Find PATH apps or Desktop files

use freedesktop_desktop_entry::{Iter, default_paths};
use std::env;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::path::PathBuf;
use walkdir::WalkDir;

use crate::launcher::appentry::AppEntry;
use crate::launcher::appentry::SourceKind;

pub fn get_path_bins() -> Vec<String> {
    let mut path_bins = Vec::new();

    if let Some(path_env) = env::var_os("PATH") {
        let paths = env::split_paths(&path_env);
        for path_buf in paths {
            if let Some(path_str) = path_buf.to_str() {
                path_bins.push(path_str.to_string());
            } else {
                eprintln!(
                    "Warning: Could not convert path {path_buf:?} to a valid UTF-8 string. Skipping."
                );
            }
        }
    } else {
        eprintln!("Error: PATH environment variable not found.");
    }

    path_bins
}

pub fn iter_path_bins() -> Vec<AppEntry> {
    let mut entries = Vec::new();

    let paths = get_path_bins();

    for path_str in paths {
        let path = PathBuf::from(path_str);
        if let Ok(dir_entries) = fs::read_dir(&path) {
            for entry_result in dir_entries {
                if let Ok(entry) = entry_result {
                    let path = entry.path();
                    if is_executable(&path) {
                        let name = entry.file_name().to_string_lossy().to_string();
                        let exec = path.clone();
                        let entry = AppEntry {
                            name,
                            exec,
                            source: SourceKind::PathBin,
                        };
                        entries.push(entry)
                    }
                }
            }
        }
    }
    entries
}

pub fn discover_desktop_entries() -> Vec<AppEntry> {
    let mut results = Vec::new();
    let entries = Iter::new(default_paths());

    for entry in entries {
        for path in WalkDir::new(entry) {
            match path {
                Ok(desktopapp) => {
                    let path = desktopapp.path();
                    if path.extension().and_then(|ext| ext.to_str()) == Some("desktop") {
                        let name = desktopapp.file_name().to_string_lossy().to_string();
                        let exec = desktopapp.path().to_path_buf();
                        let entry = AppEntry {
                            name,
                            exec,
                            source: SourceKind::DesktopFile,
                        };
                        results.push(entry)
                    }
                }
                Err(_e) => {}
            }
        }
    }
    results
}

fn is_executable(path: &Path) -> bool {
    if !path.is_file() {
        return false;
    }

    if let Ok(metadata) = fs::symlink_metadata(path) {
        let permissions = metadata.permissions();
        permissions.mode() & 0o111 != 0
    } else {
        false
    }
}
