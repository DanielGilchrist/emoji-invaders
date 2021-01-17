use std::io::{Stdout};

static BODY: &'static str = ".";
static SPEED: u8 = 1;

pub struct Bullet {
  x: u16,
  y: u16
}

impl Bullet {
  pub fn new(x: u16, y: u16) -> Bullet {
    Bullet { x, y }
  }

  pub fn draw(&self, stdout: &mut Stdout) {
    unimplemented!();
  }
}
