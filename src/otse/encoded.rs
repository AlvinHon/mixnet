use poly_ring_xnp1::{Polynomial, zq::ZqI64};

use crate::preliminaries::mat::Mat;

#[derive(Clone)]
pub struct OTSEEncoded<const Q: i64, const N: usize, const L: usize> {
    pub(crate) c: Mat<ZqI64<Q>, N, L, 1>,
}

impl<const Q: i64, const N: usize, const L: usize> OTSEEncoded<Q, N, L> {
    pub(crate) fn to_polynomials(&self) -> [Polynomial<ZqI64<Q>, N>; L] {
        let c_vecs = self.c.flatten();
        core::array::from_fn(|i| c_vecs[i].clone())
    }
}
