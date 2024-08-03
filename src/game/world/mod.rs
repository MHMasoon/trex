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
    pub message: Color,
    pub message_background: Color,
}

impl World {
    pub fn initiate(&mut self) {
        for _ in 0..self.screen.width {
            self.scenery.road.generate_top_line(); 
            self.scenery.road.generate_bottom_line();
            self.scenery.road.generate_ground();
        }
    }

    pub fn next_frame(&mut self) {
        self.scenery.road.generate_top_line(); 
        self.scenery.road.generate_bottom_line();
        self.scenery.road.generate_ground();
        self.objects.cactuses.shift();
        self.objects.cactuses.generate(self.screen.width, self.screen.height);
        self.objects.trex.check_and_shift();
    }
}