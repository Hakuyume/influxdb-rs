//! Rust implementation of [`InfluxDB's line protocol`](https://docs.influxdata.com/influxdb/v2.0/reference/syntax/line-protocol/)
//!
//! # Example
//!
//! ```
//! use influxdb_line_protocol::{DataPoint, FieldValue};
//!
//! let data_point = DataPoint {
//!     measurement: "myMeasurement",
//!     tag_set: vec![("tag1", "value1"), ("tag2", "value2")],
//!     field_set: vec![("fieldKey", FieldValue::String("fieldValue"))],
//!     timestamp: Some(1556813561098000000),
//! };
//! print!("{}", data_point.into_string().unwrap());
//! ```

#![cfg_attr(not(feature = "std"), no_std)]

mod data_point;
mod error;
mod field_value;

pub use data_point::DataPoint;
pub use error::Error;
pub use field_value::FieldValue;

fn check_string_length(value: &str) -> Result<(), Error> {
    if value.len() <= 64 << 10 {
        Ok(())
    } else {
        Err(Error::StringLengthLimit)
    }
}
