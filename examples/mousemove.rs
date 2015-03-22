extern crate libxdo;

use libxdo::XDo;

fn main() {
    let xdo = XDo::new(None).unwrap();
    xdo.move_mouse(0, 0, 0).unwrap();
}
