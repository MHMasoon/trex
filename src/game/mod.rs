use std::io::Stdout;
use rand::rngs::ThreadRng;

pub mod world;
use world::World;

pub struct Game {
    pub world: World,
    pub scores: Scores,
    pub status: GameStatus,
    pub utils: Utils,
}

pub struct Scores {
    pub highest: u16,
    pub current: u16,
}

pub struct Utils {
    pub rng: ThreadRng,
    pub stdout: Stdout,
}

#[derive(PartialEq)]
pub enum GameStatus {
    Beginning,
    Paused,
    Running,
    Over,
    Closed,
}

impl GameStatus {
    pub fn message(&self) -> &str {
        match self {
            GameStatus::Beginning => "Press Space to Start!",
            GameStatus::Paused => "Game is Paused",
            GameStatus::Over => "Game is Over",
            _ => "",
        }
    }
}