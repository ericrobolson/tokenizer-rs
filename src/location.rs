use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq)]
pub struct Location {
    pub row: usize,
    pub column: usize,
    pub path: Option<PathBuf>,
}

impl Location {
    pub fn new(column: usize, row: usize, path: Option<PathBuf>) -> Self {
        Self { column, row, path }
    }
}
impl Default for Location {
    fn default() -> Self {
        Self {
            row: Default::default(),
            column: Default::default(),
            path: Default::default(),
        }
    }
}
impl From<(usize, usize)> for Location {
    fn from((row, column): (usize, usize)) -> Self {
        Self {
            column,
            row,
            path: None,
        }
    }
}
impl From<PathBuf> for Location {
    fn from(path: PathBuf) -> Self {
        Self {
            row: 0,
            column: 0,
            path: Some(path),
        }
    }
}
