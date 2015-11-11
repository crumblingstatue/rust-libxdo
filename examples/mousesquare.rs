extern crate libxdo;

use libxdo::XDo;
use std::time::Duration;

fn move_it(xdo: &XDo, rel_x: i32, rel_y: i32, times: i32) {
    for _ in 0..times {
        xdo.move_mouse_relative(rel_x, rel_y).unwrap();
        std::thread::sleep(Duration::from_millis(10));
    }
}

fn main() {
    let xdo = XDo::new(None).unwrap();
    move_it(&xdo, 10, 0, 20);
    move_it(&xdo, 0, 10, 20);
    move_it(&xdo, -10, 0, 20);
    move_it(&xdo, 0, -10, 20);
}
