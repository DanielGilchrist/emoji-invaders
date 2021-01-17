use entities::player::{Player};
use entities::bullet::{Bullet};

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

pub struct Game {
  stdin: AsyncReader,
  stdout: RawTerminal<Stdout>,
  pub player: Player,
  pub bullet: Option<Bullet>,
}

impl Game {
  pub fn new() -> Game {
    let stdin = async_stdin();
    let stdout = stdout().into_raw_mode().unwrap();
    let (x, y) = termion::terminal_size().unwrap();
    let x_pos: u16 = x / 2;
    let y_pos: u16 = y - (y / 8);
    let player = Player::new(x_pos, y_pos);
    let bullet = None::<Bullet>;

    Game {
      stdin,
      stdout,
      player,
      bullet,
    }
  }

  pub fn setup(&mut self) {
    write!(
      self.stdout,
      "{}{}{}",
      ClearAll,
      Goto(self.player.x_pos(), self.player.y_pos()),
      Hide
    ).unwrap();

    self.stdout.flush().unwrap();
  }

  pub fn reset(&mut self) {
    write!(
      self.stdout,
      "{}{}{}",
      ClearAll,
      Reset,
      Goto(1, 1),
    ).unwrap();
  }

  pub fn main_loop(&mut self) {
    let stdin = &mut self.stdin;

    loop {
      // keeps shit smoooooth
      sleep(Duration::from_millis(10));

      let key = stdin.keys().next();
      match key {
        Some(Ok(Key::Char('q'))) => break,
        Some(Ok(Key::Left))      => self.player.move_left(),
        Some(Ok(Key::Right))     => self.player.move_right(),
        Some(Ok(Key::Up))        => self.player.fire(game),
        None | _                 => (),
      };

      self.player.draw(&mut self.stdout);

    }
  }
}
