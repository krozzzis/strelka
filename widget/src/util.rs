use std::path::PathBuf;

pub fn filename(path: PathBuf) -> Option<String> {
    // Remove .md
    if let Some(Some("md")) = path.extension().map(|x| x.to_str()) {
        if let Some(filename) = path.file_stem() {
            let filename = filename.to_string_lossy().to_string();
            return Some(filename);
        }
    } else if let Some(filename) = path.file_name() {
        let filename = filename.to_string_lossy().to_string();
        return Some(filename);
    }
    None
}
