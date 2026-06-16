use poly_ring_xnp1::{Polynomial, zq::ZqI64};
use serde_derive::{Deserialize, Serialize};

use crate::preliminaries::mat::Mat;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OTSEMessage<const Q: i64, const N: usize, const L: usize> {
    pub(crate) m: Mat<ZqI64<Q>, N, L, 1>,
}

impl<const Q: i64, const N: usize, const L: usize> OTSEMessage<Q, N, L> {
    pub(crate) fn to_polynomials(&self) -> [Polynomial<ZqI64<Q>, N>; L] {
        let m_vecs = self.m.flatten();
        core::array::from_fn(|i| m_vecs[i].clone())
    }
}
