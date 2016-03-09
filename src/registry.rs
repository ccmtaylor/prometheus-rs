use std::collections::BTreeMap; 

use metrics;

struct Sample<'a> {
    name: &'a str,
    labels: &'a BTreeMap<&'a str, &'a str>,
    value: f64,
}

enum Kind {
    Counter,
    Gauge,
    Histogram,
}

struct Metric<'a> {
    name: &'a str,
    kind: Kind,
    help: &'a str,
    samples: Vec<Sample<'a>>
}

trait Collector {
    fn as_metric(&self) -> Metric;
}

struct Counters<'a> {
    name: &'a str,
    help: &'a str,
    children: BTreeMap<BTreeMap<&'a str,&'a str>, Counter>,
}

impl<'a> Counters<'a> {
    pub fn with_labels(&mut self, labels: BTreeMap<&'a str, &'a str>) -> &Counter {
        let child = Counter {
            value: 0.0,
        };
        self.children.entry(labels).or_insert(child)
    }
}

impl<'a> Collector for Counters<'a> {
   fn as_metric(&self) -> Metric {
        Metric {
            name: self.name,
            kind: Kind::Counter,
            help: self.help,
            samples: self.children
                .iter()
                .map(|(labels, child)| Sample {
                    name: self.name,
                    labels: labels,
                    value: child.value,
                })
                .collect(),
        }
    }
}

pub struct Counter {
    value: f64,
}

impl<'a> metrics::Counter for Counter {
    fn inc_by(&mut self, amount: f64) {
        self.value += amount;
    }
}

