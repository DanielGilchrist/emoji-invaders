extern crate termion;

use termion::cursor::{Goto};
use termion::clear::{CurrentLine, All as ClearAll};
use std::io::{Write, Stdout};

static BODY: &str = "•";
static SPEED: u8 = 1;

pub struct Bullet {
  x:     u16,
  y:     u16,
  index: usize
}

impl Bullet {
  pub fn new(x: u16, y: u16, index: usize) -> Bullet {
    Bullet {
      x,
      y,
      index
    }
  }

  pub fn index(&self) -> usize {
    self.index
  }

  pub fn should_destroy(&self) -> bool {
    let (_max_x, max_y) = termion::terminal_size().unwrap();
    self.y == max_y - 10
  }

  pub fn draw(&mut self, stdout: &mut Stdout) {
    self.update();

    write!(
      stdout,
      "{}{}{}{}",
      ClearAll,
      Goto(self.x, self.y),
      CurrentLine,
      BODY
    ).unwrap();

    stdout.flush().unwrap();
  }

  pub fn update(&mut self) {
    self.y -= SPEED as u16;
  }
}
