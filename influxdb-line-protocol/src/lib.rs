mod field_value;

use core::fmt::Write;
pub use field_value::FieldValue;

#[derive(Debug, thiserror::Error)]
pub enum Error {
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
    for c in measurement.chars() {
        match c {
            ',' => writer.write_str(r#"\,"#)?,
            ' ' => writer.write_str(r#"\ "#)?,
            _ => writer.write_char(c)?,
        }
    }

    for (key, value) in tag_set {
        write!(writer, ",")?;
        escape(&mut writer, key)?;
        write!(writer, "=")?;
        escape(&mut writer, value)?;
    }

    write!(writer, " ")?;
    for (i, (key, value)) in field_set.into_iter().enumerate() {
        if i > 0 {
            write!(writer, ",")?;
        }
        escape(&mut writer, key)?;
        write!(writer, "=")?;
        value.to_writer(&mut writer)?;
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

// for Tag key, Tag value, and Field Key
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

#[cfg(test)]
mod tests {
    use super::{to_string, FieldValue};
    use core::iter;

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
}
