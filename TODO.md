# TODO.md — dubs

## Phase 1: Data Foundation
| Est (coop) | Task |
|------------|------|
| 15 min | Create `data/schema.json` — JSON Schema for theme validation |
| 15 min | Create `data/adjectives.json` — general, combat, cosmic groups (~50 words) |
| 10 min | Create `data/patterns.json` — 5 built-in patterns |
| 20 min | Create `data/themes/gundam.json` — mobile_suits, characters, ships |
| 20 min | Create `data/themes/star-trek.json` — ship_classes, species, characters |
| 5 min  | Validate all JSON against schema |
| **~1.5h** | **Phase 1 total — 1 session** |

## Phase 2: Ruby Gem (primary implementation)
| Est (coop) | Task |
|------------|------|
| 10 min | Scaffold gem: gemspec, Gemfile, lib structure, Rakefile |
| 15 min | `Dubs::Data` — load JSON data, theme registry |
| 15 min | `Dubs::Theme` — theme object, categories, introspection |
| 15 min | `Dubs::Token` — hex, numeric, alpha, none generators |
| 15 min | `Dubs::Pattern` — template parsing, interpolation |
| 20 min | `Dubs::Generator` — main generate method, options handling |
| 10 min | `Dubs` module — top-level API (`Dubs.generate`, `Dubs.themes`) |
| 10 min | Custom theme registration (`register_theme`, `register_theme_file`) |
| 30 min | RSpec tests — full coverage |
| 10 min | Publish to RubyGems |
| **~2.5h** | **Phase 2 total — 1 session** |

## Phase 3: Rust Crate
| Est (coop) | Task |
|------------|------|
| 10 min | Scaffold: Cargo.toml, lib.rs, modules |
| 15 min | Data embedding — `include_str!`, serde deserialization |
| 15 min | Theme/Pattern/Token types + enums |
| 20 min | Generator — builder pattern, generate() |
| 20 min | Tests |
| 10 min | Publish to crates.io |
| **~1.5h** | **Phase 3 total — 1 session** |

## Phase 4: TypeScript Package
| Est (coop) | Task |
|------------|------|
| 10 min | Scaffold: package.json, tsconfig, src structure |
| 30 min | Generator, types, data loading |
| 20 min | Tests (vitest) |
| 10 min | Publish to npm |
| **~1h** | **Phase 4 total — 1 session** |

## Phase 5: Python Package
| Est (coop) | Task |
|------------|------|
| 10 min | Scaffold: pyproject.toml, src structure |
| 30 min | Generator, data loading, API |
| 20 min | Tests (pytest) |
| 10 min | Publish to PyPI |
| **~1h** | **Phase 5 total — 1 session** |

## Phase 6: Remaining Themes
| Est (coop) | Task |
|------------|------|
| 20 min | `data/themes/star-wars.json` |
| 20 min | `data/themes/transformers.json` |
| 20 min | `data/themes/warhammer-40k.json` |
| 15 min | `data/themes/nato.json` |
| **~1.25h** | **Phase 6 total — 1 session** |

## Phase 7: Cross-language Tests & CI
| Est (coop) | Task |
|------------|------|
| 20 min | `tests/fixtures/seeded-expectations.json` — deterministic test vectors |
| 15 min | Cross-language test script |
| 20 min | GitHub Actions CI — test matrix for all 4 languages |
| 15 min | README.md with examples for all languages |
| 10 min | CLAUDE.md |
| **~1.25h** | **Phase 7 total — 1 session** |

## Summary

| Phase | Coop Est | Sessions |
|-------|----------|----------|
| 1. Data Foundation | 1.5h | 1 |
| 2. Ruby Gem | 2.5h | 1 |
| 3. Rust Crate | 1.5h | 1 |
| 4. TypeScript | 1h | 1 |
| 5. Python | 1h | 1 |
| 6. More Themes | 1.25h | 1 |
| 7. Tests & CI | 1.25h | 1 |
| **Total** | **~10h** | **4-5 sessions** |
