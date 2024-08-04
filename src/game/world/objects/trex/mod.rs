use crossterm::terminal::size;

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
                (2, screen_height - 2),
                (0, screen_height - 2),
                // body
                (0, screen_height - 3),
                (1, screen_height - 3),
                (2, screen_height - 3),
                (2, screen_height - 4),
                // head
                (3, screen_height - 4),
            ],
        }
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
