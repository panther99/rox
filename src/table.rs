pub fn print_table(table: [char; 9])
{
    println!("+---+---+---+");
    println!("| {} | {} | {} |", table[0], table[1], table[2]);
    println!("+---+---+---+");
    println!("| {} | {} | {} |", table[3], table[4], table[5]);
    println!("+---+---+---+");
    println!("| {} | {} | {} |", table[6], table[7], table[8]);
    println!("+---+---+---+");
}

pub fn full_table(table: [char; 9]) -> bool
{
    table.iter().all(|&c| c != ' ')
}

pub fn check_table(table: [char; 9]) -> bool
{
    if full_table(table) {
        return true;
    }

    if  (table[0] != ' ') && (table[0] == table[1] && table[1] == table[2]) ||
        (table[3] != ' ') && (table[3] == table[4] && table[4] == table[5]) ||
        (table[6] != ' ') && (table[6] == table[7] && table[7] == table[8]) ||
        (table[0] != ' ') && (table[0] == table[3] && table[3] == table[6]) ||
        (table[1] != ' ') && (table[1] == table[4] && table[4] == table[7]) ||
        (table[2] != ' ') && (table[2] == table[5] && table[5] == table[8]) ||
        (table[0] != ' ') && (table[0] == table[4] && table[4] == table[8]) ||
        (table[2] != ' ') && (table[2] == table[4] && table[4] == table[6])
    {
        return true;
    }

    false
}

pub fn valid_move(table: [char; 9], field: usize) -> bool
{
    let mut valid = false;

    if field > 0 && field < 10 {
        if table[field-1] == ' ' {
            valid = true;
        }
    }

    return valid;
}

pub fn clear_table(table: &mut [char; 9])
{
    for field in table {
        *field = ' ';
    }
}