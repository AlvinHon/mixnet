use poly_ring_xnp1::zq::ZqI64;

use crate::preliminaries::mat::Mat;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OTSEMessage<const Q: i64, const N: usize, const L: usize> {
    pub(crate) m: Mat<ZqI64<Q>, N, L, 1>,
}
