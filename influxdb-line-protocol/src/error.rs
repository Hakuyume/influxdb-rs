#[derive(Debug)]
#[cfg_attr(feature = "std", derive(thiserror::Error))]
pub enum Error {
    #[cfg_attr(
        feature = "std",
        error("measurement names, tag keys, and field keys cannot begin with an underscore")
    )]
    NamingRestrictions,
    #[cfg_attr(feature = "std", error("points must have at least one field"))]
    EmptyFieldSet,
    #[cfg_attr(feature = "std", error("length limit 64KB"))]
    StringLengthLimit,
    #[cfg_attr(
        feature = "std",
        error("line protocol does not support the newline character in tag or field values")
    )]
    Newline,
    #[cfg_attr(feature = "std", error(transparent))]
    Fmt(core::fmt::Error),
}

impl From<core::fmt::Error> for Error {
    fn from(value: core::fmt::Error) -> Self {
        Self::Fmt(value)
    }
}
