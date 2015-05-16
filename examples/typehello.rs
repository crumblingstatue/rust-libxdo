extern crate libxdo;

use libxdo::XDo;

fn main() {
    let xdo = XDo::new(None).unwrap();
    xdo.enter_text("Hello, World!", 250_000).unwrap();
}
