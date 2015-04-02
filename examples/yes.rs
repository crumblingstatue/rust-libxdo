extern crate libxdo;

use libxdo::XDo;

fn main() {
    let xdo = XDo::new(None).unwrap();
    for _ in 0..10 {
        xdo.key_sequence("y", 0).unwrap();
        xdo.key_sequence("Return", 0).unwrap();
        std::thread::sleep_ms(100);
    }
}
