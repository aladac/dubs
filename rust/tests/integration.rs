use dubs::{Dubs, DubsError, PatternName, Theme, TokenCase, TokenType};

#[test]
fn generate_default() {
    let name = dubs::generate(None).unwrap();
    assert!(!name.is_empty());
    assert!(name.contains('-'));
}

#[test]
fn generate_with_theme() {
    let name = dubs::generate(Some("gundam")).unwrap();
    assert!(!name.is_empty());
}

#[test]
fn generate_with_underscore_theme() {
    let name = dubs::generate(Some("star_trek")).unwrap();
    assert!(!name.is_empty());
}

#[test]
fn generate_unknown_theme() {
    let result = dubs::generate(Some("nonexistent"));
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), DubsError::ThemeNotFound(_)));
}

#[test]
fn list_themes() {
    let themes = dubs::themes();
    assert!(themes.contains(&"gundam"));
    assert!(themes.contains(&"star-trek"));
}

#[test]
fn builder_hex_token() {
    let theme = Theme::by_name("gundam").unwrap();
    let name = Dubs::new(theme)
        .token_type(TokenType::Hex)
        .token_length(6)
        .generate();

    let token = name.rsplit('-').next().unwrap();
    assert_eq!(token.len(), 6);
    assert!(token.chars().all(|c| c.is_ascii_hexdigit()));
}

#[test]
fn builder_uppercase_hex() {
    let theme = Theme::by_name("gundam").unwrap();
    let name = Dubs::new(theme)
        .token_type(TokenType::Hex)
        .token_length(6)
        .token_case(TokenCase::Upper)
        .generate();

    let token = name.rsplit('-').next().unwrap();
    assert!(token.chars().all(|c| c.is_ascii_uppercase() || c.is_ascii_digit()));
}

#[test]
fn builder_no_token() {
    let theme = Theme::by_name("gundam").unwrap();
    let name = Dubs::new(theme)
        .token_type(TokenType::None)
        .pattern(PatternName::Codename)
        .generate();

    let parts: Vec<&str> = name.split('-').collect();
    assert_eq!(parts.len(), 2);
}

#[test]
fn builder_designation_pattern() {
    let theme = Theme::by_name("star-trek").unwrap();
    let name = Dubs::new(theme)
        .pattern(PatternName::Designation)
        .token_type(TokenType::Hex)
        .token_length(6)
        .generate();

    assert!(name.contains("-class-"));
}

#[test]
fn builder_custom_separator() {
    let theme = Theme::by_name("gundam").unwrap();
    let name = Dubs::new(theme)
        .separator(".")
        .token_type(TokenType::None)
        .pattern(PatternName::Codename)
        .generate();

    assert!(name.contains('.'));
    assert!(!name.contains('-'));
}

#[test]
fn seeded_determinism() {
    let a = Dubs::new(Theme::by_name("gundam").unwrap())
        .seed(42)
        .generate();
    let b = Dubs::new(Theme::by_name("gundam").unwrap())
        .seed(42)
        .generate();
    assert_eq!(a, b);
}

#[test]
fn different_seeds_different_output() {
    let a = Dubs::new(Theme::by_name("gundam").unwrap())
        .seed(42)
        .generate();
    let b = Dubs::new(Theme::by_name("gundam").unwrap())
        .seed(99)
        .generate();
    assert_ne!(a, b);
}

#[test]
fn combat_adjectives() {
    let name = Dubs::new(Theme::by_name("gundam").unwrap())
        .adjective_group("combat")
        .seed(42)
        .generate();

    assert!(!name.is_empty());
}

#[test]
fn specific_category() {
    let name = Dubs::new(Theme::by_name("gundam").unwrap())
        .category("characters")
        .seed(42)
        .generate();

    assert!(!name.is_empty());
}

#[test]
fn theme_categories() {
    let theme = Theme::by_name("gundam").unwrap();
    let cats = theme.categories();
    assert!(cats.contains(&"mobile_suits"));
    assert!(cats.contains(&"characters"));
    assert!(cats.contains(&"ships"));
}

#[test]
fn theme_nouns() {
    let theme = Theme::by_name("gundam").unwrap();
    let nouns = theme.nouns(Some("mobile_suits")).unwrap();
    assert!(nouns.contains(&"sazabi".to_string()));
    assert!(nouns.contains(&"zaku".to_string()));
}
