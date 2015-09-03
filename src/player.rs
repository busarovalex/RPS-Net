use server_commands::ServerCommand;
use game_state::GameState;

pub struct Player {
    pub to_write: Vec<ServerCommand>,
}

impl Player {
    pub fn new(game_state: GameState) -> Player {
        Player {
            to_write: vec![ServerCommand::GameState(game_state)],
        }
    }
}
