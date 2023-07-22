use std::{fmt, io, result};
use thiserror::Error;

pub type Result<T> = result::Result<T, MatrixError>;

#[derive(Error, Debug)]
pub enum MatrixError {
    #[error("failed to read file")]
    ReadFileError,

    #[error("failed to print matrix")]
    PrintMatrixError,
    // #[error("failed to print matrix: {0}")]
    // PrintMatrixError(io::Error),

    #[error("vertex can not less than 0")]
    VertexError,

    #[error("vertex: {0} must smaller than {1} ")]
    InvalidVertexEdge(usize, usize),

    // 自环边错误
    #[error("self loop detected")]
    SelfLoop,

    #[error("parallel edge detected")]
    ParallelEdge
}
