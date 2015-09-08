use rps::{Player, WIDTH, HEIGHT};
use rps::field::{Field, PovField};
use std::fmt::{Debug, Formatter, Error};

#[derive(Clone, Copy, RustcEncodable, RustcDecodable)]
pub struct GameState {
    pub turns: u32,
    pub current_turn: Player,
    pub winner: Option<Player>,
    pub field: PovField,
}

impl GameState {
    pub fn dumb() -> GameState {
        GameState {
            turns: 999,
            current_turn: Player::Red,
            winner: None,
            field: PovField {
                pov: Player::Red,
                field: Field {
                    rows: [[None; WIDTH]; HEIGHT],
                },
            },
        }
    }
    
    pub fn pov(&self) -> Player {
        self.field.pov
    }
}

impl Debug for GameState {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        f.debug_struct("GameState")
            .field("turns", &self.turns)
            .field("current_turn", &self.current_turn)
            .field("winner", &self.winner)
            .field("field", &"...")
            .finish()
    }
}
