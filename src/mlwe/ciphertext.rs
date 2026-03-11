use poly_ring_xnp1::{Polynomial, zq::ZqI64};

/// MLWE ciphertext type
pub struct MlweCiphertext<const Q: i64, const N: usize> {
    pub(crate) u: Polynomial<ZqI64<Q>, N>,
    pub(crate) v: Polynomial<ZqI64<Q>, N>,
}
