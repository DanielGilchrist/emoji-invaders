extern crate termion;

mod entities;
use entities::player::{Player};

use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::event::Key;
use termion::cursor::{Goto, Show, Hide};
use termion::clear::{All};
use std::io::{Write, Stdout, stdout, stdin};

fn main() {
  let stdin = stdin();
  let mut stdout = stdout().into_raw_mode().unwrap();

  let (x, y) = termion::terminal_size().unwrap();

  let x_pos: u16 = x / 2;
  let y_pos: u16 = y - (y / 8);

  let mut player = Player::new(x_pos, y_pos);

  setup(&player, &mut stdout);

  for c in stdin.keys() {
    match c.unwrap() {
      Key::Char('q') => break,
      Key::Left      => player.move_left(),
      Key::Right     => player.move_right(),
      _              => ()
    };

    player.draw(&mut stdout);
  }

  reset(&mut stdout);
}

fn setup(player: &Player, stdout: &mut Stdout) {
  write!(
    stdout,
    "{}{}{}",
    All,
    Goto(player.x_pos(), player.y_pos()),
    Hide
  ).unwrap();

  stdout.flush().unwrap();

  player.draw(stdout);
}

fn reset(stdout: &mut Stdout) {
  write!(
    stdout,
    "{}{}{}",
    All,
    Goto(0, 0),
    Show
  ).unwrap();
}
