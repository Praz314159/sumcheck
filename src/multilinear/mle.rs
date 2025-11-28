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
use hashbrown::HashMap;
use rand::Rng;
use std::collections::HashSet;
use std::marker::PhantomData;

use super::error::{MLEError, OracleError};
use super::traits::{BCubeMap, MLE};

/// mapping accessible as an oracle
pub struct BCubeMapOracle<F: Field> {
    // dimension >= 2
    pub dim: usize,

    // Hashmap of bitstrings of length dim to field element. There are a couple of ways
    // we can represent these bitstrings. But for now, I think the thing to do is represent
    // these as byte vectors.
    pub map: HashMap<Vec<F>, F>,
}

impl<F: Field> BCubeMapOracle<F> {
    // basic constructor
    pub fn new(dim: usize, map: HashMap<Vec<F>, F>) -> Result<BCubeMapOracle<F>, OracleError> {
        // define boolean elements in field
        let b = HashSet::from([F::zero(), F::one()]);

        // check size of map
        let npoints = 1 << dim;
        if map.len() != npoints {
            return Err(OracleError::IncorrectOracleSize);
        }

        // check that the map is from {0,1}^dim
        for (point, _) in &map {
            // check dimension
            if point.len() != dim {
                return Err(OracleError::IncorrectOraclePointDimension {
                    expected: dim,
                    found: point.len(),
                });
            }

            // check all coordinates are boolean
            if !point.iter().all(|elt| b.contains(elt)) {
                return Err(OracleError::NonbooleanOraclePoint);
            }
        }

        Ok(BCubeMapOracle { dim, map })
    }

    pub fn new_rand<R: Rng>(dim: usize, rng: &mut R) -> Result<BCubeMapOracle<F>, OracleError>
    where
        F: UniformRand,
    {
        // generate random field 2^dim random field elements
        let num_points = 1 << dim;

        // initialize hashmap of correct size
        let mut map: HashMap<Vec<F>, F> = HashMap::with_capacity(num_points);

        // construct hashmap
        for n in 0..num_points {
            let bcube_elt = to_bcube_elt(dim, n);
            let field_elt = F::rand(rng);

            map.insert(bcube_elt, field_elt);
        }

        Self::new(dim, map)
    }
}

impl<F: Field> BCubeMap<F> for BCubeMapOracle<F> {
    // impl query on hashmap
    fn query(&self, point: &[F]) -> Result<F, OracleError> {
        // Look up point
        self.map
            .get(point)
            .copied()
            .ok_or(OracleError::PointNotFound)
    }

    // impl iter
    fn iter(&self) -> impl Iterator<Item = (&Vec<F>, &F)> {
        self.map.iter()
    }
}

// Now we have a map we can query. We want multiple different MLEs that use it
// in order to evaluate differently.

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

/// implementation
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
    // b in B^w in a brute force manner. Then sums the produc
    // of the map query with the multivariate lagrange basis
    fn naive(&self, z: &[F]) -> Result<F, MLEError> {
        // check that z has the right dimension
        if z.len() != self.dim {
            return Err(MLEError::WrongDimension {
                expected: self.dim,
                found: z.len(),
            });
        }
        // iterate over map to compute MLE_f(b)
        self.oracle.iter().try_fold(F::zero(), |acc, (b, &f_b)| {
            let chi = eq(b, z)?;
            Ok(acc + f_b * chi)
        })
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

/// helper function to compute multivariate lagrange basis polynomial
/// from any b and z
pub fn eq<F: Field>(b: &[F], z: &[F]) -> Result<F, MLEError> {
    // check if dimension mismatch
    if b.len() != z.len() {
        return Err(MLEError::InconsistentDimensions {
            b_dim: b.len(),
            z_dim: z.len(),
        });
    }

    Ok(b.iter()
        .zip(z.iter())
        .map(|(&b, &z)| b * z + (F::one() - b) * (F::one() - z))
        .product())
}

/// helper function that takes an int and returns the
/// boolean hypercube element, where the hypercube is a
/// subset of F^dim
pub fn to_bcube_elt<F: Field>(dim: usize, n: usize) -> Vec<F> {
    (0..dim)
        .map(|i| {
            if (n >> i) & 1 == 1 {
                F::one()
            } else {
                F::zero()
            }
        })
        .collect()
}
