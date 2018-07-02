extern crate termion;

use termion::raw::IntoRawMode;
use std::io::{Write, stdout};

fn main() {
  let mut stdout = stdout().into_raw_mode().unwrap();
  writeln!(stdout, "Hello, world!").unwrap();
}
