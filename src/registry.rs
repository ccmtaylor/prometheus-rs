use std::collections::BTreeMap; 

use metrics;

trait Metric<S: Sample> {
    fn name(&self) -> &str;
    fn help(&self) -> &str;
    fn kind(&self) -> Kind;
    fn samples(&self) -> Box<Iterator<Item=&Counter>>;
}

trait Sample {
    fn name(&self) -> &str;
    fn labels(&self) -> Box<Iterator<Item=&(&str, &str)>>;
    fn value(&self) -> f64;
}

enum Kind {
    Counter,
    Gauge,
    Histogram,
}

struct Counters {
    name: String,
    help: String,
    children: BTreeMap<BTreeMap<&'static str, String>, Counter>,
}

impl Counters {
    pub fn with_labels(&mut self, labels: BTreeMap<&'static str, String>) -> &Counter {
        let child = Counter {
            value: 0.0,
        };
        self.children.entry(labels).or_insert(child)
    }
}

impl Metric<Counter> for Counters {
    fn name(&self) -> &str {&self.name}
    fn help(&self) -> &str {&self.help}
    fn kind(&self) -> Kind {Kind::Counter}
    fn samples(&self) -> Box<Iterator<Item=&Counter>> {&self.children.values()}
}

pub struct Counter {
    name: String,
    value: f64,
    labels: BTreeMap<&'static str, String>,
}

impl Sample for Counter {
    fn name(&self) -> &str {&self.name}
    fn value(&self) -> f64 {self.value}
    fn labels(&self) -> Box<Iterator<Item=&(&str, &str)>> {Box::from(self.labels.iter().map(|k, s| (k, &s[..])))}
}

impl<'a> metrics::Counter for Counter {
    fn inc_by(&mut self, amount: f64) {
        self.value += amount;
    }
}

