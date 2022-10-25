use thiserror::Error;

#[derive(Error, Debug)]
pub enum TestError {
    #[error("{0}")]
    Custom(String),
}
