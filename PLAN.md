# PLAN.md — dubs

> Themed name generator. Like haikunator, but with categories.
> `"vicious-sazabi-a3f1"` · `"burning-akira-class-EFFB1A"` · `"shining-deathstar-AAFF22"`

## Overview

Four-language library (Ruby, Python, TypeScript, Rust) that generates random names from themed word lists with configurable patterns and token formats. Monorepo with shared JSON data files.

## Architecture

### Monorepo Structure

```
dubs/
├── data/                          # shared theme data (canonical source)
│   ├── schema.json                # JSON Schema for theme validation
│   ├── adjectives.json            # universal adjectives (grouped by flavor)
│   ├── patterns.json              # built-in pattern definitions
│   └── themes/
│       ├── gundam.json
│       ├── star-trek.json
│       ├── star-wars.json
│       ├── transformers.json
│       ├── warhammer-40k.json
│       └── nato.json
├── ruby/                          # gem: dubs
├── python/                        # package: dubs
├── typescript/                    # npm: @saiden/dubs
├── rust/                          # crate: dubs
├── tests/                         # cross-language consistency
│   ├── fixtures/seeded-expectations.json
│   └── cross-language-test.sh
├── justfile                       # orchestration
├── CLAUDE.md
└── README.md
```

### Data Files (JSON)

All languages read from the same JSON data files. Each package bundles a copy for distribution.

**Theme format** (`data/themes/gundam.json`):
```json
{
  "name": "gundam",
  "display_name": "Mobile Suit Gundam",
  "categories": {
    "mobile_suits": ["zaku", "gouf", "sazabi", "nu-gundam", ...],
    "characters": ["amuro", "char", "lalah", ...],
    "ships": ["white-base", "argama", ...]
  },
  "default_category": "mobile_suits"
}
```

**Adjectives** — theme-independent, grouped by flavor:
- `general`: burning, crimson, dark, fierce, golden, hidden, iron, phantom, savage, shining, silent, vicious, wild
- `combat`: armored, ballistic, charged, elite, fortified, heavy, lethal, stealth, tactical
- `cosmic`: astral, celestial, lunar, nebula, nova, orbital, solar, stellar, void

**Patterns** — configurable templates:
- `default`: `{adjective}-{noun}-{token}`
- `classified`: `{noun}-{adjective}-{token}`
- `designation`: `{adjective}-{noun}-class-{token}`
- `codename`: `{adjective}-{noun}`
- `serial`: `{noun}-{token}`

### API Surface

Consistent across all four languages, adapted to idioms.

```ruby
# Ruby
Dubs.generate                                    # => "vicious-sazabi-a3f1"
Dubs.generate(theme: :gundam, token: :hex)       # => "crimson-exia-b7e2"
Dubs.generate(theme: :star_trek, pattern: :designation, token: :hex, token_length: 6, token_case: :upper)
# => "burning-akira-class-EFFB1A"

Dubs.themes                                      # => [:gundam, :star_trek, ...]
Dubs.theme(:gundam).categories                   # => [:mobile_suits, :characters, :ships]
Dubs.register_theme(name: :battletech, data: {}) # custom themes
```

```python
# Python
dubs.generate()
dubs.generate(theme="gundam", token="hex")
dubs.themes()
```

```typescript
// TypeScript
import { generate } from "@saiden/dubs";
generate({ theme: "gundam", token: "hex" });
```

```rust
// Rust
dubs::generate()?;
Dubs::new(Theme::Gundam).token(TokenType::Hex).generate()?;
```

### Options

| Option | Values | Default |
|--------|--------|---------|
| `theme` | `:gundam`, `:star_trek`, `:star_wars`, `:transformers`, `:warhammer_40k`, `:nato` | random theme |
| `category` | theme-specific (e.g. `:mobile_suits`) | theme's `default_category` |
| `pattern` | `:default`, `:classified`, `:designation`, `:codename`, `:serial` | `:default` |
| `token` | `:hex`, `:numeric`, `:alpha`, `:none` | `:numeric` |
| `token_length` | integer | `4` |
| `token_case` | `:lower`, `:upper` | `:lower` |
| `separator` | string | `"-"` |
| `adjective_group` | `:general`, `:combat`, `:cosmic` | `:general` |
| `seed` | integer or nil | `nil` (random) |

### Themes for v1.0

| Theme | Categories | Sample |
|-------|-----------|--------|
| **gundam** | mobile_suits, characters, ships | `vicious-sazabi-a3f1` |
| **star-trek** | ship_classes, species, characters | `burning-akira-class-EFFB1A` |
| **star-wars** | vehicles, planets, characters, weapons | `shining-deathstar-AAFF22` |
| **transformers** | autobots, decepticons, combiners | `savage-megatron-7742` |
| **warhammer-40k** | chapters, units, primarchs | `iron-ultramarines-0451` |
| **nato** | phonetic, operations | `tactical-bravo-3318` |

### Design Decisions

1. **JSON over YAML** — native parsing in all 4 languages, no extra dependencies
2. **Copy-on-build** — `justfile` copies `data/` into each package before publish; packages are self-contained
3. **Seeded determinism via index selection** — `adj_idx = seed % adj.len`, `noun_idx = (seed / adj.len) % nouns.len`; portable across languages without PRNG matching
4. **Adjectives are theme-independent** — avoids duplication, allows mixing (cosmic adj + Gundam nouns)
5. **Functional + builder API** — quick `generate()` for 80% of cases, builder for complex config

### Publishing

| Language | Package | Registry |
|----------|---------|----------|
| Ruby | `dubs` | RubyGems |
| Python | `dubs` | PyPI |
| TypeScript | `@saiden/dubs` | npm |
| Rust | `dubs` | crates.io |

### Testing

1. **Per-language unit tests** — RSpec, pytest, vitest, cargo test
2. **Property tests** — output matches pattern regex, bounded length, seed determinism
3. **Cross-language consistency** — `tests/fixtures/seeded-expectations.json` with deterministic test vectors run against all 4 implementations
4. **Schema validation** — CI validates all theme JSON against `data/schema.json`

## Implementation Order

1. Data files + schema → 2. Ruby gem → 3. Rust crate → 4. TypeScript package → 5. Python package → 6. Remaining themes → 7. Cross-language tests + CI
