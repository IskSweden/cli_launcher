use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct AppEntry {
    pub name: String,  // Display name
    pub exec: PathBuf, // What to run
    #[allow(dead_code)]
    pub source: SourceKind, // TODO PRINT LABEL
}

#[derive(Debug, Clone)]
pub enum SourceKind {
    PathBin,
    DesktopFile,
}
