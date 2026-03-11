use poly_ring_xnp1::{Polynomial, zq::ZqI64};

/// Opening: randomness vector r
pub struct AjtaiCommitmentOpening<const Q: i64, const N: usize, const M: usize> {
    pub(crate) r: [Polynomial<ZqI64<Q>, N>; M],
}
