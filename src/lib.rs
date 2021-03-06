//! Client library for exposing [prometheus][prometheus] metrics.
//!
//! Currently this crate only support text format for exposing metrics.
//!
//! [prometheus]: https://prometheus.io/
//!
//! # Examples
//!
//! ```
//! use prometrics::default_gatherer;
//! use prometrics::metrics::{CounterBuilder, GaugeBuilder};
//!
//! let mut counter = CounterBuilder::new("count")
//!     .default_registry()
//!     .finish()
//!     .unwrap();
//! let mut gauge = GaugeBuilder::new("gauge")
//!     .label("foo", "bar")
//!     .default_registry()
//!     .finish()
//!     .unwrap();
//!
//!  counter.increment();
//!  gauge.set(12.3);
//!
//!  let metrics = default_gatherer().lock().unwrap().gather();
//!  assert_eq!(
//!     metrics
//!         .into_iter()
//!         .map(|m| format!("\n{}", m))
//!         .collect::<Vec<_>>()
//!         .join(""),
//!     format!("\n{}\n{}\n\n{}\n{}\n",
//!             "# TYPE count counter",
//!             "count 1",
//!             "# TYPE gauge gauge",
//!             "gauge{foo=\"bar\"} 12.3"));
//! ```
//!
//! # References
//!
//! - [Data model](https://prometheus.io/docs/concepts/data_model/)
//! - [Metric types](https://prometheus.io/docs/concepts/metric_types/)
//! - [Writing client libraries](https://prometheus.io/docs/instrumenting/writing_clientlibs/)
//! - [Exposition formats](https://prometheus.io/docs/instrumenting/exposition_formats/)
#![warn(missing_docs)]
extern crate atomic_immut;
#[macro_use]
extern crate lazy_static;
#[cfg(target_os = "linux")]
extern crate libc;
#[cfg(target_os = "linux")]
extern crate procinfo;
#[macro_use]
extern crate trackable;

pub use collect::Collect;
pub use error::{Error, ErrorKind};
pub use registry::{default_registry, default_gatherer, Registry, Gatherer};

pub mod bucket;
pub mod label;
pub mod metric;
pub mod metrics;
pub mod quantile;
pub mod timestamp;

mod atomic;
mod collect;
mod error;
mod registry;

/// This crate specific `Result` type.
pub type Result<T> = std::result::Result<T, Error>;

#[cfg(test)]
mod test {
    use metrics::{CounterBuilder, GaugeBuilder};
    use super::*;

    #[test]
    fn it_works() {
        let mut counter = CounterBuilder::new("count")
            .default_registry()
            .finish()
            .unwrap();
        let mut gauge = GaugeBuilder::new("gauge")
            .label("foo", "bar")
            .default_registry()
            .finish()
            .unwrap();

        counter.increment();
        gauge.set(12.3);

        let metrics = default_gatherer().lock().unwrap().gather();
        assert_eq!(
            metrics
                .into_iter()
                .map(|m| format!("\n{}", m))
                .collect::<Vec<_>>()
                .join(""),
            r#"
# TYPE count counter
count 1

# TYPE gauge gauge
gauge{foo="bar"} 12.3
"#
        );
    }
}
