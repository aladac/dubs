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

    let star_wars: ThemeData =
        serde_json::from_str(include_str!("../../data/themes/star-wars.json")).expect("Failed to parse star-wars.json");
    map.insert("star-wars", star_wars);

    let transformers: ThemeData =
        serde_json::from_str(include_str!("../../data/themes/transformers.json")).expect("Failed to parse transformers.json");
    map.insert("transformers", transformers);

    let warhammer: ThemeData =
        serde_json::from_str(include_str!("../../data/themes/warhammer-40k.json")).expect("Failed to parse warhammer-40k.json");
    map.insert("warhammer-40k", warhammer);

    let nato: ThemeData =
        serde_json::from_str(include_str!("../../data/themes/nato.json")).expect("Failed to parse nato.json");
    map.insert("nato", nato);

    let pokemon: ThemeData =
        serde_json::from_str(include_str!("../../data/themes/pokemon.json")).expect("Failed to parse pokemon.json");
    map.insert("pokemon", pokemon);

    let leet: ThemeData =
        serde_json::from_str(include_str!("../../data/themes/leet.json")).expect("Failed to parse leet.json");
    map.insert("leet", leet);

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
