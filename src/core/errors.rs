
#[derive(thiserror::Error, Debug)]
#[error("Wrong number of arguments to operator: expected {expected}, got {actual}")]
pub struct WrongNumberOfArgumentsError {
    pub expected: usize,
    pub actual: usize,
}

#[derive(thiserror::Error, Debug)]
#[error("Attempted to reduce a non-reduceible expression")]
pub struct NonreduceibleExpressionError;