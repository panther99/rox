use std::io;

mod player;
mod table;
mod game;

use game::Game;
use game::GameState;

fn main() {
    let mut current_game = Game::new_against_human();
    let mut input = String::new();
    let mut field = 20;
    let mut game_started = false;

    println!(" _____ _____ __ __ ");
    println!("| __  |     |  |  |");
    println!("|    -|  |  |-   -|");
    println!("|__|__|_____|__|__|");

    println!("ROX - Tic Tac Toe in Rust");
    println!("Author: panther99 <nikola.stojakovic@hotmail.com>");

    while current_game.is_running() {
        while !current_game.is_over() {

            while !game_started {
                println!("\nChoose playing mod:");
                println!("[1] Player VS Player");
                println!("[2] Player VS Computer\n");

                io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read the line.");
                let answer = input.chars().next().unwrap();

                match answer {
                    '1' => {
                        game_started = true;
                    },
                    '2' => {
                        current_game = Game::new_against_computer();
                        game_started = true;
                    },
                    _ => {
                        input.clear();
                        println!("Please provide a valid input (1 or 2).");
                    }
                }
            }

            input.clear();
            current_game.print_table();
            println!("Current player: {}", current_game.current_player());

            println!("Choose field (1-9):");
            if current_game.players_turn() {
                while !current_game.play_player(field) {
                    io::stdin()
                        .read_line(&mut input)
                        .expect("Failed to read the line.");

                    match input.trim().parse::<usize>() {
                        Ok(n) => field = n,
                        Err(_) => {
                            input.clear();
                            field = 0;
                            println!("That field is already filled or input is invalid. Try again.");
                        }
                    }
                }
            } else {
                current_game.play_computer();
            }

        }

        match current_game.check_winner() {
            GameState::XWon => println!("Player X won the game!"),
            GameState::OWon => println!("Player O won the game!"),
            GameState::HumanWon => println!("Human won the game!"),
            GameState::ComputerWon => println!("Computer won the game!"),
            GameState::Tie => println!("It's a tie!")
        }

        println!("Do you want to play again? (y/n)");
        input.clear();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read the line.");
        let answer = input.chars().next().unwrap();

        if answer == 'y' || answer == 'Y' {
            input.clear();
            current_game = Game::new_against_human();
            game_started = false;
            field = 20;
        } else {
            println!("Bye!");
            current_game.quit();
        }
    }
}