//! MLWE encryption scheme module

pub mod ciphertext;
pub use ciphertext::MlweCiphertext;
pub mod decrypt;
pub use decrypt::MlweSecretKey;
pub mod encrypt;
pub use encrypt::MlwePublicKey;

use crate::preliminaries::algebra::sample_b_eta;
use poly_ring_xnp1::{Polynomial, zq::ZqI64};
use rand::RngExt;

/// Key generation for MLWE
pub fn keygen<const Q: i64, const N: usize>(
    eta: usize,
) -> (MlwePublicKey<Q, N>, MlweSecretKey<Q, N>) {
    let a = sample_poly::<Q, N>();
    let s = sample_poly_b_eta::<Q, N>(eta);
    let e = sample_poly_b_eta::<Q, N>(eta);
    let b = a.clone() * s.clone() + e;
    (MlwePublicKey { a, b }, MlweSecretKey { s })
}

/// Sample a random polynomial in Zq[X]/(X^N+1)
pub(crate) fn sample_poly<const Q: i64, const N: usize>() -> Polynomial<ZqI64<Q>, N> {
    let mut rng = rand::rng();
    let coeffs = (0..N).map(|_| ZqI64::new(rng.random_range(0..Q))).collect();
    Polynomial::<ZqI64<Q>, N>::new(coeffs)
}

/// Sample a polynomial with coefficients from B_eta
pub(crate) fn sample_poly_b_eta<const Q: i64, const N: usize>(
    eta: usize,
) -> Polynomial<ZqI64<Q>, N> {
    let coeffs = (0..N).map(|_| sample_b_eta::<Q>(eta)).collect();
    Polynomial::<ZqI64<Q>, N>::new(coeffs)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn mlwe_encrypt_decrypt_roundtrip() {
        const Q: i64 = 3019;
        const N: usize = 512;
        let eta = 2;
        // Binary message
        let m = Polynomial::<ZqI64<Q>, N>::new(vec![
            ZqI64::new(1),
            ZqI64::new(0),
            ZqI64::new(1),
            ZqI64::new(0),
        ]);
        let mut failures = 0;
        let trials = 100;
        for _ in 0..trials {
            let (pk, sk) = keygen::<Q, N>(eta);
            let ct = pk.encrypt(&m, eta);
            let m_rec = sk.decrypt(&ct);
            if !m.iter().zip(m_rec.iter()).all(|(a, b)| a == b) {
                failures += 1;
            }
        }
        println!("Decryption failures: {} out of {}", failures, trials);
        assert!(
            failures <= 10, // TODO it may be too large!
            "Too many decryption failures: {} out of {}",
            failures,
            trials
        );
    }
}
