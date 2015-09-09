use rps::moves::Move;
use rps::Player;

use std::net::TcpStream;
use std::io::{Read, Write, Error};
use std::time::Duration;

use super::reader::Reader;
use super::writer::Writer;
use super::error::IterError;
use game_state::GameState;
use commands::ClientCommand;
use commands::ServerCommand;

pub const TIMEOUT_MSEC: u32 = 50;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum State {
    Idle,
    WaitingForGame,
    Game,
}

pub struct Inner {
    game_state: GameState,
    stream: TcpStream,
    reader: Reader,
    writer: Writer,
    move_to_write: Option<Move>,
    state: State,
    last_sent_move: Option<u32>,
    shot_down: bool,
}

impl Inner {
    pub fn connect(address: &str) -> Result<Inner, Error> {
        let stream = try!( TcpStream::connect(address) );
        try!( stream.set_read_timeout(Some(Duration::new(0, TIMEOUT_MSEC * 1000))) );
        try!( stream.set_write_timeout(Some(Duration::new(0, TIMEOUT_MSEC * 1000))) );
        
        Ok(Inner {
            game_state: GameState::dumb(),
            stream: stream,   
            reader: Reader::new(),
            writer: Writer::new(),
            move_to_write: None,
            state: State::Idle,
            last_sent_move: None,
            shot_down: false,
        })    
    }
    
    pub fn game_in_progress(&self) -> bool {
        self.state == State::Game
    }
    
    pub fn pov(&self) -> Player { self.game_state.pov() }
    
    pub fn is_shot_down(&self) -> bool { self.shot_down }
    
    pub fn shut_down(&mut self) {
        self.shot_down = true;
    }
    
    pub fn can_send_move(&self) -> bool {
        self.state == State::Game
        && self.game_state.current_turn == self.game_state.pov()
        && self.last_sent_move.is_none()
        && self.game_state.winner.is_none()
    }
    
    pub fn possible_moves(&self) -> Vec<Move> {
        self.game_state.field.possible_moves()
    }
    
    pub fn game_state(&self) -> Option<GameState> {
        if (self.state == State::Game) || 
           (self.state == State::Idle && self.game_state.winner.is_some()) 
        {    
            Some(self.game_state)
        } else {
            None
        }
    }
    
    pub fn send_move(&mut self, movement: Move) {
        self.move_to_write = Some(movement);
        self.last_sent_move = Some(self.game_state.turns);
    }
    
    pub fn join(&mut self) {
        if self.state != State::WaitingForGame {
            self.writer.push(ClientCommand::JoinNewGame);
            self.state = State::WaitingForGame;
        }
    }
    
    pub fn one_cycle(&mut self) {
        trace!("New cycle: {:?}", self.state);
        match self.state {
            State::Idle => {
                if let Err(e) = self.ping_iter() {
                    trace!("StateIdle error: {:?}", e);
                }
            },
            State::WaitingForGame => {
                if let Err(e) = self.join_iter() {
                    trace!("StateWaitingForGame error: {:?}", e);
                }
            },
            State::Game => {
                if let Err(e) = self.game_iter() {
                    trace!("StateGame error: {:?}", e);
                }
            }
        }
    }
    
    fn join_iter(&mut self) -> Result<(), IterError> {
        self.writer.push(ClientCommand::Ping);
        
        try!( self.writer.write(&mut self.stream) );
        
        try!( self.reader.read(&mut self.stream) );
        
        match self.reader.commands() {
            Ok(comms) => {
                for comm in comms.iter() {
                    match *comm {
                        ServerCommand::GameState(game_state) => {
                            debug!("join_iter: Got game state: turn {}", game_state.turns);
                            self.game_state = game_state;
                            if game_state.turns == 1 {
                                self.state = State::Game;
                            }
                        },
                        _ => { unreachable!(); }
                    }
                }
            },
            Err(err) => {
                error!("Couldn't decode: {}", err);
                ::std::process::exit(1);
            }
            
        }
        
        Ok(())
    }
    
    fn ping_iter(&mut self) -> Result<(), IterError> {
        self.writer.push(ClientCommand::Ping);
        
        try!( self.writer.write(&mut self.stream) );
        
        try!( self.reader.read(&mut self.stream) );
        
        try!( self.reader.commands() );
        
        Ok(())
    }
    
    fn game_iter(&mut self) -> Result<(), IterError> {
        if let Some(mov) = self.move_to_write.take() {
            trace!("Sending {:?}", mov);
            let comm = ClientCommand::MakeMove(self.game_state.turns, mov);
            self.writer.push(comm);
        } else {
            trace!("No move, pinging");
            self.writer.push(ClientCommand::Ping);
        }
        
        try!( self.writer.write(&mut self.stream) );
        
        try!( self.reader.read(&mut self.stream) );
        
        match self.reader.commands() {
            Ok(comms) => {
                for comm in comms.iter() {
                    match *comm {
                        ServerCommand::GameState(game_state) => {
                            debug!("game_iter: Got game state: turn {}", game_state.turns);
                            self.game_state = game_state;
                            self.last_sent_move = None;
                            if self.game_state.winner.is_some() {
                                debug!("game_iter: Game finished.");
                                self.state = State::Idle;
                            }
                        },
                        ServerCommand::InvalidMove(turn, mov) => {
                            if turn == self.game_state.turns {
                                self.last_sent_move = None;
                            }
                            debug!("game_iter: Move {:?} at turn {} was invalid", &mov, turn);    
                        }
                        _ => { unreachable!(); }
                    }
                }
            },
            Err(err) => {
                error!("Couldn't decode: {}", err);
                ::std::process::exit(1);
            }
            
        }
        
        Ok(())
    }
}
