use crossterm::terminal::size;
use rand::Rng;

pub struct Objects {
    pub trex: Trex,
    pub cactuses: Cactuses,
}

pub struct Trex {
    pub pixels: [(u16, u16); 7],
    pub status: TrexStatus,
    pub height: u16,
    pub max_height: u16,
}

impl Default for Trex {
    fn default() -> Self {
        let screen_height: u16 = size().unwrap().1;
        Trex {
            status: TrexStatus::OnGround,
            height: 0,
            max_height: 10,
            pixels: [
                // legs
                (2, screen_height - 1),
                (0, screen_height - 1),
                // body
                (0, screen_height - 2),
                (1, screen_height - 2),
                (2, screen_height - 2),
                (2, screen_height - 3),
                // head
                (3, screen_height - 3),
            ],
        }
    }
}

pub struct Cactuses {
    pub pixels: Vec<(u16, u16)>,
    pub cactus_distance: u8,
}

#[derive(PartialEq)]
pub enum TrexStatus {
    OnGround,
    Rising,
    Falling,
}

#[derive(PartialEq)]
pub enum TrexMoveDirection {
    Up,
    Down,
}

impl Cactuses {
    pub fn shift(&mut self) {
        for pixel in self.pixels.iter_mut() {
            pixel.0 -= 1;
        }
    }

    pub fn generate(&mut self, screen_width: u16, screen_height: u16) {
        let mut rng = rand::thread_rng();
        if self.cactus_distance == 0 {
        let cactus_form: u8 = rng.gen_range(1..=3);
        let mut cactus_pixels: Vec<(u16, u16)> = Vec::new();
        match cactus_form {
            1 => {
                cactus_pixels = vec![
                    (screen_width + 1, screen_height - 2),
                    (screen_width + 1, screen_height - 3),
                    (screen_width + 1, screen_height - 4),
                    (screen_width + 1, screen_height - 5),
                    (screen_width + 2, screen_height - 3),
                    (screen_width + 3, screen_height - 3),
                    (screen_width + 3, screen_height - 4),
                ];
            },
            2 => {
                cactus_pixels = vec![
                    (screen_width + 4, screen_height - 2),
                    (screen_width + 4, screen_height - 3),
                    (screen_width + 4, screen_height - 4),
                    (screen_width + 4, screen_height - 5),
                    (screen_width + 3, screen_height - 3),
                    (screen_width + 2, screen_height - 3),
                    (screen_width + 2, screen_height - 4),
                ];
            },
            3 => {
                cactus_pixels = vec![
                    (screen_width + 4, screen_height - 2),
                    (screen_width + 4, screen_height - 3),
                    (screen_width + 4, screen_height - 4),
                    (screen_width + 4, screen_height - 5),
                    (screen_width + 3, screen_height - 3),
                    (screen_width + 2, screen_height - 3),
                    (screen_width + 2, screen_height - 4),
                    (screen_width + 5, screen_height - 3),
                    (screen_width + 6, screen_height - 3),
                    (screen_width + 6, screen_height - 4),
                ];
            },
            _ => (),
        }
        self.pixels.extend(cactus_pixels);
        self.cactus_distance = rng.gen_range(100..200);
    }
    self.cactus_distance -= 1;
    }
}

impl Trex {
    pub fn check_and_shift(&mut self) {
        match self.status {
            TrexStatus::Rising => {
                if self.height < self.max_height {
                    self.shift(TrexMoveDirection::Up);
                } else {
                    self.status = TrexStatus::Falling;
                }
            },
            TrexStatus::Falling => {
                if self.height > 0 {
                    self.shift(TrexMoveDirection::Down);
                } else {
                    self.status = TrexStatus::OnGround;
                }
            },
            _ => {
            }
        }
    }

    fn shift(&mut self, direction: TrexMoveDirection) {
           if direction  == TrexMoveDirection::Up {
            self.height += 1;
        } else {
            self.height -= 1;
        }

        for pixel in self.pixels.iter_mut() {
            match direction {
                TrexMoveDirection::Up => {
                    pixel.1 -= 1;
                },
                TrexMoveDirection::Down => {
                    pixel.1 += 1;
                },
            }
        }
    }
}