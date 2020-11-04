mod field_value;

pub use field_value::FieldValue;
use std::fmt::Write;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("measurement names, tag keys, and field keys cannot begin with an underscore")]
    NamingRestrictions,
    #[error("points must have at least one field")]
    EmptyFieldSet,
    #[error("length limit 64KB")]
    StringLengthLimit,
    #[error(transparent)]
    Fmt(#[from] std::fmt::Error),
}

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
mod tests {
    use super::{to_string, FieldValue};
    use std::iter;

    #[test]
    fn test_to_string() {
        assert_eq!(
            to_string(
                "myMeasurement",
                vec![("tag1", "value1"), ("tag2", "value2")],
                vec![("fieldKey", FieldValue::String("fieldValue"))],
                Some(1556813561098000000),
            )
            .unwrap(),
            r#"myMeasurement,tag1=value1,tag2=value2 fieldKey="fieldValue" 1556813561098000000"#
        );
        assert_eq!(
            to_string(
                "my Measurement",
                iter::empty(),
                vec![("fieldKey", FieldValue::String(r#"string value"#))],
                None,
            )
            .unwrap(),
            r#"my\ Measurement fieldKey="string value""#
        );
        assert_eq!(
            to_string(
                "myMeasurement",
                iter::empty(),
                vec![(
                    "fieldKey",
                    FieldValue::String(r#""string" within a string"#)
                )],
                None,
            )
            .unwrap(),
            r#"myMeasurement fieldKey="\"string\" within a string""#
        );
        assert_eq!(
            to_string(
                "myMeasurement",
                vec![("tag Key1", "tag Value1"), ("tag Key2", "tag Value2")],
                vec![("fieldKey", FieldValue::Float(100.))],
                None,
            )
            .unwrap(),
            r#"myMeasurement,tag\ Key1=tag\ Value1,tag\ Key2=tag\ Value2 fieldKey=100"#
        );
        assert_eq!(
            to_string(
                "myMeasurement",
                vec![("tagKey", "🍭")],
                vec![("fieldKey", FieldValue::String(r#"Launch 🚀"#))],
                Some(1556813561098000000),
            )
            .unwrap(),
            r#"myMeasurement,tagKey=🍭 fieldKey="Launch 🚀" 1556813561098000000"#,
        );

        assert_eq!(
            to_string(
                "myMeasurement",
                iter::empty(),
                vec![
                    ("fieldKey1", FieldValue::Float(1.)),
                    ("fieldKey2", FieldValue::Integer(2))
                ],
                None,
            )
            .unwrap(),
            r#"myMeasurement fieldKey1=1,fieldKey2=2i"#
        );
    }

    #[test]
    #[should_panic(expected = "NamingRestrictions")]
    fn test_to_string_naming_restrictions() {
        to_string(
            "_myMeasurement",
            iter::empty(),
            vec![("fieldKey", FieldValue::String("fieldValue"))],
            None,
        )
        .unwrap();
    }

    #[test]
    #[should_panic(expected = "EmptyFieldSet")]
    fn test_to_string_empty_field_set() {
        to_string("myMeasurement", iter::empty(), vec![], None).unwrap();
    }

    #[test]
    #[should_panic(expected = "StringLengthLimit")]
    fn test_to_string_string_length_limit() {
        let mut field_value = String::new();
        for _ in 0..=64 << 10 {
            field_value += "a"
        }
        to_string(
            "myMeasurement",
            iter::empty(),
            vec![("fieldKey", FieldValue::String(&field_value))],
            None,
        )
        .unwrap();
    }
}
