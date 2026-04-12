use poly_ring_xnp1::zq::ZqI64;

use crate::preliminaries::mat::Mat;

pub struct OTSEEncoded<const Q: i64, const N: usize, const L: usize> {
    pub(crate) c: Mat<ZqI64<Q>, N, L, 1>,
}
