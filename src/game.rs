extern crate rand;
use game::rand::prelude::*;
use std::collections::HashMap;

use table::Table;
use player::Player;
use player::PlayerKind;

pub enum GameState {
    XWon,
    OWon,
    HumanWon,
    ComputerWon,
    Tie
}

pub struct Game {
    player: Player,
    table: Table,
    available_moves: HashMap<usize, usize>,
    ref_available_moves: Vec<usize>,
    against_computer: bool,
    over: bool,
    running: bool,
    steps: u32
}

fn game_template() -> Game {
    // Generating HashMap for available moves.
    let mut moves = HashMap::new();
    for i in 1..10 {
        moves.insert(i, i);
    }

    /*
     * Vector will work as a reference to the
     * HashMap with available moves which will
     * allow us to implement random access for
     * game with computer.
     */
    let mut references: Vec<usize> = vec![];
    for i in 1..10 {
        references.push(i);
    }

    Game {
        player: Player::new(PlayerKind::X),
        table: Table::new(),
        available_moves: moves,
        ref_available_moves: references,
        against_computer: false,
        over: false,
        running: true,
        steps: 1
    }
}

impl Game {
    pub fn new_against_human() -> Game {
        game_template()
    }

    pub fn new_against_computer() -> Game {
        Game {
            against_computer: true,
            ..game_template()
        }
    }

    pub fn change_player(&mut self) {
        self.player.change();
        self.steps += 1;
    }

    pub fn current_player(&self) -> String {
        self.player.print()
    }

    fn remove_from_available_moves(&mut self, _move: usize) -> bool {
        if self.mark_table(_move) {
            if let Some(_) = self.available_moves.get(&_move) {
                self.available_moves.remove(&_move);
                self.ref_available_moves.retain(|&x| x != _move);
                return true;
            }
        }
        false
    }

    pub fn players_turn(&self) -> bool {
        !self.against_computer || self.steps % 2 != 0
    }

    pub fn play_player(&mut self, _move: Option<usize>) -> bool {
        match _move {
            Some(n) => {
                if self.remove_from_available_moves(n) {
                    self.change_player();
                    self.check_table();
                    return true;
                }
                return false;
            },
            None => return false
        }
        /*
        if self.remove_from_available_moves(_move) {
            self.change_player();
            self.check_table();
            return true;
        }
        false
        */
    }

    pub fn play_computer(&mut self) -> bool {
        if self.against_computer && self.steps % 2 == 0 {
            let mut rng = thread_rng();
            let _move: usize = rng.gen_range(1, self.ref_available_moves.len());
            if self.remove_from_available_moves(_move) {
                self.change_player();
                self.check_table();
                return true;
            }
        }
        false
    }

    pub fn print_table(&self) {
        self.table.print();
    }

    fn mark_table(&mut self, field: usize) -> bool {
        self.table.mark(&self.player, field)
    }

    pub fn check_table(&mut self) {
        if self.table.is_full() || self.table.ends_game()  {
            self.over = true;
        }
    }

    pub fn is_over(&self) -> bool {
        self.over
    }

    pub fn is_running(&self) -> bool {
        self.running
    }

    /*
     * As players are changed after every step
     * we just have to show opposite player
     * than the current one as a winner.
     */
    pub fn check_status(&self) -> GameState {
        if self.table.is_full() {
            return GameState::Tie;
        }

        if self.against_computer {
            match self.player.kind {
                PlayerKind::X => return GameState::ComputerWon,
                PlayerKind::O => return GameState::HumanWon
            };
        } else {
            match self.player.kind {
                PlayerKind::X => return GameState::OWon,
                PlayerKind::O => return GameState::XWon
            };
        }
    }

    pub fn quit(&mut self) {
        self.running = false;
    }
}