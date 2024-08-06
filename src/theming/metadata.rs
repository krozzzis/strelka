use std::{borrow::Cow, path::Path};

use serde::Deserialize;

use crate::theming::theme::Info;

#[derive(Debug, Clone)]
pub struct ThemeMetadata<'a> {
    pub info: Info<'a>,
    pub path: Cow<'a, Path>,
}

#[derive(Deserialize)]
struct PalettelessTheme<'a> {
    info: Info<'a>,
}

impl<'a> ThemeMetadata<'a> {
    pub async fn from_file(path: &'a Path) -> Result<ThemeMetadata<'static>, ()> {
        let text = tokio::fs::read_to_string(path).await.map_err(|_| ())?;
        let theme: PalettelessTheme = toml::from_str(&text).map_err(|_| ())?;
        let info = theme.info;

        Ok(ThemeMetadata {
            info: info.to_owned(),
            path: Cow::Owned(path.to_owned()),
        })
    }
}
