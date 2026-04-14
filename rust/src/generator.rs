use crate::data::ADJECTIVES;
use crate::pattern::{self, PatternName};
use crate::theme::Theme;
use crate::token::{self, TokenCase, TokenType};

/// Builder for generating themed names.
///
/// ```
/// use dubs::{Dubs, Theme, TokenType, TokenCase, PatternName};
///
/// let name = Dubs::new(Theme::by_name("gundam").unwrap())
///     .token_type(TokenType::Hex)
///     .token_length(6)
///     .token_case(TokenCase::Upper)
///     .pattern(PatternName::Designation)
///     .generate();
///
/// assert!(name.contains("-class-"));
/// ```
pub struct Dubs {
    theme: Theme,
    category: Option<String>,
    pattern: PatternName,
    token_type: TokenType,
    token_length: usize,
    token_case: TokenCase,
    separator: String,
    adjective_group: String,
    seed: Option<u64>,
}

impl Dubs {
    /// Create a new generator with a theme.
    pub fn new(theme: Theme) -> Self {
        Self {
            theme,
            category: None,
            pattern: PatternName::Default,
            token_type: TokenType::Numeric,
            token_length: 4,
            token_case: TokenCase::Lower,
            separator: "-".to_string(),
            adjective_group: "general".to_string(),
            seed: None,
        }
    }

    pub fn category(mut self, category: &str) -> Self {
        self.category = Some(category.to_string());
        self
    }

    pub fn pattern(mut self, pattern: PatternName) -> Self {
        self.pattern = pattern;
        self
    }

    pub fn token_type(mut self, token_type: TokenType) -> Self {
        self.token_type = token_type;
        self
    }

    pub fn token_length(mut self, length: usize) -> Self {
        self.token_length = length;
        self
    }

    pub fn token_case(mut self, case: TokenCase) -> Self {
        self.token_case = case;
        self
    }

    pub fn separator(mut self, sep: &str) -> Self {
        self.separator = sep.to_string();
        self
    }

    pub fn adjective_group(mut self, group: &str) -> Self {
        self.adjective_group = group.to_string();
        self
    }

    pub fn seed(mut self, seed: u64) -> Self {
        self.seed = Some(seed);
        self
    }

    /// Generate the name.
    pub fn generate(&self) -> String {
        let adjective = self.pick_adjective();
        let noun = self.pick_noun();
        let tok = token::generate(self.token_type, self.token_length, self.token_case, self.seed);
        let template = self.pattern.template();

        pattern::interpolate(template, &adjective, &noun, &tok, &self.separator)
    }

    fn pick_adjective(&self) -> String {
        let adjectives = ADJECTIVES
            .group(&self.adjective_group)
            .unwrap_or(&ADJECTIVES.general);

        match self.seed {
            Some(s) => adjectives[(s as usize) % adjectives.len()].clone(),
            None => {
                use rand::prelude::IndexedRandom;
                adjectives
                    .choose(&mut rand::rng())
                    .cloned()
                    .unwrap_or_default()
            }
        }
    }

    fn pick_noun(&self) -> String {
        let cat = self.category.as_deref();
        let nouns = self.theme.nouns(cat).unwrap_or_default();

        match self.seed {
            Some(s) => {
                let adj_len = ADJECTIVES
                    .group(&self.adjective_group)
                    .map(|a| a.len())
                    .unwrap_or(1);
                nouns[((s as usize) / adj_len) % nouns.len()].clone()
            }
            None => {
                use rand::prelude::IndexedRandom;
                nouns
                    .choose(&mut rand::rng())
                    .cloned()
                    .unwrap_or_default()
            }
        }
    }
}
