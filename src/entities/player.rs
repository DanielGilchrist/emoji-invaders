extern crate termion;

use termion::cursor::{Goto};
use termion::clear::{CurrentLine};
use std::io::{Write, Stdout};

static BODY: &'static str = "/\\";
static SPEED: u8 = 1;

pub struct Player {
  x: u16,
  y: u16,
}

impl Player {
  pub fn new(x: u16, y: u16) -> Player {
    Player { x, y }
  }

  pub fn draw(&self, stdout: &mut Stdout) {
    write!(
      stdout,
      "{}{}{}",
      Goto(self.x, self.y),
      CurrentLine,
      BODY
    ).unwrap();

    stdout.flush().unwrap();
  }

  pub fn x_pos(&self) -> u16 {
    self.x
  }

  pub fn y_pos(&self) -> u16 {
    self.y
  }

  pub fn move_left(&mut self) {
    self.x -= SPEED as u16;
  }

  pub fn move_right(&mut self) {
    self.x += SPEED as u16;
  }
}
