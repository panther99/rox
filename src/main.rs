use std::io;

mod table;
mod player;

fn main() {

	// create the table with 9 empty fields
	let mut table: [char; 9] = [' '; 9];

	// set x as first player ('o' will be changed to 'x')
	let mut player: char = 'o';

	let mut stdin = io::stdin();
	let mut input = String::new();
	let mut field = 0;
	let mut valid: bool = true;
	let mut game_over: bool = false;

	println!("ROX - Tic Tac Toe in Rust");
	println!("Author: Nikola S. (panther99)");
    
	while !game_over {
    
		// change player
		player::change_player(&mut player);

		// clears input, prints table and curren player
		input.clear();
		table::print_table(table);
		println!("Current player: {}", player);
		println!("Choose field (1-9): ");
		
		// get user input
		stdin.read_line(&mut input)
			.expect("Failed to read line");

		// we're shadowing field to usize to use it 
		// for indexing array in field checker
		let field = input.trim().parse::<usize>();

		// this will catch errors such as empty input
		match field {
			
			// check if user wrote valid field
			Ok(f) => {

				if f > 0 && f < 10 {
					if table[f-1] == ' ' {
						table[f-1] = player;
						valid = true;
					} else {
						println!("That field isn't empty!");
						valid = false;
					}
				} else {
					println!("That field doesn't exist!");
					valid = false;
				}

			},

			// changing player right away so
			// variable of current player will
			// return to the same player while
			// there's an error in user input
			Err(e) => { 
				println!("Error: {:?}", e);
				player::change_player(&mut player); 
			}

		}

		// do this while field is invalid
		while !valid {

			input.clear();
			table::print_table(table);
			println!("Choose field (1-9): ");
			stdin.read_line(&mut input)
				.expect("Failed to read line");

			let field = input.trim().parse::<usize>();

			// this will catch errors such as empty input
			match field {
			
				// check if user wrote valid field
				Ok(f) => {

					if f > 0 && f < 10 {
						if table[f-1] == ' ' {
							table[f-1] = player;
							valid = true;
						} else {
							println!("That field isn't empty!");
							valid = false;
						}
					} else {
						println!("That field doesn't exist!");
						valid = false;
					}

				},

				// changing player right away so
				// variable of current player will
				// return to the same player while
				// there's an error in user input
				Err(e) => { 
					println!("Error: {:?}", e);
					player::change_player(&mut player); 
				}

			}

		}

		// check if current player won the game
		game_over = table::check_table(table, player);

	}
	
	// check if table is full or last player won the game
	if table::full_table(table) {
		println!("It's a tie!");
	} else {
		println!("Player {} won the game!", player);
	}

}

