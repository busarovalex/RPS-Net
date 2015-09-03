//! List of commands that clients send to server
use rps::moves::Move;

#[derive(Debug, Clone, Copy, PartialEq, Eq, RustcEncodable, RustcDecodable)]
pub enum ClientCommand {
    Ping,
    JoinNewGame,
    MakeMove(u32, Move),
}
