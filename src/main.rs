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

  flush(&mut stdout);

  // init player
  draw_player(&mut stdout, x_pos, y_pos);

  for c in stdin.keys() {
    match c.unwrap() {
      Key::Char('q') => break,
      Key::Left      => move_left(&mut x_pos),
      Key::Right     => move_right(&mut x_pos),
      _              => ()
    };

    draw_player(&mut stdout, x_pos, y_pos);
  }

  // reset terminal
  write!(stdout, "{}{}{}",
    termion::clear::All,
    termion::cursor::Goto(0, 0),
    termion::cursor::Show).unwrap();
}

fn move_left(pos: &mut u16) {
  *pos -= 1;
}

fn move_right(pos: &mut u16) {
  *pos += 1;
}

fn draw_player(stdout: &mut std::io::Stdout, x_pos: u16, y_pos: u16) {
  write!(stdout, "{}{}{}",
    termion::cursor::Goto(x_pos, y_pos),
    termion::clear::CurrentLine,
    PLAYER).unwrap();

  flush(stdout);
}

fn flush(stdout: &mut std::io::Stdout) {
  stdout.flush().unwrap();
}
