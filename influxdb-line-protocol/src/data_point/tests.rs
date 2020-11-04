use super::DataPoint;
use crate::FieldValue;
use std::iter;

#[test]
fn test_to_string() {
    assert_eq!(
        DataPoint {
            measurement: "myMeasurement",
            tag_set: vec![("tag1", "value1"), ("tag2", "value2")],
            field_set: vec![("fieldKey", FieldValue::String("fieldValue"))],
            timestamp: Some(1556813561098000000),
        }
        .to_string()
        .unwrap(),
        concat!(
            r#"myMeasurement,tag1=value1,tag2=value2 fieldKey="fieldValue" 1556813561098000000"#,
            "\n"
        ),
    );
    assert_eq!(
        DataPoint {
            measurement: "my Measurement",
            tag_set: iter::empty(),
            field_set: vec![("fieldKey", FieldValue::String(r#"string value"#))],
            timestamp: None,
        }
        .to_string()
        .unwrap(),
        concat!(r#"my\ Measurement fieldKey="string value""#, "\n"),
    );
    assert_eq!(
        DataPoint {
            measurement: "myMeasurement",
            tag_set: iter::empty(),
            field_set: vec![(
                "fieldKey",
                FieldValue::String(r#""string" within a string"#)
            )],
            timestamp: None,
        }
        .to_string()
        .unwrap(),
        concat!(
            r#"myMeasurement fieldKey="\"string\" within a string""#,
            "\n"
        ),
    );
    assert_eq!(
        DataPoint {
            measurement: "myMeasurement",
            tag_set: vec![("tag Key1", "tag Value1"), ("tag Key2", "tag Value2")],
            field_set: vec![("fieldKey", FieldValue::Float(100.))],
            timestamp: None,
        }
        .to_string()
        .unwrap(),
        concat!(
            r#"myMeasurement,tag\ Key1=tag\ Value1,tag\ Key2=tag\ Value2 fieldKey=100"#,
            "\n"
        ),
    );
    assert_eq!(
        DataPoint {
            measurement: "myMeasurement",
            tag_set: vec![("tagKey", "🍭")],
            field_set: vec![("fieldKey", FieldValue::String(r#"Launch 🚀"#))],
            timestamp: Some(1556813561098000000),
        }
        .to_string()
        .unwrap(),
        concat!(
            r#"myMeasurement,tagKey=🍭 fieldKey="Launch 🚀" 1556813561098000000"#,
            "\n"
        ),
    );

    assert_eq!(
        DataPoint {
            measurement: "myMeasurement",
            tag_set: iter::empty(),
            field_set: vec![
                ("fieldKey1", FieldValue::Float(1.)),
                ("fieldKey2", FieldValue::Integer(2))
            ],
            timestamp: None,
        }
        .to_string()
        .unwrap(),
        concat!(r#"myMeasurement fieldKey1=1,fieldKey2=2i"#, "\n"),
    );
}

#[test]
#[should_panic(expected = "NamingRestrictions")]
fn test_to_string_naming_restrictions() {
    DataPoint {
        measurement: "_myMeasurement",
        tag_set: iter::empty(),
        field_set: vec![("fieldKey", FieldValue::String("fieldValue"))],
        timestamp: None,
    }
    .to_string()
    .unwrap();
}

#[test]
#[should_panic(expected = "EmptyFieldSet")]
fn test_to_string_empty_field_set() {
    DataPoint {
        measurement: "myMeasurement",
        tag_set: iter::empty(),
        field_set: vec![],
        timestamp: None,
    }
    .to_string()
    .unwrap();
}

#[test]
#[should_panic(expected = "StringLengthLimit")]
fn test_to_string_string_length_limit() {
    let mut field_value = String::new();
    for _ in 0..=64 << 10 {
        field_value += "a"
    }
    DataPoint {
        measurement: "myMeasurement",
        tag_set: iter::empty(),
        field_set: vec![("fieldKey", FieldValue::String(&field_value))],
        timestamp: None,
    }
    .to_string()
    .unwrap();
}

#[test]
#[should_panic(expected = "Newline")]
fn test_to_string_newline() {
    DataPoint {
        measurement: "myMeasurement",
        tag_set: iter::empty(),
        field_set: vec![("fieldKey", FieldValue::String("field\nValue"))],
        timestamp: None,
    }
    .to_string()
    .unwrap();
}
