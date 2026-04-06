use poly_ring_xnp1::{Polynomial, zq::ZqI64};

use crate::{
    mlwe::{MlweCiphertext, sample_poly_b_eta},
    preliminaries::{algebra::multiply_by_q_div_2, mat::Mat},
};

/// MLWE public key type
pub struct MlwePublicKey<const Q: i64, const N: usize, const K: usize> {
    pub(crate) a: Mat<ZqI64<Q>, N, K, K>,
    pub(crate) b: Mat<ZqI64<Q>, N, K, 1>,
}

impl<const Q: i64, const N: usize, const K: usize> MlwePublicKey<Q, N, K> {
    /// MLWE encryption: encrypts binary message m
    pub fn encrypt<R: rand::RngExt + ?Sized>(
        &self,
        m: &Polynomial<ZqI64<Q>, N>,
        eta: usize,
        rng: &mut R,
    ) -> MlweCiphertext<Q, N, K> {
        // Encode a binary message polynomial as [q/2] * m
        let m_enc = multiply_by_q_div_2(m.clone());

        let r_t = Mat::<ZqI64<Q>, N, 1, K>::from_fn(|| sample_poly_b_eta::<Q, N, _>(eta, rng));
        let e2_t = Mat::<ZqI64<Q>, N, 1, K>::from_fn(|| sample_poly_b_eta::<Q, N, _>(eta, rng));
        let e3 = Mat::<ZqI64<Q>, N, 1, 1>::from_fn(|| sample_poly_b_eta(eta, rng));
        let u_t = r_t.dot(&self.a).add(&e2_t);
        let v = r_t.dot(&self.b).add(&e3).polynomials[0][0].clone() + m_enc;
        MlweCiphertext { u_t, v }
    }
}
