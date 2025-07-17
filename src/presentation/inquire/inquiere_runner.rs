use crate::domain::card::{PileDirection};
use crate::domain::game::{Game, GameResult};
use colored::Colorize;
use inquire::{Select, Text};

fn show_game_state(game: &Game) {
    let player_cards = game.player_cards();
    let piles = game.show_piles();
    println!("\n{}", "=== Current Game State ===".cyan().bold());

    // Show piles with their direction and top card
    println!("\n{}", "Piles:".green().bold());
    for (i, pile) in piles.iter().enumerate() {
        let direction = match pile.get_direction() {
            PileDirection::Ascending => "↑ Ascending".bright_green(),
            PileDirection::Descending => "↓ Descending".bright_red(),
        };
        println!(
            "Pile {}: {} - Top Card: {}",
            i + 1,
            direction,
            pile.get_top()
        );
    }

    // Show player's cards
    println!("\n{}", "Remaining Cards:".green().bold());
    for card in game.remaining_cards().iter() {
        print!("-{}", card.value());
    }
    
    // Show player's cards
    println!("\n{}", "Your Cards:".green().bold());
    for card in player_cards.iter() {
        print!(" ({})", card.value());
    }
    println!();
}

fn play_turn(game: &mut Game) {
    while game.current_status() == GameResult::InProgress {
        show_game_state(&game);

        let actions = vec!["Play Card", "End Turn"];

        let choice = if game.can_finish_turn() {
            let choice = Select::new("What would you like to do?", actions)
                .prompt()
                .expect("Failed to select action");
            choice
        } else {
            "Play Card"
        };

        match choice {
            "Play Card" => {
                // Convert cards to strings for selection
                let card_options: Vec<String> = game
                    .player_cards()
                    .iter()
                    .map(|c| c.value().to_string())
                    .collect();

                // Select a card
                let selected_card = Select::new("Choose a card to play:", card_options)
                    .prompt()
                    .expect("Failed to select card");

                // Create pile options with their direction and top card
                let pile_options: Vec<String> = game
                    .show_piles()
                    .iter()
                    .enumerate()
                    .map(|(i, p)| {
                        let direction = match p.get_direction() {
                            PileDirection::Ascending => "↑",
                            PileDirection::Descending => "↓",
                        };
                        format!("Pile {} {} (Top: {})", i + 1, direction, p.get_top())
                    })
                    .collect();

                // Select a pile
                let selected_pile = Select::new("Choose a pile:", pile_options)
                    .prompt()
                    .expect("Failed to select pile");

                // Extract pile index from selection (assuming format "Pile X ...")
                let pile_index = selected_pile
                    .split_whitespace()
                    .nth(1)
                    .and_then(|s| s.parse::<usize>().ok())
                    .map(|n| n - 1)
                    .unwrap();

                // Convert selected card string to Card
                let card_number = selected_card.parse::<u8>().unwrap();

                // Try to play the card
                if let Err(e) = game.play_card(card_number, pile_index) {
                    println!("{}", e.to_string().bright_red());
                }
            }
            "End Turn" => {
                game.finnish_turn();
            }
            _ => {
                println!("Invalid action");
            }
        }
    }
}

pub fn run() {
    let name = Text::new("What is your name?")
        .prompt()
        .expect("Failed to get name");

    let mut game = Game::new(name);

    while game.current_status() == GameResult::InProgress {
        play_turn(&mut game);
    }

    show_game_state(&game);
    match game.current_status() {
        GameResult::PlayerWin => {
            println!("{} {}", game.player_name(), " You win!".bright_green())
        }
        GameResult::GameWin => {
            println!("{}", "The game win!".bright_red())
        }
        GameResult::InProgress => {
            println!("you should be playing")
        }
    }
}
