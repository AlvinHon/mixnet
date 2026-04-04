use poly_ring_xnp1::{Polynomial, zq::ZqI64};

use crate::mlwe::{MlweCiphertext, sample_poly_b_eta};

/// MLWE public key type
pub struct MlwePublicKey<const Q: i64, const N: usize> {
    pub(crate) a: Polynomial<ZqI64<Q>, N>,
    pub(crate) b: Polynomial<ZqI64<Q>, N>,
}

impl<const Q: i64, const N: usize> MlwePublicKey<Q, N> {
    /// MLWE encryption: encrypts binary message m
    pub fn encrypt<R: rand::RngExt + ?Sized>(
        &self,
        m: &Polynomial<ZqI64<Q>, N>,
        eta: usize,
        rng: &mut R,
    ) -> MlweCiphertext<Q, N> {
        let m_enc = Self::encode_message(m);
        let r = sample_poly_b_eta::<Q, N, R>(eta, rng);
        let e1 = sample_poly_b_eta::<Q, N, R>(eta, rng);
        let e2 = sample_poly_b_eta::<Q, N, R>(eta, rng);
        let u = self.a.clone() * r.clone() + e1;
        let v = self.b.clone() * r + e2 + m_enc;
        MlweCiphertext { u, v }
    }

    /// Encode a binary message polynomial as [q/2] * m
    fn encode_message(m: &Polynomial<ZqI64<Q>, N>) -> Polynomial<ZqI64<Q>, N> {
        let q_half = Q / 2;
        Polynomial::<ZqI64<Q>, N>::new(
            m.iter()
                .map(|c| ZqI64::new(i64::from(c.clone()) * q_half))
                .collect(),
        )
    }
}
