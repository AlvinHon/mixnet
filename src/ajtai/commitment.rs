use poly_ring_xnp1::zq::ZqI64;

use crate::preliminaries::mat::Mat;

/// Commitment value
pub struct AjtaiCommitment<const Q: i64, const N: usize, const K1: usize> {
    pub(crate) c: Mat<ZqI64<Q>, N, K1, 1>,
}
