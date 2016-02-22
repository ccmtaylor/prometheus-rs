use time;

pub trait Counter {
    fn inc(&mut self) {
        self.inc_by(1.0);
    }
    fn inc_by(&mut self, amt: f64);
}

pub trait Gauge {
    fn inc(&mut self) {
        self.inc_by(1.0);
    }
    fn inc_by(&mut self, amt: f64);

    fn dec(&mut self) {
        self.dec_by(1.0);
    }
    fn dec_by(&mut self, amt: f64) {
        self.inc_by(-amt);
    }
    fn set(&mut self, val: f64);
    fn timer<'a>(&'a mut self) -> GaugeTimer<'a, Self>
        where Self: Sized {
        println!("starting timer");
        GaugeTimer {
            gauge: self,
            start_s: time::precise_time_s(),
        }
    }
}


pub struct GaugeTimer<'a, G: Gauge + 'a> {
    gauge: &'a mut G,
    start_s: f64,
}
impl<'a, G> Drop for GaugeTimer<'a, G> where G: Gauge {
    fn drop(&mut self) {
        let delta = time::precise_time_s() - self.start_s;
        println!("stopping timer, {}", delta);
        self.gauge.set(delta);
    }
}

trait Histogram {
    fn observe(&mut self, val: f64);
}

#[cfg(test)]
mod tests {
    use super::{Counter, Gauge};

    impl Counter for f64 {
        fn inc_by(&mut self, amt: f64) {*self += amt;}
    }

    #[test]
    fn counter() {
        let mut c = 0.0;
        Counter::inc(&mut c);
        assert_eq!(c, 1.0);
    }

    impl Gauge for f64 {
        fn inc_by(&mut self, amt: f64) {*self += amt;}
        fn set(&mut self, val: f64) {*self = val;}
    }

    #[test]
    fn gauge() {
        let mut g = 0.0;
        Gauge::inc(&mut g);
        assert_eq!(g, 1.0);
        g.dec_by(0.5);
        assert_eq!(g, 0.5);
        g.set(7.0);
        assert_eq!(g, 7.0);
    }

    #[test]
    fn gauge_timer() {
        use std::thread;
        use std::time::Duration;

        let mut g = 0.0;
        {
            let _ = g.timer();
            thread::sleep(Duration::new(10, 0));
        }
        println!("timer is {}", g);
        assert_eq!(0.0, g);
    }
}