use std::collections::BTreeMap; 

use metrics;

struct Sample<'a> {
    name: &'a str,
    labels: BTreeMap<&'a str, &'a str>,
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

impl<'a> From<Counters<'a>> for Metric<'a> {
   fn from(counters: Counters<'a>) -> Metric<'a> {
        Metric {
            name: counters.name,
            kind: Kind::Counter,
            help: counters.help,
            samples: counters.children
                .iter()
                .map(|(labels, child)| Sample {
                    name: counters.name,
                    labels: labels.clone(),
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

