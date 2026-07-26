mod card;
mod deck;
mod hand;
mod player;
mod rules;
mod simulator;

use card::{Card, Rank, Suit};
use clap::{Parser, ValueEnum};
use hand::HandRank;
use rayon::prelude::*;
use rules::GameRules;
use simulator::PokerSimulator;
use std::collections::HashMap;
use std::ops::RangeInclusive;
use std::process::ExitCode;
use std::time::Instant;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Defaults to Texas
    #[arg(short, long, value_enum, default_value_t = GamePreset::Texas)]
    game: GamePreset,

    /// Number of players
    #[arg(short, long)]
    players: Option<usize>,

    /// Hole cards dealt to each player
    #[arg(short = 'H', long)]
    hole_cards: Option<usize>,

    /// Shared community cards
    #[arg(short, long)]
    community_cards: Option<usize>,

    /// Number of 52-card decks used
    #[arg(short, long)]
    decks: Option<usize>,

    /// Wild card, e.g. `AS`, `10H`, `KD`. Repeat the flag for multiple
    #[arg(short, long = "wild", value_parser = parse_card)]
    wild: Vec<Card>,

    /// How many hole cards a player must use, as an inclusive range
    #[arg(long, value_parser = parse_range)]
    hole_cards_in_play: Option<RangeInclusive<usize>>,

    /// Number of hands to simulate
    #[arg(short = 'n', long, default_value_t = 100_000)]
    simulations: usize,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, ValueEnum)]
enum GamePreset {
    #[value(alias = "holdem")]
    Texas,
    Omaha,
    #[value(alias = "draw")]
    FiveCardDraw,
}

impl GamePreset {
    fn rules(self, num_players: usize) -> GameRules {
        match self {
            GamePreset::Texas => GameRules::texas_holdem(num_players),
            GamePreset::Omaha => GameRules::omaha(num_players),
            GamePreset::FiveCardDraw => GameRules::five_card_draw(num_players),
        }
    }
}

fn parse_card(s: &str) -> Result<Card, String> {
    let s = s.trim().to_uppercase();
    if s.len() < 2 {
        return Err(format!("'{s}' is too short for a card (e.g. `AS`, `10H`)"));
    }
    let (rank_str, suit_str) = s.split_at(s.len() - 1);
    let suit = match suit_str {
        "H" => Suit::Hearts,
        "D" => Suit::Diamonds,
        "C" => Suit::Clubs,
        "S" => Suit::Spades,
        other => return Err(format!("unknown suit '{other}' (use H, D, C, or S)")),
    };
    let rank = match rank_str {
        "2" => Rank::Two,
        "3" => Rank::Three,
        "4" => Rank::Four,
        "5" => Rank::Five,
        "6" => Rank::Six,
        "7" => Rank::Seven,
        "8" => Rank::Eight,
        "9" => Rank::Nine,
        "10" | "T" => Rank::Ten,
        "J" => Rank::Jack,
        "Q" => Rank::Queen,
        "K" => Rank::King,
        "A" => Rank::Ace,
        other => return Err(format!("unknown rank '{other}' (use 2-10, T, J, Q, K, A)")),
    };
    Ok(Card::new(rank, suit))
}

/// `0..=2`: 0 up to 2; `2..=2`: two exactly.
fn parse_range(s: &str) -> Result<RangeInclusive<usize>, String> {
    let s = s.trim();
    let parse = |part: &str| -> Result<usize, String> {
        part.trim()
            .parse::<usize>()
            .map_err(|_| format!("'{part}' is not a valid number"))
    };

    let (start, end) = if let Some((a, b)) = s.split_once("..=") {
        (a, b)
    } else if let Some((a, b)) = s.split_once("..") {
        (a, b)
    } else if let Some((a, b)) = s.split_once('-') {
        (a, b)
    } else {
        (s, s)
    };
    Ok(parse(start)?..=parse(end)?)
}

fn main() -> ExitCode {
    let args = Args::parse();

    let players = args.players.unwrap_or(2);
    let mut rules = args.game.rules(players);

    if let Some(p) = args.players {
        rules.num_players = p;
    }
    if let Some(h) = args.hole_cards {
        rules.hole_cards_per_player = h;
        // default play none to all 
        if args.hole_cards_in_play.is_none() {
            rules.hole_cards_to_play = 0..=h;
        }
    }
    if let Some(c) = args.community_cards {
        rules.community_cards = c;
    }
    if let Some(d) = args.decks {
        rules.num_decks = d;
    }
    if !args.wild.is_empty() {
        rules.wild_cards = args.wild;
    }
    if let Some(range) = args.hole_cards_in_play {
        rules.hole_cards_to_play = range;
    }

    if let Err(err) = rules.validate() {
        eprintln!("Invalid rules: {err}");
        return ExitCode::FAILURE;
    }

    let num_simulations = args.simulations;

    println!(
        "Running {} simulations across {} threads",
        num_simulations,
        rayon::current_num_threads()
    );
    println!(
        "Rules: {} players, {} hole ({}-{} used) + {} community cards, {} deck(s), {} wild card(s)\n",
        rules.num_players,
        rules.hole_cards_per_player,
        rules.hole_cards_to_play.start(),
        rules.hole_cards_to_play.end(),
        rules.community_cards,
        rules.num_decks,
        rules.wild_cards.len(),
    );

    let start = Instant::now();
    let hand_type_counts: HashMap<HandRank, usize> = (0..num_simulations)
        .into_par_iter()
        .fold(HashMap::new, |mut counts, _| {
            let mut simulator = PokerSimulator::new(rules.clone());
            simulator.simulate_complete_hand();
            for hand_type in simulator.get_hand_types() {
                *counts.entry(hand_type).or_insert(0) += 1;
            }
            counts
        })
        .reduce(HashMap::new, |mut acc, partial| {
            for (hand_type, count) in partial {
                *acc.entry(hand_type).or_insert(0) += count;
            }
            acc
        });
    let elapsed = start.elapsed();

    let total_hands = num_simulations * rules.num_players;
    let sims_per_sec = num_simulations as f64 / elapsed.as_secs_f64();
    println!(
        "Simulated {} hands in {:.3?} ({:.0} simulations/sec)\n",
        total_hands, elapsed, sims_per_sec
    );
    println!("Hand Type Distribution ({} hands):", total_hands);
    for (hand_type, count) in &hand_type_counts {
        let percentage = (*count as f64) / (total_hands as f64) * 100.0;
        println!("  {:?}: {:.2}%", hand_type, percentage);
    }

    ExitCode::SUCCESS
}
