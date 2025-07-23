// src/launcher/discover.rs
// Find PATH apps or Desktop files

use walkdir::WalkDir;
use freedesktop_desktop_entry::{default_paths, Iter};
use std::path::PathBuf;
use std::env;

use crate::appentry::AppEntry;
use creat::appentry::SourceKind;

pub fn get_desktop_entry() -> Vec<DesktopEntry> {

    let mut entries_vec: Vec<DesktopEntry> = Vec::new();
    let desktop_paths = default_paths;
    
    for path in desktop_paths {
        for entry in WalkDir::new(path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file() && e.file_name().to_string_lossy().ends_with(".desktop"))
        {
            let path = entry.path;

        }
    }


    
}