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
        concat!(
            r#"myMeasurement,tag1=value1,tag2=value2 fieldKey="fieldValue" 1556813561098000000"#,
            "\n"
        ),
    );
    assert_eq!(
        to_string(
            "my Measurement",
            iter::empty(),
            vec![("fieldKey", FieldValue::String(r#"string value"#))],
            None,
        )
        .unwrap(),
        concat!(r#"my\ Measurement fieldKey="string value""#, "\n"),
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
        concat!(
            r#"myMeasurement fieldKey="\"string\" within a string""#,
            "\n"
        ),
    );
    assert_eq!(
        to_string(
            "myMeasurement",
            vec![("tag Key1", "tag Value1"), ("tag Key2", "tag Value2")],
            vec![("fieldKey", FieldValue::Float(100.))],
            None,
        )
        .unwrap(),
        concat!(
            r#"myMeasurement,tag\ Key1=tag\ Value1,tag\ Key2=tag\ Value2 fieldKey=100"#,
            "\n"
        ),
    );
    assert_eq!(
        to_string(
            "myMeasurement",
            vec![("tagKey", "🍭")],
            vec![("fieldKey", FieldValue::String(r#"Launch 🚀"#))],
            Some(1556813561098000000),
        )
        .unwrap(),
        concat!(
            r#"myMeasurement,tagKey=🍭 fieldKey="Launch 🚀" 1556813561098000000"#,
            "\n"
        ),
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
        concat!(r#"myMeasurement fieldKey1=1,fieldKey2=2i"#, "\n"),
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

#[test]
#[should_panic(expected = "Newline")]
fn test_to_string_newline() {
    to_string(
        "myMeasurement",
        iter::empty(),
        vec![("fieldKey", FieldValue::String("field\nValue"))],
        None,
    )
    .unwrap();
}
