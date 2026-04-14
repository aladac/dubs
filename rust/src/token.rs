use rand::Rng;

/// Token format type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenType {
    Hex,
    Numeric,
    Alpha,
    None,
}

/// Token letter case.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenCase {
    Lower,
    Upper,
}

const HEX_CHARS: &[u8] = b"0123456789abcdef";
const ALPHA_CHARS: &[u8] = b"abcdefghijklmnopqrstuvwxyz0123456789";

pub fn generate(
    token_type: TokenType,
    length: usize,
    case: TokenCase,
    seed: Option<u64>,
) -> String {
    if token_type == TokenType::None || length == 0 {
        return String::new();
    }

    let raw = match (token_type, seed) {
        (TokenType::Hex, Some(s)) => seeded_chars(HEX_CHARS, length, s),
        (TokenType::Hex, None) => random_chars(HEX_CHARS, length),
        (TokenType::Numeric, Some(s)) => {
            let val = s % 10u64.pow(length as u32);
            format!("{:0>width$}", val, width = length)
        }
        (TokenType::Numeric, None) => {
            let val = rand::rng().random_range(0..10u64.pow(length as u32));
            format!("{:0>width$}", val, width = length)
        }
        (TokenType::Alpha, Some(s)) => seeded_chars(ALPHA_CHARS, length, s),
        (TokenType::Alpha, None) => random_chars(ALPHA_CHARS, length),
        (TokenType::None, _) => unreachable!(),
    };

    match case {
        TokenCase::Upper => raw.to_uppercase(),
        TokenCase::Lower => raw,
    }
}

fn random_chars(charset: &[u8], length: usize) -> String {
    let mut rng = rand::rng();
    (0..length)
        .map(|_| charset[rng.random_range(0..charset.len())] as char)
        .collect()
}

fn seeded_chars(charset: &[u8], length: usize, seed: u64) -> String {
    (0..length)
        .map(|i| charset[((seed + i as u64 * 7) % charset.len() as u64) as usize] as char)
        .collect()
}
