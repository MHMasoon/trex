pub mod trex;
pub use trex::Trex;

mod cactuses;
pub use cactuses::Cactuses;

#[derive(Default)]
pub struct Objects {
    pub trex: Trex,
    pub cactuses: Cactuses,
}
