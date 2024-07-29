pub struct Scenery {
    pub road: Road,
    pub clouds: Clouds,
}

pub struct Road {
    pub next_top_stone_distance: u16,
    pub next_bottom_stone_distance: u16,
    pub next_grain_distance: u16,
    pub top_line: Vec<LineStatus>,
    pub bottom_line: Vec<LineStatus>,
    pub ground: Vec<bool>,
}

pub struct Clouds {

}

pub enum LineStatus {
    Line,
    StoneStart,
    StoneMiddle,
    StoneEnd,
}