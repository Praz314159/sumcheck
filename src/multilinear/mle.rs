//! Different implementations of the multilinear extension
//!
//! The goal is to implement different ways of evaluating the multilinear extension.
//!     1. Naive method -- this is O(w * 2^w) time and O(2^w) space
//!     2. Zhu's method
//!     3. Rothblum's method
//!     4. My method
//!
//! Once these are implemented, we can profile.

use ark_ff::{Field, UniformRand};
use rand::Rng;
use std::marker::PhantomData;

use super::error::{MLEError, OracleError};
use super::traits::{BCubeMap, MLE};

/// Dense oracle - stores values in a Vec indexed by integer representation of boolean points
/// This is memory-efficient: only stores 2^dim field elements, no key storage
pub struct DenseOracle<F: Field> {
    dim: usize,
    values: Vec<F>,
}

/// Different algorithms for evaluating the multilinear extension
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EvaluationType {
    Naive,
    Zhu,
    Rothblum,
    Ramakrishna,
}

/// Multilinear extension that can be evaluated using different strategies
pub struct MultilinearExtension<F: Field, M: BCubeMap<F>> {
    pub oracle: M,
    pub dim: usize,
    pub strategy: EvaluationType,
    _phantom: PhantomData<F>,
}

/// Implementations
impl<F: Field> DenseOracle<F> {
    /// Create from a vector of values
    /// values[i] corresponds to the boolean point to_bcube_elt(dim, i)
    pub fn new(dim: usize, values: Vec<F>) -> Result<DenseOracle<F>, OracleError> {
        let expected_len = 1 << dim;

        if values.len() != expected_len {
            return Err(OracleError::IncorrectOracleSize);
        }

        Ok(DenseOracle { dim, values })
    }

    /// Create with random values
    pub fn new_rand<R: Rng>(dim: usize, rng: &mut R) -> DenseOracle<F>
    where
        F: UniformRand,
    {
        let num_points = 1 << dim;
        let values: Vec<F> = (0..num_points).map(|_| F::rand(rng)).collect();
        DenseOracle { dim, values }
    }
}

impl<F: Field> BCubeMap<F> for DenseOracle<F> {
    fn dim(&self) -> usize {
        self.dim
    }

    fn query(&self, index: usize) -> Result<F, OracleError> {
        self.values
            .get(index)
            .copied()
            .ok_or(OracleError::PointNotFound)
    }

    fn iter(&self) -> impl Iterator<Item = (usize, &F)> {
        self.values.iter().enumerate()
    }
}

// Now we have a map we can query. We want multiple different MLEs that use it
// in order to evaluate differently.

/// implement MLE
impl<F: Field, M: BCubeMap<F>> MultilinearExtension<F, M> {
    // constructor
    pub fn new(oracle: M, dim: usize, strategy: EvaluationType) -> Self {
        MultilinearExtension {
            oracle,
            dim,
            strategy,
            _phantom: PhantomData,
        }
    }

    // the naive evaluation computes eq(b, z) for all
    // b in B^w in a brute force manner. Then sums the product
    // of the map query with the multivariate lagrange basis
    fn naive(&self, z: &[F]) -> Result<F, MLEError> {
        // check that z has the right dimension
        if z.len() != self.dim {
            return Err(MLEError::WrongDimension {
                expected: self.dim,
                found: z.len(),
            });
        }
// iterate over map to compute MLE_f(z)
        let one_minus_z: Vec<F> = z
            .iter()
            .map(|z_i| F::one() - z_i)
            .collect();
        
        Ok(self
            .oracle
            .iter()
            .map(|(i, &f_b)| {
                let chi = eq(self.dim, i, z, &one_minus_z);
                f_b * chi
            })
            .sum())
    }

    fn zhu(&self, _z: &[F]) -> Result<F, MLEError> {
        // TODO: Implement Zhu's method
        todo!("Implement Zhu's MLE evaluation")
    }

    fn rothblum(&self, _z: &[F]) -> Result<F, MLEError> {
        // TODO: Implement Rothblum's method
        todo!("Implement Rothblum's MLE evaluation")
    }

    fn ramakrishna(&self, _z: &[F]) -> Result<F, MLEError> {
        // TODO: Implement custom method
        todo!("Implement custom MLE evaluation")
    }
}

impl<F: Field, M: BCubeMap<F>> MLE<F, M> for MultilinearExtension<F, M> {
    fn evaluate(&self, z: &[F]) -> Result<F, MLEError> {
        match self.strategy {
            EvaluationType::Naive => self.naive(z),
            EvaluationType::Zhu => self.zhu(z),
            EvaluationType::Rothblum => self.rothblum(z),
            EvaluationType::Ramakrishna => self.ramakrishna(z),
        }
    }
}

/// Compute eq(b, z) where b is represented by index
/// index represents the boolean point b where bit i of index gives b_i
/// Returns the multivariate Lagrange basis polynomial evaluated at z
/// one_minus_z should be precomputed as (1 - z[i]) for each i
pub fn eq<F: Field>(dim: usize, index: usize, z: &[F], one_minus_z: &[F]) -> F {
    (0..dim)
        .map(|i| {
            if (index >> i) & 1 == 1 {
                z[i]
            } else {
                one_minus_z[i]
            }
        })
        .product()
}
