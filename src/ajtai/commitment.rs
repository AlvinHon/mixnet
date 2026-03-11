use poly_ring_xnp1::{Polynomial, zq::ZqI64};

/// Commitment value
pub struct AjtaiCommitment<const Q: i64, const N: usize> {
    pub(crate) c: Polynomial<ZqI64<Q>, N>,
}
