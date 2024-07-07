
#[derive(thiserror::Error, Debug)]
#[error("Wrong number of arguments to operator: expected {expected}, got {actual}")]
pub struct WrongNumberOfArgumentsError {
    pub expected: usize,
    pub actual: usize,
}