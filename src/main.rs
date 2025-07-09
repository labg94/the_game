use crate::domain::game::{Game, GameResult};
use inquire::Text;

mod domain;

use crate::domain::card::{Card, Pile, PileDirection};
use colored::Colorize;
use inquire::Select;

fn show_game_state(player_cards: &Vec<Card>, piles: &[Pile; 4]) {
    println!("\n{}", "=== Current Game State ===".cyan().bold());

    // Show piles with direction and top card
    println!("\n{}", "Piles:".green().bold());
    for (i, pile) in piles.iter().enumerate() {
        let direction = match pile.direction {
            PileDirection::Ascending => "↑ Ascending".bright_green(),
            PileDirection::Descending => "↓ Descending".bright_red(),
        };
        println!("Pile {}: {} - Top Card: {}", i + 1, direction, pile.top.0);
    }

    // Show player's cards
    println!("\n{}", "Your Cards:".green().bold());
    for (i, card) in player_cards.iter().enumerate() {
        println!("{}. {}", i + 1, card.0);
    }
    println!();
}

fn play_turn(game: &mut Game) {
    while game.current_status() == GameResult::InProgress {
        show_game_state(&game.player.get_cards(), &game.show_piles());

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
                    .player
                    .get_cards()
                    .iter()
                    .map(|c| c.0.to_string())
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
                        let direction = match p.direction {
                            PileDirection::Ascending => "↑",
                            PileDirection::Descending => "↓",
                        };
                        format!("Pile {} {} (Top: {})", i + 1, direction, p.top.0)
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
                    println!("{}", e.bright_red());
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

fn main() {
    let name = Text::new("What is your name?")
        .prompt()
        .expect("Failed to get name");

    let mut game = Game::new(name);

    while game.current_status() == GameResult::InProgress {
        show_game_state(&game.player.get_cards(), &game.show_piles());

        play_turn(&mut game);
    }

    show_game_state(&game.player.get_cards(), &game.show_piles());
    match game.current_status() {
        GameResult::PlayerWin => {
            println!("You win!")
        }
        GameResult::GameWin => {
            println!("The game win!")
        }
        GameResult::InProgress => {
            println!("you should be playing")
        }
    }
}
