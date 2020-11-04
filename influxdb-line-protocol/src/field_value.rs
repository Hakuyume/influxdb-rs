use core::fmt::{Result, Write};

#[derive(Clone, Copy, Debug)]
pub enum FieldValue<'a> {
    Float(f64),
    Integer(i64),
    UInteger(u64),
    String(&'a str),
    Boolean(bool),
}

impl FieldValue<'_> {
    pub fn to_writer<W>(&self, mut writer: W) -> Result
    where
        W: Write,
    {
        match self {
            FieldValue::Float(v) => write!(writer, "{}", v),
            FieldValue::Integer(v) => write!(writer, "{}i", v),
            FieldValue::UInteger(v) => write!(writer, "{}u", v),
            FieldValue::String(v) => {
                writer.write_char('"')?;
                for c in v.chars() {
                    match c {
                        '"' => writer.write_str(r#"\""#)?,
                        '\\' => writer.write_str(r#"\\"#)?,
                        _ => writer.write_char(c)?,
                    }
                }
                writer.write_char('"')?;
                Ok(())
            }
            FieldValue::Boolean(v) => {
                if *v {
                    write!(writer, "true")
                } else {
                    write!(writer, "false")
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::FieldValue;

    fn check(field_value: FieldValue, expected: &str) {
        let mut string = String::new();
        field_value.to_writer(&mut string).unwrap();
        assert_eq!(string, expected);
    }

    #[test]
    fn test_float() {
        check(FieldValue::Float(1.), "1");
        check(
            FieldValue::Float(-1.234456e+78),
            "-1234456000000000000000000000000000000000000000000000000000000000000000000000000",
        );
    }

    #[test]
    fn test_integer() {
        check(FieldValue::Integer(1), "1i");
        check(FieldValue::Integer(12485903), "12485903i");
        check(FieldValue::Integer(-12485903), "-12485903i");
    }

    #[test]
    fn test_uinteger() {
        check(FieldValue::UInteger(1), "1u");
        check(FieldValue::UInteger(12485903), "12485903u");
    }

    #[test]
    fn test_string() {
        check(
            FieldValue::String(r#"this is a string"#),
            r#""this is a string""#,
        );
        check(
            FieldValue::String(r#""string" within a string"#),
            r#""\"string\" within a string""#,
        );
        check(
            FieldValue::String(r#""\"string\" within a string""#),
            r#""\"\\\"string\\\" within a string\"""#,
        );
    }

    #[test]
    fn test_boolean() {
        check(FieldValue::Boolean(true), "true");
        check(FieldValue::Boolean(false), "false");
    }
}
