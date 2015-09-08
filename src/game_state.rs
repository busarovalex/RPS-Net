use rps::{Player, WIDTH, HEIGHT};
use rps::field::{Field, PovField};

#[derive(Debug, Clone, Copy, RustcEncodable, RustcDecodable)]
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
