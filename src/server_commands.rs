//! List of commands that server sends to clients
use ::game_state::GameState;
use rps::moves::Move;


#[derive(Debug, Clone, Copy, RustcEncodable, RustcDecodable)]
pub enum ServerCommand {
    NewGameStarted,
    GameState(GameState),
    InvalidMove(u32, Move),
}
