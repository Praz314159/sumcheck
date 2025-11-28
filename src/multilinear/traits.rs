//! We need:
//!     1. A map
//!     2. A multilinear extension

use super::error::{MLEError, OracleError};
use ark_ff::Field;

/// This is a mapping from the boolean hypercube to some finite
/// field F. Any mapping can be queried. The query can either
/// be an evaluation or it can literally be a mapping that is
/// queried as an oracle in constant time.
pub trait BCubeMap<F: Field> {
    // dimension of the boolean hypercube
    fn dim(&self) -> usize;

    // query by index i, returns f(to_bcube_elt(dim, i))
    fn query(&self, index: usize) -> Result<F, OracleError>;

    // iterate over (index, &value) pairs
    fn iter(&self) -> impl Iterator<Item = (usize, &F)>;
}

/// This is a multilinear extension of a mapping from the boolean
/// hypercube to some finite field F. A MLE is generic over any
/// such function and finite field.
///
/// We are benchmarking different algorithms for evaluating multilinear
/// extensions. So, the idea is that we different implementations of this
/// trait.
pub trait MLE<F: Field, M: BCubeMap<F>> {
    // evaluate takes a point z in F^w and evaluates MLE_f(z) in F.
    // There are a few different ways to implement this as we will see
    fn evaluate(&self, z: &[F]) -> Result<F, MLEError>;
}
