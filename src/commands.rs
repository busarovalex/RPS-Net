use rps::moves::Move;
use ::game_state::GameState;

/// List of commands that clients send to server
#[derive(Debug, Clone, Copy, PartialEq, Eq, RustcEncodable, RustcDecodable)]
pub enum ClientCommand {
    Ping,
    JoinNewGame,
    MakeMove(u32, Move),
}

/// List of commands that server sends to clients
#[derive(Debug, Clone, Copy, RustcEncodable, RustcDecodable)]
pub enum ServerCommand {
    NewGameStarted,
    GameState(GameState),
    InvalidMove(u32, Move),
}
