use core::ThemeId;
use std::{collections::HashMap, path::Path};

use crate::metadata::ThemeMetadata;

/// Represents an index of themes, storing metadata for each theme.
#[derive(Debug, Default)]
pub struct ThemeIndex {
    metadata: HashMap<ThemeId, ThemeMetadata<'static>>,
}

impl ThemeIndex {
    /// Creates a new, empty `ThemeIndex`.
    pub fn new() -> Self {
        Self {
            metadata: HashMap::new(),
        }
    }

    /// Adds a theme to the index.
    pub fn add(&mut self, id: impl Into<ThemeId>, metadata: ThemeMetadata<'static>) {
        self.metadata.insert(id.into(), metadata);
    }

    pub fn get_path<'a>(&mut self, id: impl Into<&'a ThemeId>) -> Option<&Path> {
        if let Some(meta) = self.metadata.get(id.into()) {
            if let Some(path) = &meta.path {
                Some(path.as_ref())
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Returns an iterator over theme IDs and their corresponding paths.
    pub fn paths(&self) -> impl Iterator<Item = (&ThemeId, &Path)> {
        self.metadata
            .iter()
            .map(|(id, meta)| (id, meta.path.as_ref()))
            .filter_map(|(id, path)| path.map(|path| (id, path.as_ref())))
    }

    /// Returns an iterator over all theme IDs in the index.
    pub fn ids(&self) -> impl Iterator<Item = &ThemeId> {
        self.metadata.keys()
    }

    /// Loads themes from a directory, parsing metadata for each theme.
    pub async fn load_from_directory(path: impl AsRef<Path>) -> Result<Self, std::io::Error> {
        let path = path.as_ref();
        if path.is_dir() {
            let mut dirs = tokio::fs::read_dir(path).await?;
            let mut index = ThemeIndex::new();

            // Process each subdirectory
            while let Ok(Some(dir_entry)) = dirs.next_entry().await {
                if dir_entry.file_type().await?.is_dir() {
                    let metadata_file_path = {
                        let mut path = dir_entry.path();
                        path.push("metadata.toml");
                        path
                    };

                    // Parse metadata from TOML file
                    let metadata_content = tokio::fs::read_to_string(metadata_file_path).await?;
                    let metadata: ThemeMetadata =
                        toml::from_str(&metadata_content).map_err(|e| {
                            std::io::Error::new(std::io::ErrorKind::InvalidData, e.to_string())
                        })?;

                    index.add(metadata.id.clone(), metadata);
                }
            }

            Ok(index)
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Given path is not a directory",
            ))
        }
    }
}

#[cfg(test)]
mod test {
    use core::smol_str::SmolStr;
    use std::{borrow::Cow, path::Path};

    use crate::metadata::ThemeMetadata;

    use super::ThemeIndex;

    #[test]
    fn paths() {
        let mut index = ThemeIndex::new();

        index.add(
            "one",
            ThemeMetadata {
                id: Cow::Borrowed("one"),
                name: Cow::Borrowed("One"),
                path: Some(Cow::Borrowed(Path::new("./one/"))),
            },
        );

        index.add(
            "two",
            ThemeMetadata {
                id: Cow::Borrowed("two"),
                name: Cow::Borrowed("Two"),
                path: None,
            },
        );

        index.add(
            "three",
            ThemeMetadata {
                id: Cow::Borrowed("three"),
                name: Cow::Borrowed("Three"),
                path: Some(Cow::Borrowed(Path::new("./three/"))),
            },
        );

        let mut paths: Vec<_> = index.paths().collect();
        paths.sort_by(|(id, _path), (id2, _path2)| id.cmp(id2));

        assert_eq!(
            paths.as_slice(),
            [
                (&SmolStr::new("one"), Path::new("./one/")),
                (&SmolStr::new("three"), Path::new("./three/"))
            ]
        );
    }

    #[test]
    fn no_path() {
        let mut index = ThemeIndex::new();

        index.add(
            "one",
            ThemeMetadata {
                id: Cow::Borrowed("one"),
                name: Cow::Borrowed("One"),
                path: None,
            },
        );

        index.add(
            "two",
            ThemeMetadata {
                id: Cow::Borrowed("two"),
                name: Cow::Borrowed("Two"),
                path: None,
            },
        );

        index.add(
            "three",
            ThemeMetadata {
                id: Cow::Borrowed("three"),
                name: Cow::Borrowed("Three"),
                path: None,
            },
        );

        let paths: Vec<_> = index.paths().collect();

        assert_eq!(paths.as_slice(), []);
    }

    #[test]
    fn ids() {
        let mut index = ThemeIndex::new();

        index.add(
            "one",
            ThemeMetadata {
                id: Cow::Borrowed("one"),
                name: Cow::Borrowed("One"),
                path: Some(Cow::Borrowed(Path::new("./one/"))),
            },
        );

        index.add(
            "two",
            ThemeMetadata {
                id: Cow::Borrowed("two"),
                name: Cow::Borrowed("Two"),
                path: None,
            },
        );

        index.add(
            "three",
            ThemeMetadata {
                id: Cow::Borrowed("three"),
                name: Cow::Borrowed("Three"),
                path: Some(Cow::Borrowed(Path::new("./three/"))),
            },
        );

        let mut ids: Vec<_> = index.ids().collect();
        ids.sort();

        assert_eq!(
            ids.as_slice(),
            [
                &SmolStr::new("one"),
                &SmolStr::new("three"),
                &SmolStr::new("two"),
            ]
        );
    }
}
