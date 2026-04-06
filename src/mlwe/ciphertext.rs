use poly_ring_xnp1::{Polynomial, zq::ZqI64};

use crate::preliminaries::mat::Mat;

/// MLWE ciphertext type
pub struct MlweCiphertext<const Q: i64, const N: usize, const K: usize> {
    pub(crate) u_t: Mat<ZqI64<Q>, N, 1, K>,
    pub(crate) v: Polynomial<ZqI64<Q>, N>,
}
