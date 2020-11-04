//! Rust implementation of [`InfluxDB's line protocol`](https://docs.influxdata.com/influxdb/v2.0/reference/syntax/line-protocol/)
//!
//! # Example
//!
//! ```
//! use influxdb_line_protocol::FieldValue;
//!
//! print!(
//!     "{}",
//!     influxdb_line_protocol::to_string(
//!         "myMeasurement",
//!         vec![("tag1", "value1"), ("tag2", "value2")],
//!         vec![("fieldKey", FieldValue::String("fieldValue"))],
//!         Some(1556813561098000000),
//!     )
//!     .unwrap()
//! );
//! ```

mod error;
mod field_value;

pub use error::Error;
pub use field_value::FieldValue;
use std::fmt::Write;

pub fn to_writer<'a, W, T, F>(
    mut writer: W,
    measurement: &str,
    tag_set: T,
    field_set: F,
    timestamp: Option<i64>,
) -> Result<(), Error>
where
    W: Write,
    T: IntoIterator<Item = (&'a str, &'a str)>,
    F: IntoIterator<Item = (&'a str, FieldValue<'a>)>,
{
    check_string_length(measurement)?;
    if measurement.starts_with('_') {
        return Err(Error::NamingRestrictions);
    }
    for c in measurement.chars() {
        match c {
            '\n' => return Err(Error::Newline),
            ',' => writer.write_str(r#"\,"#)?,
            ' ' => writer.write_str(r#"\ "#)?,
            _ => writer.write_char(c)?,
        }
    }

    for (key, value) in tag_set {
        write!(writer, ",")?;
        check_string_length(key)?;
        if key.starts_with('_') {
            return Err(Error::NamingRestrictions);
        }
        escape(&mut writer, key)?;
        write!(writer, "=")?;
        check_string_length(value)?;
        escape(&mut writer, value)?;
    }

    let mut count = 0;
    for (i, (key, value)) in field_set.into_iter().enumerate() {
        if i == 0 {
            write!(writer, " ")?;
        } else {
            write!(writer, ",")?;
        }
        check_string_length(key)?;
        if key.starts_with('_') {
            return Err(Error::NamingRestrictions);
        }
        escape(&mut writer, key)?;
        write!(writer, "=")?;
        value.to_writer(&mut writer)?;

        count += 1;
    }
    if count == 0 {
        return Err(Error::EmptyFieldSet);
    }

    if let Some(timestamp) = timestamp {
        write!(writer, " {}", timestamp)?;
    }

    writeln!(writer)?;
    Ok(())
}

pub fn to_string<'a, T, F>(
    measurement: &str,
    tag_set: T,
    field_set: F,
    timestamp: Option<i64>,
) -> Result<String, Error>
where
    T: IntoIterator<Item = (&'a str, &'a str)>,
    F: IntoIterator<Item = (&'a str, FieldValue<'a>)>,
{
    let mut string = String::new();
    to_writer(&mut string, measurement, tag_set, field_set, timestamp)?;
    Ok(string)
}

// For tag key, tag value, and field key
fn escape<W>(mut writer: W, value: &str) -> Result<(), Error>
where
    W: Write,
{
    for c in value.chars() {
        match c {
            '\n' => return Err(Error::Newline),
            ',' => writer.write_str(r#"\,"#)?,
            '=' => writer.write_str(r#"\="#)?,
            ' ' => writer.write_str(r#"\ "#)?,
            _ => writer.write_char(c)?,
        }
    }
    Ok(())
}

fn check_string_length(value: &str) -> Result<(), Error> {
    if value.len() <= 64 << 10 {
        Ok(())
    } else {
        Err(Error::StringLengthLimit)
    }
}

#[cfg(test)]
mod tests;
