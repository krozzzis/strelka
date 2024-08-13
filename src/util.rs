use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
    sync::Arc,
    time::Duration,
};

use tokio::{fs, io::AsyncWriteExt};

use crate::theming::{metadata::ThemeMetadata, Theme};
use futures_core::stream::Stream;

pub async fn get_theme_metadatas<'a>(
    dir: impl AsRef<Path>,
) -> impl Stream<Item = ThemeMetadata<'a>> {
    let mut dir_entries = fs::read_dir(dir).await.unwrap();

    async_stream::stream! {
        while let Some(entry) = dir_entries.next_entry().await.unwrap() {
            let path = entry.path();
            if path.is_file() && path.extension() == Some(OsStr::new("toml")) {
                let theme = ThemeMetadata::from_file(&path.clone()).await;
                if let Ok(theme) = theme {
                    yield theme;
                }
            }
        }
    }
}

pub async fn save_file(path: PathBuf, text: Arc<String>) -> tokio::io::Result<()> {
    let mut file = fs::File::create(path).await?;

    file.write_all(text.as_bytes()).await?;

    file.flush().await?;
    Ok(())
}

pub async fn get_directory_content(dir: impl Into<PathBuf>) -> Vec<PathBuf> {
    let mut files = Vec::new();
    let dir_path = dir.into();

    let mut dir_entries = fs::read_dir(dir_path).await.unwrap();

    while let Some(entry) = dir_entries.next_entry().await.unwrap() {
        let path = entry.path();
        files.push(path);
    }

    files
}

pub async fn delay(secs: u64) {
    tokio::time::sleep(Duration::new(secs, 0)).await
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

pub async fn load_theme_from_file(path: impl Into<PathBuf>) -> Option<Theme> {
    let theme = Theme::from_file(path.into()).await;
    if let Ok(theme) = theme {
        Some(theme)
    } else {
        None
    }
}
