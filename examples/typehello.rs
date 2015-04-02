extern crate libxdo;

use libxdo::XDo;

fn main() {
    let xdo = XDo::new(None).unwrap();
    xdo.type_text("Hello, World!", 250_000).unwrap();
}
