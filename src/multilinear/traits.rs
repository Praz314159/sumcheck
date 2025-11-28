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
    // query takes a point b in F^w, and returns the evaluation
    // f(b) in F. One thing here is that b is actually in {0,1}^w.
    // Ideally, we don't actually need to represent these as field
    // elements, but in the beginning we probably can.
    fn query(&self, b: &[F]) -> Result<F, OracleError>;

    // return iterator so points of the boolean hypercube don't have
    // to be reconstructed in order to be queried
    fn iter(&self) -> impl Iterator<Item = (&Vec<F>, &F)>;
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
