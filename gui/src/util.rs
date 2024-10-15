use std::path::Path;

pub fn get_file_name(path: &Path) -> String {
    path.file_name()
        .and_then(|os_str| os_str.to_str())
        .unwrap_or("")
        .to_owned()
}
