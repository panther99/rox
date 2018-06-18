use std::io;

mod table;
mod player;

fn main() {

    // create the table with 9 empty fields
    let mut _table: [char; 9] = [' '; 9];

    // set x as first player
    let mut _player = 'x';
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
        table::print_table(_table);
        println!("Current player: {}", _player);
        println!("Choose field (1-9): ");

        io::stdin().read_line(&mut input).expect("Failed to read the line.");
        match input.trim().parse::<usize>() {
            Ok(n) => field = n,
            Err(_) => {
                input.clear();
                field = 0;
            },
        }

        if table::valid_move(_table, field) {
            _table[field-1] = _player;
        } else {
            while !table::valid_move(_table, field) {
                println!("Please type the number between 1 and 9.");
                io::stdin().read_line(&mut input).expect("Failed to read the line.");
                match input.trim().parse::<usize>() {
                    Ok(n) => field = n,
                    Err(_) => {
                        input.clear();
                        field = 0;
                    },
                }
            }
            _table[field-1] = _player;
        }

        if table::check_table(_table) {
            if table::full_table(_table) {
                println!("It's a tie!");
            } else {
                println!("Player {} won the game!", _player);
            }

            println!("Do you want to play again? (y/n)");
            input.clear();
            io::stdin().read_line(&mut input).expect("Failed to read the line.");
            let answer = input.chars().next().unwrap();

            if answer == 'y' || answer == 'Y' {
                table::clear_table(&mut _table);
                _player = 'x';
            } else {
                println!("Bye!");
                game_over = true;
            }
        } else {
            player::change_player(&mut _player);
        }
    }

}