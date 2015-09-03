use game_state::GameState;
use ser_de::{ser, de};
use client_commands::ClientCommand;
use server_commands::ServerCommand;
use rps::moves::Move;

use std::net::TcpStream;
use std::io::{Read, Write, Error};
use std::time::Duration;

pub const TIMEOUT_MSEC: u32 = 50;

pub struct Client {
    game_state: GameState,
    stream: TcpStream,
    de_buf: Vec<u8>,
    buf: [u8; 4098],
    to_write: Vec<u8>,
    move_to_write: Option<Move>,
}

impl Client {
    pub fn connect(address: &str) -> Result<Client, Error> {
        let mut stream = try!( TcpStream::connect(address) );
        try!( stream.set_read_timeout(Some(Duration::new(0, TIMEOUT_MSEC * 1000))) );
        try!( stream.set_write_timeout(Some(Duration::new(0, TIMEOUT_MSEC * 1000))) );
        let (game_state, de_buf) = Self::join_game(&mut stream);
        
        Ok(Client {
            game_state: game_state,
            stream: stream,   
            de_buf: de_buf,
            buf: [0; 4098], 
            to_write: Vec::new(),
            move_to_write: None,
        })    
    }
    
    fn join_game(stream: &mut TcpStream) -> (GameState, Vec<u8>) {
        debug!("Joining game");
        let buf = ser(ClientCommand::JoinNewGame);
        let mut from = 0;
        loop {
            match stream.write(&buf[from..]) {
                Ok(written) => {
                    if from + written == buf.len() { break; } else { from += written; }
                },
                Err(_) => {},
            }
        }
        debug!("Join command was sent");
        let ping = ser(ClientCommand::Ping);
        let mut ping_written = 0;
        let mut de_buf = Vec::<u8>::new();
        let mut buf = [0u8; 4098];
        let game_state;
        loop {
            trace!("Writing to stream");
            match stream.write(&ping[ping_written..]) {
                Ok(written) => {
                    if ping_written + written == ping.len() { ping_written = 0; } else { ping_written += written; }
                },
                Err(_) => {},
            }
            
            trace!("Reading from stream");
            if let Ok(read) = stream.read(&mut buf[..]) {
                de_buf.push_all(&buf[0..read]);
            }
            if let Some(comm) = de(&mut de_buf) {
                trace!("Received command: {:?}", &comm);
                match comm {
                    ServerCommand::GameState(state) => { game_state = state; break;},
                    _ => {},
                }
            }
            ::std::thread::sleep_ms(TIMEOUT_MSEC);
        }
        (game_state, de_buf)
    }
    
    pub fn game_state(&self) -> GameState {
        self.game_state
    }
    
    pub fn send_move(&mut self, movement: Move) {
        self.move_to_write = Some(movement);
    }
    
    pub fn one_cycle(&mut self) {
        trace!("New cycle");
        if let Some(mov) = self.move_to_write.take() {
            trace!("Sending {:?}", mov);
            let comm = ClientCommand::MakeMove(self.game_state.turns, mov);
            self.to_write.push_all(&ser(comm)[..]);
        } else {
            trace!("No move, pinging");
            self.to_write.push_all(&ser(ClientCommand::Ping)[..]);
        }
        
        match self.stream.write(&self.to_write[..]) {
            Ok(written) => {
                trace!("Written, {}", written);
                let mut still = self.to_write.split_off(written);
                ::std::mem::swap(&mut self.to_write, &mut still);
            },
            Err(err) => { error!("Didn't write: {}", err); },
        }
        
        if let Ok(read) = self.stream.read(&mut self.buf[..]) {
            self.de_buf.push_all(&self.buf[0..read]);
        }
        
        while let Some(comm) = de(&mut self.de_buf) {
            match comm {
                ServerCommand::GameState(game_state) => {
                    debug!("Got game state: turn {}", game_state.turns);
                    self.game_state = game_state;
                },
                ServerCommand::InvalidMove(turn, mov) => {
                    debug!("Move {:?} at turn {} was invalid", &mov, turn);    
                }
                _ => { unreachable!(); }
            }
        }
    }
}
