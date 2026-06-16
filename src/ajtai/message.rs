use poly_ring_xnp1::{Polynomial, zq::ZqI64};
use serde_derive::{Deserialize, Serialize};

use crate::preliminaries::{algebra::is_bin, mat::Mat};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AjtaiMessage<const Q: i64, const N: usize, const L: usize> {
    pub(crate) m: Mat<ZqI64<Q>, N, L, 1>,
}

impl<const Q: i64, const N: usize, const L: usize> AjtaiMessage<Q, N, L> {
    /// Create message from vector of polynomials
    pub fn from_polynomials(ps: Vec<Polynomial<ZqI64<Q>, N>>) -> Self {
        assert!(ps.len() <= L);
        assert!(ps.iter().all(is_bin));

        let mut m = Mat::<ZqI64<Q>, N, L, 1>::from_fn(|| {
            Polynomial::<ZqI64<Q>, N>::new(vec![ZqI64::new(0); N])
        });
        for (i, p) in ps.into_iter().enumerate() {
            m.polynomials[i][0] = p;
        }
        Self { m }
    }
}
