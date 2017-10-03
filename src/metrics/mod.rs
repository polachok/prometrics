//! Metrics and related types.
//!
//! # References
//!
//! - [Metric types](https://prometheus.io/docs/concepts/metric_types/)
pub use self::counter::{Counter, CounterBuilder, CounterCollector};
pub use self::gauge::{Gauge, GaugeBuilder, GaugeCollector};
pub use self::histogram::{Histogram, HistogramBuilder, HistogramCollector};
pub use self::summary::{Summary, SummaryBuilder, SummaryCollector};

mod counter;
mod gauge;
mod histogram;
mod summary;
