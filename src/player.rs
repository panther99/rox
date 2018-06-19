pub enum PlayerKind {
    X,
    O
}

pub struct Player {
    pub kind: PlayerKind
}

impl Player {
    pub fn new(_kind: PlayerKind) -> Player {
        Player { kind: _kind }
    }

    pub fn change(&mut self) {
        match self.kind {
            PlayerKind::X => self.kind = PlayerKind::O,
            PlayerKind::O => self.kind = PlayerKind::X
        }
    }

    pub fn print(&self) -> String {
        match self.kind {
            PlayerKind::X => String::from("x"),
            PlayerKind::O => String::from("o")
        }
    }

    pub fn set_to_x(&mut self) {
        self.kind = PlayerKind::X;
    }
}