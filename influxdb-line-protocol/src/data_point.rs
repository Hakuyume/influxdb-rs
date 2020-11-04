use crate::{check_string_length, Error, FieldValue};
use core::fmt::Write;

#[derive(Clone, Copy, Debug)]
pub struct DataPoint<'a, T, F>
where
    T: IntoIterator<Item = (&'a str, &'a str)>,
    F: IntoIterator<Item = (&'a str, FieldValue<'a>)>,
{
    pub measurement: &'a str,
    pub tag_set: T,
    pub field_set: F,
    pub timestamp: Option<i64>,
}

impl<'a, T, F> DataPoint<'a, T, F>
where
    T: IntoIterator<Item = (&'a str, &'a str)>,
    F: IntoIterator<Item = (&'a str, FieldValue<'a>)>,
{
    pub fn into_writer<W>(self, mut writer: W) -> Result<(), Error>
    where
        W: Write,
    {
        check_string_length(self.measurement)?;
        if self.measurement.starts_with('_') {
            return Err(Error::NamingRestrictions);
        }
        for c in self.measurement.chars() {
            match c {
                '\n' => return Err(Error::Newline),
                ',' => writer.write_str(r#"\,"#)?,
                ' ' => writer.write_str(r#"\ "#)?,
                _ => writer.write_char(c)?,
            }
        }

        for (key, value) in self.tag_set {
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
        for (i, (key, value)) in self.field_set.into_iter().enumerate() {
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

        if let Some(timestamp) = self.timestamp {
            write!(writer, " {}", timestamp)?;
        }

        writeln!(writer)?;
        Ok(())
    }

    #[cfg(feature = "std")]
    pub fn into_string(self) -> Result<String, Error> {
        let mut string = String::new();
        self.into_writer(&mut string)?;
        Ok(string)
    }
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

#[cfg(test)]
mod tests;
