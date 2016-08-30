use std::io;

fn print_table(table: [char; 9]) {
    
	println!("+---+---+---+");
	println!("| {} | {} | {} |", table[0], table[1], table[2]);
	println!("+---+---+---+");
	println!("| {} | {} | {} |", table[3], table[4], table[5]);
	println!("+---+---+---+");
	println!("| {} | {} | {} |", table[6], table[7], table[8]);
	println!("+---+---+---+");

}

fn change_player(mut player: char) -> char {
	if player == 'x' { 'o' } else { 'x' }
}

/*
/	We're adding player as second parameter
/	because we want to know who played
/	last move so we wouldn't have to check
/	both times
*/
fn check_table(table: [char; 9], player: char) -> bool {

	if full_table(table) {
		return true;
	}

	if (table[0], table[1], table[2]) == (player, player, player) ||
		(table[3], table[4], table[5]) == (player, player, player) ||
		(table[6], table[7], table[8]) == (player, player, player) ||
		(table[0], table[3], table[6]) == (player, player, player) ||
		(table[1], table[4], table[7]) == (player, player, player) ||
		(table[2], table[5], table[8]) == (player, player, player) ||
		(table[0], table[4], table[8]) == (player, player, player) ||
		(table[2], table[4], table[6]) == (player, player, player) 
	{	
		return true;
	}

	false

}

fn full_table(table: [char; 9]) -> bool {

	if table[0] != ' ' &&
		table[1] != ' ' &&
		table[2] != ' ' &&
		table[3] != ' ' &&
		table[4] != ' ' &&
		table[5] != ' ' &&
		table[6] != ' ' &&
		table[7] != ' ' &&
		table[8] != ' ' 
	{
		true
	} 
	else 
	{
		false
	}

}

fn main() {

	let mut table: [char; 9] = [' '; 9];

	// set x as first player
	let mut player: char = 'x';

	let mut stdin = io::stdin();
	let mut input = String::new();
	let mut field = 0;
	let mut valid: bool = true;
	let mut game_over: bool = false;

	println!("ROX - Tic Tac Toe in Rust");
	println!("Author: Nikola S. (panther99)");
    
	while !game_over {
    
		// change player
		player = change_player(player);

		// clears input, prints table and curren player
		input.clear();
		print_table(table);
		println!("Current player: {}", player);
		println!("Choose field (1-9): ");
		
		// get user input
		io::stdin().read_line(&mut input)
			.expect("Failed to read line");

		// we're shadowing field to usize to use it 
		// for indexing array in field checker
		let field: usize = input.trim().parse()
			.expect("Failed to read line");

		// check if user wrote valid field
		if field > 0 && field < 10 {
			if table[field-1] == ' ' {
				table[field-1] = player;
				valid = true;
			} else {
				println!("That field isn't empty!");
				valid = false;
			}
		} else {
			println!("That field doesn't exist!");
			valid = false;
		}

		// do this while field is invalid
        while !valid {

			input.clear();
			print_table(table);
			println!("Choose field (1-9): ");
			io::stdin().read_line(&mut input)
				.expect("Failed to read line");
			let field: usize = input.trim().parse()
				.expect("Failed to read line");
		
			if field > 0 && field < 10 {
				if table[field-1] == ' ' {
					table[field-1] = player;
					valid = true;
				} else {
					println!("That field isn't empty!");
					valid = false;
				}
			} else {
				println!("That field doesn't exist!");
				valid = false;
			}

        }

		// check if current player won the game
		game_over = check_table(table, player);

    }
	
	// check if table is full or last player won the game
	if full_table(table) {
		println!("It's a tie!");
	} else {
		println!("Player {} won the game!", player);
	}

}

