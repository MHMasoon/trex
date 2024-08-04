use rand::Rng;

#[derive(Default)]
pub struct Cactuses {
    pub pixels: Vec<(u16, u16)>,
    pub cactus_distance: u8,
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
