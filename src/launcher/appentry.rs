use std::path::PathBuf;


#[derive(Debug, Clone)]
pub struct AppEntry {
    pub name: String,       // Display name
    pub exec: PathBuf,       // What to run
    pub source: SourceKind, // PATH, Desktop
}

#[derive(Debug, Clone)]
pub enum SourceKind {
    PathBin,
    DesktopFile,
}

impl AppEntry {
    // todo

}
