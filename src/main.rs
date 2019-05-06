extern crate termion;

mod entities;
use entities::player::{Player};

use std::io::{Write, Stdout, stdout};
use std::time::{Duration};
use std::thread::{sleep};

use termion::{AsyncReader, async_stdin};
use termion::input::{TermRead};
use termion::raw::{IntoRawMode, RawTerminal};
use termion::event::{Key};
use termion::cursor::{Goto, Hide};
use termion::clear::{All as ClearAll};
use termion::style::{Reset};

struct Game {
  stdin: AsyncReader,
  stdout: RawTerminal<Stdout>,
  player: Player,
}

impl Game {
  pub fn new() -> Game {
    let stdin = async_stdin();
    let stdout = stdout().into_raw_mode().unwrap();
    let (x, y) = termion::terminal_size().unwrap();
    let x_pos: u16 = x / 2;
    let y_pos: u16 = y - (y / 8);
    let player = Player::new(x_pos, y_pos);

    Game {
      stdin,
      stdout,
      player,
    }
  }

  fn setup(&mut self) {
    write!(
      self.stdout,
      "{}{}{}",
      ClearAll,
      Goto(self.player.x_pos(), self.player.y_pos()),
      Hide
    ).unwrap();

    self.stdout.flush().unwrap();
  }

  fn reset(&mut self) {
    write!(
      self.stdout,
      "{}{}{}",
      ClearAll,
      Reset,
      Goto(1, 1),
    ).unwrap();
  }

  fn main_loop(&mut self) {
    let stdin = &mut self.stdin;

    loop {
      // keeps shit smoooooth
      sleep(Duration::from_millis(10));

      let key = stdin.keys().next();
      match key {
        Some(Ok(Key::Char('q'))) => break,
        Some(Ok(Key::Left))      => self.player.move_left(),
        Some(Ok(Key::Right))     => self.player.move_right(),
        None                 => (),
        _                    => ()
      };

      self.player.draw(&mut self.stdout);
    }
  }
}

fn main() {
  let mut game = Game::new();

  game.setup();
  game.main_loop();
  game.reset();
}
