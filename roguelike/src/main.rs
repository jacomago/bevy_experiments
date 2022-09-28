#![warn(clippy::all, clippy::pedantic)]

mod map;

mod prelude {
    pub use bracket_lib::prelude::*;
    pub const SCREEN_WIDTH: usize = 80;
    pub const SCREEN_HEIGHT: usize = 50;
    pub use crate::map::*;
}

use prelude::*;

fn main() {
    println!("Hello, world!");
}
