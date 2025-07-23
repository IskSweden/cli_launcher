struct AppEntry {
    name: String,       // Display name
    exec: String,       // What to run
    source: SourceKind, // PATH, Desktop
}

enum SourceKind {
    PathBin,
    DesktopFile,
}