#![feature(std_misc, thread_sleep)]

extern crate libxdo;

use libxdo::XDo;
use std::time::Duration;

fn main() {
    let xdo = XDo::new(None).unwrap();
    for _ in 0..10 {
        xdo.key_sequence("y", Duration::milliseconds(0)).unwrap();
        xdo.key_sequence("Return", Duration::milliseconds(0)).unwrap();
        std::thread::sleep(Duration::milliseconds(100));
    }
}
