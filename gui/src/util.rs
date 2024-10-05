use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use tokio::{fs, io::AsyncWriteExt};

pub async fn init_workdir(dir: PathBuf) {
    let _ = fs::create_dir(dir).await;
}

pub async fn save_file(path: PathBuf, text: Arc<String>) -> tokio::io::Result<()> {
    let mut file = fs::File::create(path).await?;

    file.write_all(text.as_bytes()).await?;

    file.flush().await?;
    Ok(())
}

pub async fn open_file(path: impl Into<PathBuf>) -> Result<(PathBuf, String), ()> {
    let path = path.into();
    let content = fs::read_to_string(&path).await.map_err(|_| ())?;
    Ok((path, content))
}

pub async fn pick_file(directory: Option<PathBuf>) -> Result<(PathBuf, String), ()> {
    let handler = if let Some(dir) = directory {
        rfd::AsyncFileDialog::new().set_directory(dir)
    } else {
        rfd::AsyncFileDialog::new()
    }
    .pick_file()
    .await;

    if let Some(path) = handler {
        let content = open_file(path.path()).await.map_err(|_| ())?;
        Ok(content)
    } else {
        Err(())
    }
}

pub fn get_file_name(path: &Path) -> String {
    path.file_name()
        .and_then(|os_str| os_str.to_str())
        .unwrap_or("")
        .to_owned()
}
