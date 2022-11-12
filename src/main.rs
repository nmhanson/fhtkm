#![feature(is_some_with)]

use game::state::GameState;

mod game;
mod render;
fn main() -> std::io::Result<()> {
    let mut gs = GameState::init();
    gs.main_loop()
}
