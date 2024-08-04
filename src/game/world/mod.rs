pub mod scenery;
use scenery::Scenery;

pub mod objects;
use objects::Objects;

pub mod theme;
pub use theme::Theme;

pub mod screen;
pub use screen::Screen;

#[derive(Default)]
pub struct World {
    pub screen: Screen,
    pub scenery: Scenery,
    pub objects: Objects,
    pub theme: Theme,
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

    pub fn reset(&mut self) {
        *self = Default::default();
    }
}
