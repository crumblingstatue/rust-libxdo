extern crate libxdo;

use libxdo::XDo;

fn main() {
    let xdo = XDo::new(None).unwrap();
    xdo.mouse_down(1).unwrap();
    xdo.move_mouse_relative(150, 150).unwrap();
    xdo.mouse_up(1).unwrap();
}
