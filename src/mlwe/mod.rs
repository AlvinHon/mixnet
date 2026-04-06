//! MLWE encryption scheme module

pub mod ciphertext;
pub use ciphertext::MlweCiphertext;
pub mod decrypt;
pub use decrypt::MlweSecretKey;
pub mod encrypt;
pub use encrypt::MlwePublicKey;

/// Key generation for MLWE
pub fn keygen<const Q: i64, const N: usize, const K: usize, R: rand::RngExt + ?Sized>(
    eta: usize,
    rng: &mut R,
) -> (MlwePublicKey<Q, N, K>, MlweSecretKey<Q, N, K>) {
    use crate::preliminaries::{
        algebra::{sample_poly, sample_poly_b_eta},
        mat::Mat,
    };
    use poly_ring_xnp1::zq::ZqI64;

    let a = Mat::<ZqI64<Q>, N, K, K>::from_fn(|| sample_poly::<Q, N, _>(rng));
    let s = Mat::<ZqI64<Q>, N, K, 1>::from_fn(|| sample_poly_b_eta::<Q, N, _>(eta, rng));
    let e = Mat::<ZqI64<Q>, N, K, 1>::from_fn(|| sample_poly_b_eta::<Q, N, _>(eta, rng));
    let b = a.dot(&s).add(&e);
    (MlwePublicKey { a, b }, MlweSecretKey { s })
}

#[cfg(test)]
mod tests {
    use super::*;
    use poly_ring_xnp1::{Polynomial, zq::ZqI64};

    #[test]
    fn mlwe_encrypt_decrypt_roundtrip() {
        let mut rng = rand::rng();
        const Q: i64 = 3109;
        const N: usize = 512;
        const K: usize = 2;
        let eta = 2;

        let mut failures = 0;
        let trials = 100;
        for _ in 0..trials {
            // Random bnary message
            let m = Polynomial::<ZqI64<Q>, N>::new(
                (0..N)
                    .map(|_| ZqI64::new(rand::random_range(0..2)))
                    .collect(),
            );
            let (pk, sk) = keygen::<Q, N, K, _>(eta, &mut rng);
            let ct = pk.encrypt(&m, eta, &mut rng);
            let m_rec = sk.decrypt(&ct);
            if !m.iter().zip(m_rec.iter()).all(|(a, b)| a == b) {
                failures += 1;
            }
        }
        println!("Decryption failures: {} out of {}", failures, trials);
        assert!(
            // Given the these parameters, failure should be surprising even for the probabilistic nature of MLWE
            failures == 0,
            "Too many decryption failures: {} out of {}",
            failures,
            trials
        );
    }
}
