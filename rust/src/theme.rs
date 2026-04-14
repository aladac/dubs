use crate::data::{THEME_REGISTRY, ThemeData};
use crate::DubsError;
use rand::prelude::IndexedRandom;

/// A reference to a loaded theme.
pub struct Theme {
    pub(crate) data: &'static ThemeData,
}

impl Theme {
    /// Look up a theme by name (e.g. "gundam", "star-trek", "star_trek").
    pub fn by_name(name: &str) -> Option<Self> {
        let key = name.replace('_', "-");
        THEME_REGISTRY.get(key.as_str()).map(|data| Theme { data })
    }

    /// Pick a random theme.
    pub fn random() -> Self {
        let keys: Vec<_> = THEME_REGISTRY.keys().collect();
        let key = keys.choose(&mut rand::rng()).expect("No themes loaded");
        Theme {
            data: &THEME_REGISTRY[*key],
        }
    }

    /// Theme name.
    pub fn name(&self) -> &str {
        &self.data.name
    }

    /// Human-readable display name.
    pub fn display_name(&self) -> &str {
        &self.data.display_name
    }

    /// List category names.
    pub fn categories(&self) -> Vec<&str> {
        self.data.categories.keys().map(|s| s.as_str()).collect()
    }

    /// Get nouns for a category (or default category if None).
    pub fn nouns(&self, category: Option<&str>) -> Result<&[String], DubsError> {
        let cat = category.unwrap_or(&self.data.default_category);
        self.data
            .categories
            .get(cat)
            .map(|v| v.as_slice())
            .ok_or_else(|| DubsError::CategoryNotFound(cat.to_string()))
    }

    /// Pick a random noun from a category.
    pub fn random_noun(&self, category: Option<&str>) -> Result<&str, DubsError> {
        let nouns = self.nouns(category)?;
        Ok(nouns
            .choose(&mut rand::rng())
            .expect("Category has no nouns"))
    }
}
