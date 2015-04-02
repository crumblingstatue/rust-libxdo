extern crate libxdo;

use libxdo::XDo;

fn main() {
    let xdo = XDo::new(None).unwrap();

    for _ in 0..5 {
        xdo.click(1).unwrap();
        std::thread::sleep_ms(1000);
    }
}
