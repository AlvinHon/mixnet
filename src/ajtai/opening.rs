use poly_ring_xnp1::zq::ZqI64;

use crate::preliminaries::mat::Mat;

/// Opening: randomness vector r
pub struct AjtaiCommitmentOpening<const Q: i64, const N: usize, const K2: usize> {
    pub(crate) r: Mat<ZqI64<Q>, N, K2, 1>,
}
