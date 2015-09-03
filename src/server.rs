use rps::{Game};
use rps::Player as RpsPlayer;
use rps::unit::{GeneralUnit};
use rps::moves::{Move};
use rps::win_conditions::WinCondition;

use player::Player;
use game_state::GameState;
use server_commands::ServerCommand;
use client_commands::ClientCommand;
use std::mem::replace;

const RED: RpsPlayer = RpsPlayer::Red;
const BLUE: RpsPlayer = RpsPlayer::Blue;

pub struct Room<T: WinCondition<GeneralUnit>> {
    pub game: Game<T>,
    pub red: Player,
    pub blue: Player,
}

impl<T: WinCondition<GeneralUnit>> Room<T> {
    
    pub fn new(game: Game<T>) -> Room<T> {
        let red_gamestate = GameState {
            turns: 1,
            current_turn: RED,
            winner: None,
            field: game.perspective(RED),
        };
        let blue_gamestate = GameState {
            turns: 1,
            current_turn: RED,
            winner: None,
            field: game.perspective(BLUE),
        };
        Room {
            game: game,
            red: Player::new(red_gamestate),
            blue: Player::new(blue_gamestate),
        }
    }
    
    pub fn force_win(&mut self, player: RpsPlayer) {
        self.game.force_win(player);
        let red = self.game_state(RED);
        self.send_to_player(RED, ServerCommand::GameState(red));
        let blue = self.game_state(BLUE);
        self.send_to_player(BLUE, ServerCommand::GameState(blue));
    }

    pub fn red_commands(&mut self) -> Vec<ServerCommand> {
        replace(&mut self.red.to_write, Vec::new())
    }
    
    pub fn blue_commands(&mut self) -> Vec<ServerCommand> {
        replace(&mut self.blue.to_write, Vec::new())
    }
    
    pub fn red_process(&mut self, command: ClientCommand) {
        self.process(RED, command);
    }
    
    pub fn blue_process(&mut self, command: ClientCommand) {
        self.process(BLUE, command);
    }
    
    fn process(&mut self, player: RpsPlayer, command: ClientCommand) {
        match command {
            ClientCommand::MakeMove(turn, move_command) => {
                self.process_move(player, turn, move_command);
            },
            _ => {},
        }
    }
    
    fn process_move(&mut self, player: RpsPlayer, turn: u32, move_command: Move) {
        if turn != self.game.turns() { return; }
        if player != self.game.current_turn() { return; }
        match self.game.make_move(move_command) {
            Ok(_) => {
                let red = self.game_state(RED);
                self.send_to_player(RED, ServerCommand::GameState(red));
                let blue = self.game_state(BLUE);
                self.send_to_player(BLUE, ServerCommand::GameState(blue));
            },
            Err(_) => {
                self.send_to_player(player, ServerCommand::InvalidMove(turn, move_command));
                let game_state = self.game_state(player);
                self.send_to_player(player, ServerCommand::GameState(game_state));
            }
        }
    }
    
    fn send_to_player(&mut self, player: RpsPlayer, command: ServerCommand) {
        match player {
            RED => self.red.to_write.push(command),
            BLUE => self.blue.to_write.push(command),
        }
    }
    
    fn game_state(&self, pov: RpsPlayer) -> GameState {
        GameState {
            turns: self.game.turns(),
            current_turn: self.game.current_turn(),
            winner: self.game.winner(),
            field: self.game.perspective(pov),
        }
    }
    
    pub fn finished(&self) -> bool {
        self.game.winner().is_some()
    }
}
