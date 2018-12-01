extern crate termion;

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use std::io::{Write, stdout, stdin};

static PLAYER: &'static str = "/\\";

fn main() {
  let stdin = stdin();
  let mut stdout = stdout().into_raw_mode().unwrap();

  let (x, y) = termion::terminal_size().unwrap();

  let mut x_pos: u16 = x / 2;
  let y_pos: u16 = y - (y / 8);

  write!(stdout, "{}{}{}",
    termion::clear::All,
    termion::cursor::Goto(x_pos, y_pos),
    termion::cursor::Hide).unwrap();

  stdout.flush().unwrap();

  for c in stdin.keys() {
    write!(stdout, "{}{}{}",
      termion::cursor::Goto(x_pos, y_pos),
      termion::clear::CurrentLine,
      PLAYER).unwrap();

    stdout.flush().unwrap();

    match c.unwrap() {
      Key::Char('q') => break,
      Key::Left      => move_c(&mut x_pos, false),
      Key::Right     => move_c(&mut x_pos, true),
      _              => ()
    };
  }

  write!(stdout, "{}", termion::cursor::Show).unwrap();
}

fn move_c(pos: &mut u16, moving_right: bool) {
  if moving_right {
    *pos += 1;
  } else {
    *pos -= 1;
  }
}
