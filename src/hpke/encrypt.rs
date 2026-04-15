use poly_ring_xnp1::{Polynomial, zq::ZqI64};

use crate::{
    hpke::ciphertext::HpkeCiphertext,
    mlwe::MlwePublicKey,
    otse::{OTSEKey, OTSEMessage, OTSEParams},
    preliminaries::mat::Mat,
};

#[derive(Clone)]
pub struct HpkePublicKey<
    const Q: i64,
    const N: usize,
    const KE: usize, // K_lwe
    const Z: i64,    // 2^z
    const E: i64,    // 2^(2*eta)
    const KR: usize, // K_lwr
    const L: usize,
> {
    pub(crate) otse_params: OTSEParams<Q, N, KE, Z, E, KR, L>,
    pub(crate) pk: MlwePublicKey<Q, N, KE>,
}

impl<
    const Q: i64,
    const N: usize,
    const KE: usize,
    const Z: i64,
    const E: i64,
    const KR: usize,
    const L: usize,
> HpkePublicKey<Q, N, KE, Z, E, KR, L>
{
    pub fn encrypt<R: rand::Rng + ?Sized>(
        &self,
        m: &[Polynomial<ZqI64<Q>, N>; L],
        rng: &mut R,
    ) -> HpkeCiphertext<Q, N, KE, KR, L> {
        let otse_key = OTSEKey::<Q, N, KE, Z, E, KR, L>::new(rng);

        // Encrypt the message using the OTSE scheme
        let cs = {
            let m_mat = {
                let m_vecs = m.iter().map(|m_i| vec![m_i.clone()]).collect::<Vec<_>>();
                Mat::from(m_vecs)
            };
            otse_key.encode(&OTSEMessage::<Q, N, L> { m: m_mat }, &self.otse_params)
        };

        // Encrypt otse key using the MLWE scheme
        let c = {
            otse_key
                .s
                .polynomials
                .iter()
                .map(|s_i| self.pk.encrypt(&s_i[0], self.otse_params.eta, rng))
                .collect::<Vec<_>>()
        };

        HpkeCiphertext { c, cs }
    }

    pub fn encrypt_next<R: rand::Rng + ?Sized>(
        &self,
        ciphertext: &HpkeCiphertext<Q, N, KE, KR, L>,
        rng: &mut R,
    ) -> HpkeCiphertext<Q, N, KE, KR, L> {
        let mut next_ciphertext = self.encrypt(&ciphertext.cs.to_polynomials(), rng);

        // combine the first element c with the next ciphertext's c
        next_ciphertext.c.extend_from_slice(&ciphertext.c);

        next_ciphertext
    }
}
