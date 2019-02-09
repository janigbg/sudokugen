#![feature(test)]

extern crate rand;
extern crate rand_pcg;
#[macro_use]
extern crate log;
extern crate env_logger;

pub mod board;
pub mod generator;
pub mod group;
pub mod solver;
