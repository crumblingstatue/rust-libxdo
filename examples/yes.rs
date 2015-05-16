extern crate libxdo;

use libxdo::XDo;

fn main() {
    let xdo = XDo::new(None).unwrap();
    for _ in 0..10 {
        xdo.send_keysequence("y", 0).unwrap();
        xdo.send_keysequence("Return", 0).unwrap();
        std::thread::sleep_ms(100);
    }
}
