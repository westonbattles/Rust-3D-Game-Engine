use thiserror::Error;

#[derive(Error, Debug)]
pub enum Errors {
    #[error("This is a test error for the Rust Game Engine")]
    TestError,
}