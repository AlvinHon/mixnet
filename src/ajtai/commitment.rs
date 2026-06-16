use poly_ring_xnp1::zq::ZqI64;
use serde_derive::{Deserialize, Serialize};

use crate::preliminaries::mat::Mat;

/// Commitment value
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AjtaiCommitment<const Q: i64, const N: usize, const K1: usize> {
    pub(crate) c: Mat<ZqI64<Q>, N, K1, 1>,
}
