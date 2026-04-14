use poly_ring_xnp1::{Polynomial, zq::ZqI64};

use crate::preliminaries::mat::Mat;

/// MLWE ciphertext type
#[derive(Clone)]
pub struct MlweCiphertext<const Q: i64, const N: usize, const K: usize> {
    pub(crate) u_t: Mat<ZqI64<Q>, N, 1, K>,
    pub(crate) v: Polynomial<ZqI64<Q>, N>,
}

impl<const Q: i64, const N: usize, const K: usize> MlweCiphertext<Q, N, K> {
    /// Converts the ciphertext into a vector of polynomials with size K + 1 (the last polynomial is v)
    pub fn to_vecs(&self) -> Vec<Polynomial<ZqI64<Q>, N>> {
        self.u_t
            .flatten()
            .into_iter()
            .chain(vec![self.v.clone()])
            .collect()
    }

    /// Extracts the ciphertext from a vector of polynomials with size > K + 1
    /// Returns the ciphertext and the remaining polynomials in the vector (if any)
    pub fn extract_from_vecs(
        vecs: Vec<Polynomial<ZqI64<Q>, N>>,
    ) -> Option<(Self, Vec<Polynomial<ZqI64<Q>, N>>)> {
        if vecs.len() < K + 1 {
            return None; // Not enough polynomials to extract the ciphertext
        }
        let u_t_polys = vecs[..K].to_vec();
        let v_poly = vecs[K].clone();
        let remaining_vecs = vecs[K + 1..].to_vec();
        let u_t = Mat::<ZqI64<Q>, N, 1, K>::from_flatten(u_t_polys);
        Some((Self { u_t, v: v_poly }, remaining_vecs))
    }

    pub fn extract_all_from_vecs(vecs: Vec<Polynomial<ZqI64<Q>, N>>) -> Option<Vec<Self>> {
        if !vecs.len().is_multiple_of(K + 1) {
            return None; // The number of polynomials must be a multiple of K + 1
        }
        let mut ciphertexts = Vec::new();
        for chunk in vecs.chunks(K + 1) {
            let u_t_polys = chunk[..K].to_vec();
            let v_poly = chunk[K].clone();
            let u_t = Mat::<ZqI64<Q>, N, 1, K>::from_flatten(u_t_polys);
            ciphertexts.push(MlweCiphertext { u_t, v: v_poly });
        }
        Some(ciphertexts)
    }
}

#[cfg(test)]
mod tests {
    use num::Zero;

    use super::*;
    use crate::mlwe::encrypt::MlwePublicKey;

    #[test]
    fn test_mlwe_ciphertext_vec_conversion() {
        const Q: i64 = 3109;
        const N: usize = 512;
        const K: usize = 2;
        let eta = 2;

        let pk = MlwePublicKey::<Q, N, K> {
            a: Mat::from_fn(|| Polynomial::zero()),
            b: Mat::from_fn(|| Polynomial::zero()),
        };
        let ct = pk.encrypt(&Polynomial::zero(), eta, &mut rand::rng());
        let vecs = ct.to_vecs();
        assert_eq!(vecs.len(), K + 1);
        let (ct_extracted, remaining) = MlweCiphertext::<Q, N, K>::extract_from_vecs(vecs).unwrap();
        assert!(remaining.is_empty());
        assert_eq!(ct.u_t.flatten(), ct_extracted.u_t.flatten());
        assert_eq!(ct.v, ct_extracted.v);
    }

    #[test]
    fn test_mlwe_ciphertext_extract_from_vecs_with_remaining() {
        const Q: i64 = 3109;
        const N: usize = 512;
        const K: usize = 2;
        let eta = 2;

        let pk = MlwePublicKey::<Q, N, K> {
            a: Mat::from_fn(|| Polynomial::zero()),
            b: Mat::from_fn(|| Polynomial::zero()),
        };
        let ct = pk.encrypt(&Polynomial::zero(), eta, &mut rand::rng());
        let extra_poly = Polynomial::zero();
        let vecs = ct
            .to_vecs()
            .into_iter()
            .chain(vec![extra_poly.clone()])
            .collect();
        let (ct_extracted, remaining) = MlweCiphertext::<Q, N, K>::extract_from_vecs(vecs).unwrap();
        assert_eq!(remaining.len(), 1);
        assert_eq!(remaining[0], extra_poly);
        assert_eq!(ct.u_t.flatten(), ct_extracted.u_t.flatten());
        assert_eq!(ct.v, ct_extracted.v);
    }

    #[test]
    fn test_mlwe_ciphertext_extract_all_from_vecs() {
        const Q: i64 = 3109;
        const N: usize = 512;
        const K: usize = 2;
        let eta = 2;

        let pk = MlwePublicKey::<Q, N, K> {
            a: Mat::from_fn(|| Polynomial::zero()),
            b: Mat::from_fn(|| Polynomial::zero()),
        };
        let ct1 = pk.encrypt(&Polynomial::zero(), eta, &mut rand::rng());
        let ct2 = pk.encrypt(&Polynomial::zero(), eta, &mut rand::rng());
        let vecs = ct1.to_vecs().into_iter().chain(ct2.to_vecs()).collect();
        let ciphertexts = MlweCiphertext::<Q, N, K>::extract_all_from_vecs(vecs).unwrap();
        assert_eq!(ciphertexts.len(), 2);
        assert_eq!(ct1.u_t.flatten(), ciphertexts[0].u_t.flatten());
        assert_eq!(ct1.v, ciphertexts[0].v);
        assert_eq!(ct2.u_t.flatten(), ciphertexts[1].u_t.flatten());
        assert_eq!(ct2.v, ciphertexts[1].v);
    }

    #[test]
    fn test_mlwe_ciphertext_extract_all_from_vecs_invalid_length() {
        const Q: i64 = 3109;
        const N: usize = 512;
        const K: usize = 2;

        let vecs = vec![Polynomial::zero(); K]; // Not enough polynomials for even one ciphertext
        assert!(MlweCiphertext::<Q, N, K>::extract_all_from_vecs(vecs).is_none());

        let vecs = vec![Polynomial::zero(); K + 2]; // Not a multiple of K + 1
        assert!(MlweCiphertext::<Q, N, K>::extract_all_from_vecs(vecs).is_none());
    }
}
