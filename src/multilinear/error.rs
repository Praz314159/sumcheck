//! Error types for multilinear extension operations

use thiserror::Error;

/// Errors that can occur when constructing or querying an oracle
#[derive(Debug, Clone, Error, PartialEq, Eq)]
pub enum OracleError {
    /// Oracle size does not match 2^dim
    #[error("Oracle size must be 2^dim")]
    IncorrectOracleSize,

    /// Point not found in the boolean hypercube map
    #[error("Point not found in boolean hypercube map")]
    PointNotFound,
}

/// Errors that can occur when evaluating multilinear extensions
#[derive(Debug, Clone, Error, PartialEq, Eq)]
pub enum MLEError {
    /// Evaluation point has wrong dimension
    #[error("Dimension mismatch: expected dimension {expected}, but found dimension {found}")]
    WrongDimension { expected: usize, found: usize },

    /// Oracle error during evaluation
    #[error("Oracle error: {0}")]
    Oracle(#[from] OracleError),
}
