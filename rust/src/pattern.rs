use crate::data::PATTERNS;

/// Named pattern presets.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PatternName {
    Default,
    Classified,
    Designation,
    Codename,
    Serial,
}

impl PatternName {
    pub fn template(&self) -> &str {
        let key = match self {
            PatternName::Default => "default",
            PatternName::Classified => "classified",
            PatternName::Designation => "designation",
            PatternName::Codename => "codename",
            PatternName::Serial => "serial",
        };
        PATTERNS.patterns.get(key).map(|s| s.as_str()).unwrap_or("{adjective}-{noun}-{token}")
    }
}

/// Interpolate a pattern template with values.
pub fn interpolate(template: &str, adjective: &str, noun: &str, token: &str, separator: &str) -> String {
    let mut result = template
        .replace("{adjective}", adjective)
        .replace("{noun}", noun)
        .replace("{token}", token);

    // Replace literal hyphens with custom separator
    if separator != "-" {
        result = result.replace('-', separator);
    }

    // Clean up trailing/leading separators if token is empty
    if token.is_empty() {
        let double = format!("{separator}{separator}");
        result = result
            .trim_end_matches(separator)
            .trim_start_matches(separator)
            .replace(&double, separator);
    }

    result
}
