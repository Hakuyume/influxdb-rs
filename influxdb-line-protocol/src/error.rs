#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("measurement names, tag keys, and field keys cannot begin with an underscore")]
    NamingRestrictions,
    #[error("points must have at least one field")]
    EmptyFieldSet,
    #[error("length limit 64KB")]
    StringLengthLimit,
    #[error("line protocol does not support the newline character in tag or field values")]
    Newline,
    #[error(transparent)]
    Fmt(#[from] std::fmt::Error),
}
