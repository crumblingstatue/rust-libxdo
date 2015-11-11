extern crate libxdo;

use libxdo::XDo;
use std::time::Duration;

fn main() {
    let xdo = XDo::new(None).unwrap();

    for _ in 0..5 {
        xdo.click(1).unwrap();
        std::thread::sleep(Duration::from_secs(1));
    }
}
