use crossterm::style::Color;

pub mod scenery;
use scenery::Scenery;

pub mod objects;
use objects::Objects;

pub struct World {
    pub screen: Screen,
    pub scenery: Scenery,
    pub objects: Objects,
    pub theme: Theme,
}

pub struct Screen {
    pub width: u16,
    pub height: u16,
}

pub struct Theme {
    pub background: Color,
    pub trex: Color,
    pub collided_trex: Color,
    pub trex_eye: Color,
    pub cloud: Color,
    pub cactus: Color,
}