use player::Player;
use player::PlayerKind;

pub struct Table {
    fields: [ char; 9 ]
}

impl Table {
    pub fn new() -> Table {
        let mut _fields = [ ' '; 9 ];
        Table { fields: _fields }
    }

    pub fn mark(&mut self, player: &Player, field: usize) -> bool {
        if (field > 0 && field < 10) && self.fields[field-1] == ' ' {
            match player.kind {
                PlayerKind::X => self.fields[field-1] = 'x',
                PlayerKind::O => self.fields[field-1] = 'o'
            }
            return true;
        }

        false
    }

    pub fn print(&self) {
        println!("+---+---+---+");
        println!("| {} | {} | {} |", self.fields[0], self.fields[1], self.fields[2]);
        println!("+---+---+---+");
        println!("| {} | {} | {} |", self.fields[3], self.fields[4], self.fields[5]);
        println!("+---+---+---+");
        println!("| {} | {} | {} |", self.fields[6], self.fields[7], self.fields[8]);
        println!("+---+---+---+");
    }

    pub fn is_full(&self) -> bool {
        self.fields.iter().all(|&field| field != ' ')
    }

    pub fn ends_game(&self) -> bool {
        if self.is_full() {
            return true;
        }

        if  (self.fields[0] != ' ') && 
            (self.fields[0] == self.fields[1] && self.fields[1] == self.fields[2]) ||
            (self.fields[3] != ' ') && 
            (self.fields[3] == self.fields[4] && self.fields[4] == self.fields[5]) ||
            (self.fields[6] != ' ') &&
            (self.fields[6] == self.fields[7] && self.fields[7] == self.fields[8]) ||
            (self.fields[0] != ' ') && 
            (self.fields[0] == self.fields[3] && self.fields[3] == self.fields[6]) ||
            (self.fields[1] != ' ') && 
            (self.fields[1] == self.fields[4] && self.fields[4] == self.fields[7]) ||
            (self.fields[2] != ' ') && 
            (self.fields[2] == self.fields[5] && self.fields[5] == self.fields[8]) ||
            (self.fields[0] != ' ') && 
            (self.fields[0] == self.fields[4] && self.fields[4] == self.fields[8]) ||
            (self.fields[2] != ' ') && 
            (self.fields[2] == self.fields[4] && self.fields[4] == self.fields[6])
        {
            return true;
        }

        false
    }

    pub fn clear(&mut self) {
        for field in &mut self.fields {
            *field = ' ';
        }
    }
}