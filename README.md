# dubs

**Themed name generator. Like [haikunator](https://github.com/usmber/haikunator), but with giant robots and starships.**

```
vicious-sazabi-a3f1
burning-akira-class-EFFB1A
shining-deathstar-AAFF22
tactical-bravo-3318
savage-megatron-7742
```

Generate random, human-readable names from themed word lists. Perfect for server names, branch names, container IDs, session tokens, or anything that deserves a cooler name than `uuid-4f3a-...`.

Available in **Ruby**, **Python**, **TypeScript**, and **Rust** -- all sharing the same data files and producing the same output for a given seed.

## Install

```bash
# Ruby
gem install dubs

# Python
pip install dubs

# TypeScript
npm install @saiden/dubs

# Rust
cargo add dubs
```

## Quick Start

### Ruby

```ruby
require "dubs"

Dubs.generate
# => "vicious-sazabi-4271"

Dubs.generate(theme: :gundam, token: :hex, token_length: 6)
# => "crimson-exia-b7e2a1"

Dubs.generate(theme: :star_trek, pattern: :designation, token: :hex, token_length: 6, token_case: :upper)
# => "burning-akira-class-EFFB1A"
```

### Python

```python
import dubs

dubs.generate()
# => "phantom-zaku-8832"

dubs.generate(theme="star_wars", token="hex")
# => "golden-deathstar-c4f1"

dubs.generate(theme="transformers", adjective_group="combat", pattern="codename")
# => "tactical-megatron"
```

### TypeScript

```typescript
import { generate, themes } from "@saiden/dubs";

generate();
// => "iron-enterprise-7741"

generate({ theme: "gundam", token: "hex", tokenLength: 6 });
// => "blazing-sazabi-ff2a01"

generate({ theme: "nato", pattern: "serial" });
// => "foxtrot-2847"
```

### Rust

```rust
use dubs::Dubs;

// Quick generate
let name = dubs::generate()?;
// => "fierce-optimus-3318"

// Builder API
let name = Dubs::builder()
    .theme("warhammer_40k")
    .pattern("designation")
    .token_type("hex")
    .token_length(6)
    .token_case("upper")
    .build()
    .generate()?;
// => "iron-ultramarines-class-0A51FF"
```

## Themes

| Theme | What | Sample Nouns |
|-------|------|-------------|
| **gundam** | Mobile Suit Gundam | zaku, sazabi, exia, nu-gundam |
| **star-trek** | Star Trek | enterprise, akira, defiant, klingon |
| **star-wars** | Star Wars | deathstar, xwing, vader, tatooine |
| **transformers** | Transformers | optimus, megatron, starscream, devastator |
| **warhammer-40k** | Warhammer 40K | ultramarines, blood-angels, guilliman |
| **nato** | NATO Phonetic | alpha, bravo, foxtrot, tango |
| **pokemon** | Pokemon | pikachu, charizard, mewtwo, eevee |
| **leet** | Leet Speak | h4x0r, n00b, pwn3d, r00t |

## Patterns

Control the shape of the generated name:

| Pattern | Template | Example |
|---------|----------|---------|
| `default` | `{adjective}-{noun}-{token}` | `vicious-sazabi-a3f1` |
| `classified` | `{noun}-{adjective}-{token}` | `sazabi-vicious-a3f1` |
| `designation` | `{adjective}-{noun}-class-{token}` | `burning-akira-class-EFFB1A` |
| `codename` | `{adjective}-{noun}` | `savage-megatron` |
| `serial` | `{noun}-{token}` | `enterprise-7741` |

## Options

| Option | Values | Default |
|--------|--------|---------|
| `theme` | `gundam`, `star_trek`, `star_wars`, `transformers`, `warhammer_40k`, `nato`, `pokemon`, `leet` | random |
| `category` | theme-specific (e.g. `mobile_suits`, `characters`) | theme default |
| `pattern` | `default`, `classified`, `designation`, `codename`, `serial` | `default` |
| `token` | `hex`, `numeric`, `alpha`, `none` | `numeric` |
| `token_length` | any integer | `4` |
| `token_case` | `lower`, `upper` | `lower` |
| `separator` | any string | `"-"` |
| `adjective_group` | `general`, `combat`, `cosmic` | `general` |
| `seed` | integer or nil | `nil` (random) |

## Adjective Groups

Mix and match adjective flavors with any theme:

- **general** -- `blazing`, `crimson`, `fierce`, `golden`, `phantom`, `savage`, `vicious`, `wild` ...
- **combat** -- `armored`, `ballistic`, `elite`, `lethal`, `stealth`, `tactical` ...
- **cosmic** -- `astral`, `celestial`, `nebula`, `nova`, `stellar`, `void`, `warp` ...

```ruby
Dubs.generate(theme: :gundam, adjective_group: :cosmic)
# => "nebula-exia-3172"

Dubs.generate(theme: :warhammer_40k, adjective_group: :combat)
# => "siege-blood-angels-9041"
```

## Seeded Generation

Pass a seed for deterministic output -- same seed, same name, across all four languages:

```ruby
Dubs.generate(theme: :gundam, seed: 42)
# => always the same name
```

```python
dubs.generate(theme="gundam", seed=42)
# => same name as Ruby with seed 42
```

## Custom Themes

Register your own themes at runtime:

```ruby
Dubs.register_theme(
  name: :battletech,
  display_name: "BattleTech",
  categories: {
    mechs: ["atlas", "madcat", "timber-wolf", "daishi", "kodiak"],
    factions: ["clan-wolf", "jade-falcon", "comstar"]
  },
  default_category: "mechs"
)

Dubs.generate(theme: :battletech)
# => "fierce-atlas-2847"
```

Or load from a JSON file:

```ruby
Dubs.register_theme_file("path/to/battletech.json")
```

## License

MIT
