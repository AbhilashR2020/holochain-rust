#[derive(Debug, Clone, PartialEq)]
pub struct Metric {
    pub name: String,
    pub value: f64,
}

impl Metric {
    pub fn new(name: &str, value: f64) -> Self {
        Self {
            name: name.to_string(),
            value,
        }
    }
}

pub trait MetricPublisher: Sync + Send {
    fn publish(&mut self, metric: &Metric);
}

#[derive(Debug, Clone)]
pub struct StdoutMetricPublisher;

impl StdoutMetricPublisher {
    pub fn new() -> Self {
        StdoutMetricPublisher
    }
}

pub type DefaultPublisher = StdoutMetricPublisher;

impl MetricPublisher for StdoutMetricPublisher {
    fn publish(&mut self, metric: &Metric) {
        println!("{} {}", metric.name, metric.value);
    }
}

#[cfg(test)]
mod test {

    use super::*;
    #[test]
    fn can_publish_to_stdout() {
        let mut publisher = StdoutMetricPublisher;
        let metric = Metric::new("latency", 100.0);

        publisher.publish(&metric);
    }

}
