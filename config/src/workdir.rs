use std::path::{Path, PathBuf};

pub fn create_workdir() -> Result<PathBuf, String> {
    let homedir = dirs::home_dir().ok_or(String::from("Can't get home directory"))?;

    // ~/strelka
    let workdir_path = {
        let mut a = homedir.clone();
        a.push("strelka");
        a
    };

    std::fs::create_dir_all(&workdir_path).map_err(|e| e.to_string())?;

    Ok(workdir_path)
}

pub fn create_config_dir(workdir: &Path) -> Result<PathBuf, String> {
    // ~/strelka/.config
    let config_dir_path = {
        let mut a = workdir.to_owned();
        a.push(".config");
        a
    };

    std::fs::create_dir_all(&config_dir_path).map_err(|e| e.to_string())?;

    Ok(config_dir_path)
}
