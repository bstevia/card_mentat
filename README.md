# card_mentat

## Install

cargo install --path .

Or build in place with `cargo build --release` and run `./target/release/card_mentat`. Use `--release` - debug builds are slow.

## Usage

```
card_mentat [OPTIONS]
```

| Flag | Short | Default | Description |
|---|---|---|---|
| `--game` | `-g` | `texas` | Preset: `texas` (`holdem`), `omaha`, `five-card-draw` (`draw`) |
| `--players` | `-p` | `2` | Number of players dealt in |
| `--hole-cards` | `-H` | preset | Hole cards dealt to each player |
| `--community-cards` | `-c` | preset | Shared community (board) cards |
| `--decks` | `-d` | `1` | Number of 52-card decks in the shoe |
| `--wild` | `-w` | none | Mark a card wild; repeat for several |
| `--hole-cards-in-play` | | preset | Inclusive range of hole cards a player must use |
| `--simulations` | `-n` | `100000` | Number of hands to simulate |
| `--help` | `-h` | | Print help |
| `--version` | `-V` | | Print version |

Presets set the baseline; explicit flags override it. Note the capital `-H` - lowercase `-h` is `--help`.

**Presets**

| Preset | Hole cards | Board | Hole cards used |
|---|---|---|---|
| `texas` | 2 | 5 | `0..=2` |
| `omaha` | 4 | 5 | `2..=2` |
| `five-card-draw` | 5 | 0 | `5..=5` |

**Cards** - rank plus a one-letter suit, case-insensitive: `AS`, `kd`, `2c`. Suits are `H` `D` `C` `S`.

**Ranges** - `--hole-cards-in-play` accepts `2..=2`, `2..2`, `2-2`, or `2` (all "exactly two"), or `0..=2` for "up to two". Hold'em is `0..=2`; Omaha is `2..=2`.

## Examples

```bash
# Defaults: heads-up Hold'em, 100,000 hands
card_mentat

# Six-handed Hold'em, 200,000 hands
card_mentat --players 6 --simulations 200000

# Omaha, four players
card_mentat -g omaha -p 4

# Five-card draw with both black aces wild
card_mentat -g draw -p 5 -w AS -w AC

# Two decks - makes five of a kind possible
card_mentat -p 4 -H 3 -c 5 --hole-cards-in-play 1..=2 -d 2 -n 20000

# An invented variant: six hole cards, four-card board, at least two in play
card_mentat -p 8 -H 6 -c 4 --hole-cards-in-play 2..=5 -d 3 -n 5000
