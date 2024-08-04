use crossterm::style::Color;
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

impl Default for Theme {
    fn default() -> Self {
        Theme {
            background: Color::Blue,
            trex: Color::Green,
            trex_eye: Color::White,
            collided_trex: Color::Red,
            cloud: Color::Cyan,
            cactus: Color::Green,
            message: Color::Cyan,
            message_background: Color::Black,
        }
    }
}