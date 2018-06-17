use std::io;

mod table;
mod player;

fn main() {

    // create the table with 9 empty fields
    let mut _table: [char; 9] = [' '; 9];

    // set x as first player
    let mut _player = 'x';

    let mut stdin = io::stdin();
    let mut input = String::new();
    let mut field = 0;
    let mut valid = false;
    let mut game_over = false;

    println!("ROX - Tic Tac Toe in Rust");
    println!("Author: Nikola S. (panther99)");

    while !game_over
    {
        input.clear();
        table::print_table(_table);
        println!("Current player: {}", _player);
        println!("Choose field (1-9): ");

        stdin.read_line(&mut input).expect("Failed to read the line.");
        let field = input.trim().parse::<usize>().unwrap();

        if table::valid_move(_table, field) {
            _table[field-1] = _player;
        } else {
            while !table::valid_move(_table, field) {
                stdin.read_line(&mut input).expect("Failed to read the line.");
                let field = input.trim().parse::<usize>().unwrap();
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
            stdin.read_line(&mut input).expect("Failed to read the line.");
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