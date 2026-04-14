use serde::Deserialize;
use std::collections::HashMap;
use std::sync::LazyLock;

#[derive(Debug, Deserialize)]
pub struct ThemeData {
    pub name: String,
    pub display_name: String,
    pub categories: HashMap<String, Vec<String>>,
    pub default_category: String,
}

#[derive(Debug, Deserialize)]
pub struct AdjectiveData {
    pub general: Vec<String>,
    pub combat: Vec<String>,
    pub cosmic: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct PatternData {
    pub patterns: HashMap<String, String>,
    pub default_pattern: String,
}

pub static ADJECTIVES: LazyLock<AdjectiveData> = LazyLock::new(|| {
    serde_json::from_str(include_str!("../../data/adjectives.json")).expect("Failed to parse adjectives.json")
});

pub static PATTERNS: LazyLock<PatternData> = LazyLock::new(|| {
    serde_json::from_str(include_str!("../../data/patterns.json")).expect("Failed to parse patterns.json")
});

pub static THEME_REGISTRY: LazyLock<HashMap<&'static str, ThemeData>> = LazyLock::new(|| {
    let mut map = HashMap::new();

    let gundam: ThemeData =
        serde_json::from_str(include_str!("../../data/themes/gundam.json")).expect("Failed to parse gundam.json");
    map.insert("gundam", gundam);

    let star_trek: ThemeData =
        serde_json::from_str(include_str!("../../data/themes/star-trek.json")).expect("Failed to parse star-trek.json");
    map.insert("star-trek", star_trek);

    map
});

impl AdjectiveData {
    pub fn group(&self, name: &str) -> Option<&[String]> {
        match name {
            "general" => Some(&self.general),
            "combat" => Some(&self.combat),
            "cosmic" => Some(&self.cosmic),
            _ => None,
        }
    }
}
