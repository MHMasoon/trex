pub struct Objects {
    pub trex: Trex,
    pub cactuses: Cactuses,
}

pub struct Trex {
    pub pixels: [(u16, u16); 7],
    pub status: TrexStatus,
    pub height: u16,
    pub max_height: u16,
    pub origin: (u16, u16), // (width, height)
}

pub struct Cactuses {
    pub pixels: Vec<(u16, u16)>,
    pub next_cactus_distance: u8,
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