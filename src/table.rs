pub fn print_table(table: [char; 9]) {
    
	println!("+---+---+---+");
	println!("| {} | {} | {} |", table[0], table[1], table[2]);
	println!("+---+---+---+");
	println!("| {} | {} | {} |", table[3], table[4], table[5]);
	println!("+---+---+---+");
	println!("| {} | {} | {} |", table[6], table[7], table[8]);
	println!("+---+---+---+");

}

/*
/	We're adding player as second parameter
/	because we want to know who played
/	last move so we wouldn't have to check
/	both times
*/
pub fn check_table(table: [char; 9], player: char) -> bool {

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

pub fn full_table(table: [char; 9]) -> bool {

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
