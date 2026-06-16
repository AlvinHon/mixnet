use crate::{
    mlwe::MlweCiphertext,
    preliminaries::{algebra::cloest_q_div_2_to_bin, mat::Mat},
};
use poly_ring_xnp1::{Polynomial, zq::ZqI64};
use serde_derive::{Deserialize, Serialize};

/// MLWE secret key type
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MlweSecretKey<const Q: i64, const N: usize, const K: usize> {
    pub(crate) s: Mat<ZqI64<Q>, N, K, 1>,
}

impl<const Q: i64, const N: usize, const K: usize> MlweSecretKey<Q, N, K> {
    /// MLWE decryption: recovers binary message m
    pub fn decrypt(&self, ct: &MlweCiphertext<Q, N, K>) -> Polynomial<ZqI64<Q>, N> {
        let p = ct.v.clone() - ct.u_t.dot(&self.s).polynomials[0][0].clone();
        cloest_q_div_2_to_bin(p)
    }
}
