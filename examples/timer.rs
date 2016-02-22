extern crate prometheus;

use prometheus::metrics;
use prometheus::metrics::{Gauge};

#[derive(PartialEq, Debug)]
struct G(f64);

impl metrics::Gauge for G {
    fn inc_by(&mut self, amt: f64) {self.0 += amt;}
    fn set(&mut self, val: f64) {self.0 = val;}
}

fn main() {
        use std::thread;
        use std::time::Duration;

        let mut g = G(0.0);
        {
            let _t = g.timer();
            thread::sleep(Duration::from_millis(100));
        }
        println!("timer is {:?}", g);
}
