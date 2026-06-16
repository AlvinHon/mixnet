use poly_ring_xnp1::{Polynomial, zq::ZqI64};
use serde_derive::{Deserialize, Serialize};

use crate::{
    hpke::HpkeCiphertext,
    mlwe::MlweSecretKey,
    otse::{OTSEEncoded, OTSEKey, OTSEParams},
    preliminaries::mat::Mat,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HpkeSecretKey<
    const Q: i64,
    const N: usize,
    const KE: usize, // K_lwe
    const Z: i64,    // 2^z
    const E: i64,    // 2^(2*eta)
    const KR: usize, // K_lwr
    const L: usize,
> {
    pub(crate) otse_params: OTSEParams<Q, N, KE, Z, E, KR, L>,
    pub(crate) sk: MlweSecretKey<Q, N, KE>,
}

impl<
    const Q: i64,
    const N: usize,
    const KE: usize,
    const Z: i64,
    const E: i64,
    const KR: usize,
    const L: usize,
> HpkeSecretKey<Q, N, KE, Z, E, KR, L>
{
    pub fn decrypt(
        &self,
        ciphertext: &HpkeCiphertext<Q, N, KE, KR, L>,
    ) -> HpkeDecryptResult<Q, N, KE, KR, L> {
        if ciphertext.c.is_empty() {
            return HpkeDecryptResult::DecryptionFailed("Ciphertext is empty".to_string());
        }
        if !ciphertext.c.len().is_multiple_of(KR) {
            return HpkeDecryptResult::DecryptionFailed("Invalid ciphertext length".to_string());
        }

        let s = ciphertext
            .c
            .iter()
            .take(KR)
            .map(|c_i| self.sk.decrypt(c_i))
            .collect::<Vec<_>>();

        let remain_c = if ciphertext.c.len() > KR {
            Some(ciphertext.c[KR..].to_vec())
        } else {
            None
        };

        // reshape s into a matrix of size KR x 1
        let s_vecs = s.into_iter().map(|s_i| vec![s_i]).collect::<Vec<_>>();

        let otse_key = OTSEKey::<Q, N, KE, Z, E, KR, L> {
            s: Mat::<ZqI64<Q>, N, KR, 1>::from(s_vecs),
        };

        let decoded_m = otse_key.decode(&ciphertext.cs, &self.otse_params);

        match remain_c {
            Some(remain_c) => HpkeDecryptResult::DecryptedWithNextCiphertext(HpkeCiphertext {
                c: remain_c,
                cs: OTSEEncoded { c: decoded_m.m },
            }),
            None => HpkeDecryptResult::DecryptedMessage(decoded_m.to_polynomials()),
        }
    }
}

pub enum HpkeDecryptResult<
    const Q: i64,
    const N: usize,
    const KE: usize,
    const KR: usize,
    const L: usize,
> {
    DecryptedMessage([Polynomial<ZqI64<Q>, N>; L]),
    DecryptedWithNextCiphertext(HpkeCiphertext<Q, N, KE, KR, L>),
    DecryptionFailed(String),
}
