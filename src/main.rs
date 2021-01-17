extern crate termion;

mod entities;
use entities::game::{Game};

fn main() {
  let mut game = Game::new();

  game.setup();
  game.main_loop();
  game.reset();
}
