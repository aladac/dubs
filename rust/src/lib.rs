//! # dubs
//!
//! Themed name generator — like haikunator, but with categories.
//!
//! ```
//! let name = dubs::generate(None).unwrap();
//! println!("{}", name); // e.g. "vicious-sazabi-4271"
//!
//! let name = dubs::Dubs::new(dubs::Theme::by_name("gundam").unwrap())
//!     .token_type(dubs::TokenType::Hex)
//!     .token_length(6)
//!     .generate();
//! println!("{}", name); // e.g. "crimson-exia-b7e2a1"
//! ```

mod data;
mod generator;
mod pattern;
mod theme;
mod token;

pub use generator::Dubs;
pub use pattern::PatternName;
pub use theme::Theme;
pub use token::{TokenCase, TokenType};

use std::fmt;

/// Generate a random name with default settings.
///
/// Pass `None` for defaults, or `Some("theme_name")` to pick a theme.
pub fn generate(theme_name: Option<&str>) -> Result<String, DubsError> {
    match theme_name {
        Some(name) => {
            let theme =
                Theme::by_name(name).ok_or_else(|| DubsError::ThemeNotFound(name.to_string()))?;
            Ok(Dubs::new(theme).generate())
        }
        None => {
            let theme = Theme::random();
            Ok(Dubs::new(theme).generate())
        }
    }
}

/// List all available theme names.
pub fn themes() -> Vec<&'static str> {
    data::THEME_REGISTRY.keys().copied().collect()
}

#[derive(Debug)]
pub enum DubsError {
    ThemeNotFound(String),
    CategoryNotFound(String),
}

impl fmt::Display for DubsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DubsError::ThemeNotFound(name) => write!(f, "Theme not found: {name}"),
            DubsError::CategoryNotFound(name) => write!(f, "Category not found: {name}"),
        }
    }
}

impl std::error::Error for DubsError {}
