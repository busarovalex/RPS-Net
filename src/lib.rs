#![feature(split_off, socket_timeout, vec_push_all)]

extern crate rps;
extern crate bincode;
extern crate rustc_serialize;
#[macro_use] extern crate log;
extern crate rand;

mod player;

pub mod room;
pub mod client;
pub mod commands;
pub mod game_state;
pub mod ser_de;
