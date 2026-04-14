use clap::Parser;
use dubs::{Dubs, PatternName, Theme, TokenCase, TokenType};

/// Themed name generator — like haikunator, but with categories.
#[derive(Parser)]
#[command(name = "dubs", version, about)]
struct Cli {
    /// Theme to use (e.g. gundam, star-trek, pokemon, leet)
    #[arg(short, long)]
    theme: Option<String>,

    /// Category within the theme
    #[arg(short, long)]
    category: Option<String>,

    /// Pattern: default, classified, designation, codename, serial
    #[arg(short, long, default_value = "default")]
    pattern: String,

    /// Token type: hex, numeric, alpha, none
    #[arg(long, default_value = "numeric")]
    token: String,

    /// Token length
    #[arg(short, long, default_value_t = 4)]
    length: usize,

    /// Uppercase tokens
    #[arg(short, long)]
    upper: bool,

    /// Separator between parts
    #[arg(short, long, default_value = "-")]
    separator: String,

    /// Adjective group: general, combat, cosmic
    #[arg(short, long, default_value = "general")]
    adjectives: String,

    /// Seed for deterministic output
    #[arg(long)]
    seed: Option<u64>,

    /// Number of names to generate
    #[arg(short, long, default_value_t = 1)]
    number: usize,

    /// List available themes
    #[arg(long)]
    list_themes: bool,

    /// Show categories for a theme
    #[arg(long)]
    list_categories: Option<String>,
}

fn parse_pattern(s: &str) -> PatternName {
    match s {
        "default" => PatternName::Default,
        "classified" => PatternName::Classified,
        "designation" => PatternName::Designation,
        "codename" => PatternName::Codename,
        "serial" => PatternName::Serial,
        _ => {
            eprintln!("Unknown pattern: {s}. Using default.");
            PatternName::Default
        }
    }
}

fn parse_token_type(s: &str) -> TokenType {
    match s {
        "hex" => TokenType::Hex,
        "numeric" => TokenType::Numeric,
        "alpha" => TokenType::Alpha,
        "none" => TokenType::None,
        _ => {
            eprintln!("Unknown token type: {s}. Using numeric.");
            TokenType::Numeric
        }
    }
}

fn main() {
    let cli = Cli::parse();

    if cli.list_themes {
        let mut themes = dubs::themes();
        themes.sort();
        for t in themes {
            println!("{t}");
        }
        return;
    }

    if let Some(ref name) = cli.list_categories {
        let key = name.replace('_', "-");
        match Theme::by_name(&key) {
            Some(theme) => {
                let mut cats = theme.categories();
                cats.sort();
                for c in cats {
                    let nouns = theme.nouns(Some(c)).unwrap_or_default();
                    println!("{c} ({} nouns)", nouns.len());
                }
            }
            None => {
                eprintln!("Theme not found: {name}");
                std::process::exit(1);
            }
        }
        return;
    }

    let token_case = if cli.upper {
        TokenCase::Upper
    } else {
        TokenCase::Lower
    };

    for _ in 0..cli.number {
        let theme = match &cli.theme {
            Some(name) => match Theme::by_name(name) {
                Some(t) => t,
                None => {
                    eprintln!("Theme not found: {name}. Use --list-themes to see available themes.");
                    std::process::exit(1);
                }
            },
            None => Theme::random(),
        };

        let mut builder = Dubs::new(theme)
            .pattern(parse_pattern(&cli.pattern))
            .token_type(parse_token_type(&cli.token))
            .token_length(cli.length)
            .token_case(token_case)
            .separator(&cli.separator)
            .adjective_group(&cli.adjectives);

        if let Some(ref cat) = cli.category {
            builder = builder.category(cat);
        }

        if let Some(seed) = cli.seed {
            builder = builder.seed(seed);
        }

        println!("{}", builder.generate());
    }
}
