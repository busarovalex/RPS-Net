#![feature(split_off, socket_timeout, vec_push_all)]

extern crate rps;
extern crate bincode;
extern crate rustc_serialize;
#[macro_use] extern crate log;

mod player;

pub mod server;
pub mod client;
pub mod server_commands;
pub mod client_commands;
pub mod game_state;
pub mod ser_de;
