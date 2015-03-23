#![feature(std_misc)]

extern crate libxdo;

use libxdo::XDo;
use std::time::Duration;

fn main() {
    let xdo = XDo::new(None).unwrap();
    xdo.type_text("Hello, World!", Duration::milliseconds(250)).unwrap();
}
