use crossterm::terminal::size;

pub struct Screen {
    pub width: u16,
    pub height: u16,
}

impl Default for Screen {
    fn default() -> Self {
        Screen {
            width: size().unwrap().0,
            height: size().unwrap().1,
        }
    }
}