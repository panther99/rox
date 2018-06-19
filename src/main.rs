use std::io;

mod player;
mod table;

use player::Player as Player;
use player::PlayerKind as PlayerKind;
use table::Table as Table;

fn main() {
    let mut _table = Table::new();
    let mut _player = Player::new(PlayerKind::X);

    let mut input = String::new();
    let mut field;
    
    let mut game_over = false;

    println!(" _____ _____ __ __ ");
    println!("| __  |     |  |  |");
    println!("|    -|  |  |-   -|");
    println!("|__|__|_____|__|__|");

    println!("ROX - Tic Tac Toe in Rust");
    println!("Author: panther99 <nikola.stojakovic@hotmail.com>\n");

    while !game_over {
        input.clear();
        _table.print();
        println!("Current player: {}", _player.print());
        println!("Choose field (1-9): ");

        io::stdin().read_line(&mut input).expect("Failed to read the line.");
        match input.trim().parse::<usize>() {
            Ok(n) => field = n,
            Err(_) => {
                input.clear();
                field = 0;
            },
        }

        if !_table.mark(&_player, field) {
            while !_table.mark(&_player, field) {
                println!("Input is invalid or that field is already marked. Try again.");
                io::stdin().read_line(&mut input).expect("Failed to read the line.");
                match input.trim().parse::<usize>() {
                    Ok(n) => field = n,
                    Err(_) => {
                        input.clear();
                        field = 0;
                    },
                }
            }
        }

        if _table.ends_game() {
            if _table.is_full() {
                println!("It's a tie!");
            } else {
                println!("Player {} won the game!", _player.print());
            }

            println!("Do you want to play again? (y/n)");
            input.clear();
            io::stdin().read_line(&mut input).expect("Failed to read the line.");
            let answer = input.chars().next().unwrap();

            if answer == 'y' || answer == 'Y' {
                _table.clear();
                _player.set_to_x();
            } else {
                println!("Bye!");
                game_over = true;
            }
        } else {
            _player.change();
        }
    }
}