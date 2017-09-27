extern crate atomic_immut;
#[macro_use]
extern crate trackable;

pub use collector::Collector;
pub use error::{Error, ErrorKind};
pub use metric::Metric;
pub use registry::{default_registry, CollectorRegistry};

// pub mod format;
// pub mod metric;
pub mod label;
pub mod metrics;
// pub mod types;
pub mod timestamp;

mod atomic;
mod collector;
mod error;
mod metric;
mod registry;

pub type Result<T> = std::result::Result<T, Error>;
