use crate::mlwe::MlweCiphertext;
use poly_ring_xnp1::{Polynomial, zq::ZqI64};

/// MLWE secret key type
pub struct MlweSecretKey<const Q: i64, const N: usize> {
    pub(crate) s: Polynomial<ZqI64<Q>, N>,
}

impl<const Q: i64, const N: usize> MlweSecretKey<Q, N> {
    /// MLWE decryption: recovers binary message m
    pub fn decrypt(&self, ct: &MlweCiphertext<Q, N>) -> Polynomial<ZqI64<Q>, N> {
        let p = ct.v.clone() - self.s.clone() * ct.u.clone();
        Self::decode_message(&p)
    }

    /// Decode a polynomial to binary by thresholding at q/2
    fn decode_message(p: &Polynomial<ZqI64<Q>, N>) -> Polynomial<ZqI64<Q>, N> {
        let q_half = Q / 2;
        Polynomial::<ZqI64<Q>, N>::new(
            p.iter()
                .map(|c| {
                    let val = i64::from(c.clone());
                    if val >= q_half {
                        ZqI64::new(1)
                    } else {
                        ZqI64::new(0)
                    }
                })
                .collect(),
        )
    }
}
