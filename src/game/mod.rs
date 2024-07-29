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

pub enum GameStatus {
    Paused,
    Running,
    Over,
    Closed,
}