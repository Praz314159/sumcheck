//! Error types for multilinear extension operations

use thiserror::Error;

/// Errors that can occur when constructing or querying an oracle
#[derive(Debug, Clone, Error, PartialEq, Eq)]
pub enum OracleError {
    /// Oracle size does not match 2^dim
    #[error("Oracle size must be 2^dim")]
    IncorrectOracleSize,

    /// A key in the map has incorrect dimension
    #[error("Dimension mismatch: expected dimension {expected}, but found dimension {found}")]
    IncorrectOraclePointDimension { expected: usize, found: usize },

    /// A key contains non-boolean values (not in {0, 1})
    #[error("Non-boolean value encountered: all coordinates must be in {{0, 1}}")]
    NonbooleanOraclePoint,

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

    /// Inconsistent dimensions between b and z in eq polynomial
    #[error("b and z must have consistent dimensions: b has {b_dim}, z has {z_dim}")]
    InconsistentDimensions { b_dim: usize, z_dim: usize },

    /// Oracle error during evaluation
    #[error("Oracle error: {0}")]
    Oracle(#[from] OracleError),
}
