use crate::location::Location;

#[derive(Debug, Clone, PartialEq)]
pub struct Error {
    pub message: String,
    pub location: Location,
}
