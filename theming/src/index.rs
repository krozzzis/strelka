use std::{collections::HashMap, path::Path};

use crate::{catalog::ThemeID, metadata::ThemeMetadata};

#[derive(Debug, Default)]
pub struct ThemeIndex {
    metadata: HashMap<ThemeID, ThemeMetadata<'static>>,
}

impl ThemeIndex {
    pub fn new() -> Self {
        Self {
            metadata: HashMap::new(),
        }
    }

    pub fn add(&mut self, id: impl Into<ThemeID>, metadata: ThemeMetadata<'static>) {
        self.metadata.insert(id.into(), metadata);
    }

    pub fn paths(&self) -> impl Iterator<Item = (&ThemeID, &Path)> {
        self.metadata
            .iter()
            .map(|(id, meta)| (id, meta.path.as_ref()))
            .filter_map(|(id, path)| path.map(|path| (id, path.as_ref())))
    }
}

#[cfg(test)]
mod test {
    use core::smol_str::SmolStr;
    use std::{borrow::Cow, path::Path};

    use crate::{catalog::ThemeID, metadata::ThemeMetadata};

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

        let mut paths: Vec<_> = index.paths().collect();

        assert_eq!(paths.as_slice(), []);
    }
}
