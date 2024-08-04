use rand::Rng;

#[derive(Default)]
pub struct Scenery {
    pub road: Road,
    pub clouds: Clouds,
}

#[derive(Default)]
pub struct Road {
    pub top_stone_distance: u16,
    pub bottom_stone_distance: u16,
    pub grain_distance: u16,
    pub top_line: Vec<LineStatus>,
    pub bottom_line: Vec<LineStatus>,
    pub ground: Vec<bool>,
}

#[derive(Default)]
pub struct Clouds {}


pub enum LineStatus {
    Line,
    StoneStart,
    StoneMiddle,
    StoneEnd,
}

impl Road {
    pub fn generate_top_line(&mut self) {
        let mut rng = rand::thread_rng();
        match self.top_stone_distance {
            2 => self.top_line.push(LineStatus::StoneStart),
            1 => self.top_line.push(LineStatus::StoneMiddle),
            0 => {
                self.top_line.push(LineStatus::StoneEnd);
                self.top_stone_distance = rng.gen_range(50..100);
            },
            _ => self.top_line.push(LineStatus::Line),
        }
        self.top_stone_distance -= 1;
    }
    
    pub fn generate_bottom_line(&mut self) {
        let mut rng = rand::thread_rng();
        match self.bottom_stone_distance {
            2 => self.bottom_line.push(LineStatus::StoneStart),
            1 => self.bottom_line.push(LineStatus::StoneMiddle),
            0 => {
                self.bottom_line.push(LineStatus::StoneEnd);
                self.bottom_stone_distance = rng.gen_range(50..100);
            },
            _ => self.bottom_line.push(LineStatus::Line),
        }
        self.bottom_stone_distance -= 1;
    }

    pub fn generate_ground(&mut self) {
        let mut rng = rand::thread_rng();
        if self.grain_distance == 0 {
            self.ground.push(true);
            self.grain_distance = rng.gen_range(10..20);
        } else {
            self.ground.push(false);
        }
        self.grain_distance -= 1;
    }
}
