use rps::Player;
use rps::field::PovField;

#[derive(Debug, Clone, Copy, RustcEncodable, RustcDecodable)]
pub struct GameState {
    pub turns: u32,
    pub current_turn: Player,
    pub winner: Option<Player>,
    pub field: PovField,
}

impl GameState {
    pub fn pov(&self) -> Player {
        self.field.pov
    }
}
