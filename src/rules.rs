use crate::card::Card;
use std::fmt;
use std::ops::RangeInclusive;

/// Configurable rules for simulations
///
/// Presets: [`GameRules::texas_holdem`], [`GameRules::omaha`], [`GameRules::five_card_draw`]
/// [`GameRules::builder`]
#[derive(Debug, Clone)]
pub struct GameRules {
    pub num_players: usize,
    pub hole_cards_per_player: usize,
    pub community_cards: usize,
    pub num_decks: usize,
    pub wild_cards: Vec<Card>,
    pub hole_cards_to_play: RangeInclusive<usize>,
}

impl GameRules {
    pub fn builder() -> GameRulesBuilder {
        GameRulesBuilder::default()
    }

    pub fn texas_holdem(num_players: usize) -> Self {
        GameRules {
            num_players,
            hole_cards_per_player: 2,
            community_cards: 5,
            num_decks: 1,
            wild_cards: Vec::new(),
            hole_cards_to_play: 0..=2,
        }
    }

    pub fn omaha(num_players: usize) -> Self {
        GameRules {
            num_players,
            hole_cards_per_player: 4,
            community_cards: 5,
            num_decks: 1,
            wild_cards: Vec::new(),
            hole_cards_to_play: 2..=2,
        }
    }

    pub fn five_card_draw(num_players: usize) -> Self {
        GameRules {
            num_players,
            hole_cards_per_player: 5,
            community_cards: 0,
            num_decks: 1,
            wild_cards: Vec::new(),
            hole_cards_to_play: 5..=5,
        }
    }

    pub fn deck_size(&self) -> usize {
        52 * self.num_decks
    }

    pub fn cards_needed(&self) -> usize {
        self.num_players * self.hole_cards_per_player + self.community_cards
    }

    pub fn validate(&self) -> Result<(), RulesError> {
        if self.num_players == 0 {
            return Err(RulesError::NoPlayers);
        }
        if self.num_decks == 0 {
            return Err(RulesError::NoDecks);
        }
        let hand_size = self.hole_cards_per_player + self.community_cards;
        if hand_size < 5 {
            return Err(RulesError::HandTooSmall {
                available: hand_size,
            });
        }

        let (min_play, max_play) = (
            *self.hole_cards_to_play.start(),
            *self.hole_cards_to_play.end(),
        );
        if min_play > max_play {
            return Err(RulesError::EmptyHolePlayRange {
                min: min_play,
                max: max_play,
            });
        }
        // There must be at least one split that forms a 5-card hand: `h` hole
        // cards (within range and on hand) plus `5 - h` from the board.
        let satisfiable = (min_play..=max_play).any(|h| {
            h <= 5 && h <= self.hole_cards_per_player && (5 - h) <= self.community_cards
        });
        if !satisfiable {
            return Err(RulesError::HolePlayUnsatisfiable {
                min: min_play,
                max: max_play,
                hole: self.hole_cards_per_player,
                community: self.community_cards,
            });
        }

        let needed = self.cards_needed();
        let available = self.deck_size();
        if needed > available {
            return Err(RulesError::NotEnoughCards { needed, available });
        }
        Ok(())
    }
}

impl Default for GameRules {
    fn default() -> Self {
        GameRules::texas_holdem(2)
    }
}


#[derive(Debug, Clone, Default)]
pub struct GameRulesBuilder {
    rules: GameRules,
    hole_play_explicit: bool,
}

impl GameRulesBuilder {
    pub fn players(mut self, n: usize) -> Self {
        self.rules.num_players = n;
        self
    }

    pub fn hole_cards(mut self, n: usize) -> Self {
        self.rules.hole_cards_per_player = n;
        // Unless the caller pinned a range, default to "play any of them".
        if !self.hole_play_explicit {
            self.rules.hole_cards_to_play = 0..=n;
        }
        self
    }

    /// Constrain how many hole cards a player may use, e.g. `2..=2` for Omaha.
    pub fn hole_cards_to_play(mut self, range: RangeInclusive<usize>) -> Self {
        self.rules.hole_cards_to_play = range;
        self.hole_play_explicit = true;
        self
    }

    pub fn community_cards(mut self, n: usize) -> Self {
        self.rules.community_cards = n;
        self
    }

    pub fn decks(mut self, n: usize) -> Self {
        self.rules.num_decks = n;
        self
    }

    pub fn wild_cards(mut self, cards: Vec<Card>) -> Self {
        self.rules.wild_cards = cards;
        self
    }

    pub fn add_wild_card(mut self, card: Card) -> Self {
        self.rules.wild_cards.push(card);
        self
    }

    pub fn build(self) -> Result<GameRules, RulesError> {
        self.rules.validate()?;
        Ok(self.rules)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RulesError {
    NoPlayers,
    NoDecks,
    HandTooSmall { available: usize },
    NotEnoughCards { needed: usize, available: usize },
    /// The hole-cards-to-play range is inverted (min greater than max).
    EmptyHolePlayRange { min: usize, max: usize },
    /// No allowed number of hole cards can combine with the board to make five.
    HolePlayUnsatisfiable {
        min: usize,
        max: usize,
        hole: usize,
        community: usize,
    },
}

impl fmt::Display for RulesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RulesError::NoPlayers => write!(f, "at least one player is required"),
            RulesError::NoDecks => write!(f, "at least one deck is required"),
            RulesError::HandTooSmall { available } => write!(
                f,
                "a 5-card hand is impossible: only {} card(s) per player (hole + community)",
                available
            ),
            RulesError::NotEnoughCards { needed, available } => write!(
                f,
                "not enough cards: the deal needs {} but the shoe holds {}",
                needed, available
            ),
            RulesError::EmptyHolePlayRange { min, max } => write!(
                f,
                "hole cards to play range is empty: min {} is greater than max {}",
                min, max
            ),
            RulesError::HolePlayUnsatisfiable {
                min,
                max,
                hole,
                community,
            } => write!(
                f,
                "cannot form a 5-card hand playing {}..={} hole cards from {} hole + {} community",
                min, max, hole, community
            ),
        }
    }
}

impl std::error::Error for RulesError {}
