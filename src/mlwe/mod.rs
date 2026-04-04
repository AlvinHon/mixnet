//! MLWE encryption scheme module

pub mod ciphertext;
pub use ciphertext::MlweCiphertext;
pub mod decrypt;
pub use decrypt::MlweSecretKey;
pub mod encrypt;
pub use encrypt::MlwePublicKey;

use crate::preliminaries::algebra::sample_b_eta;
use poly_ring_xnp1::{Polynomial, zq::ZqI64};

/// Key generation for MLWE
pub fn keygen<const Q: i64, const N: usize, R: rand::RngExt + ?Sized>(
    eta: usize,
    rng: &mut R,
) -> (MlwePublicKey<Q, N>, MlweSecretKey<Q, N>) {
    let a = sample_poly::<Q, N, _>(rng);
    let s = sample_poly_b_eta::<Q, N, _>(eta, rng);
    let e = sample_poly_b_eta::<Q, N, _>(eta, rng);
    let b = a.clone() * s.clone() + e;
    (MlwePublicKey { a, b }, MlweSecretKey { s })
}

/// Sample a random polynomial in Zq[X]/(X^N+1)
pub(crate) fn sample_poly<const Q: i64, const N: usize, R: rand::RngExt + ?Sized>(
    rng: &mut R,
) -> Polynomial<ZqI64<Q>, N> {
    let coeffs = (0..N).map(|_| ZqI64::new(rng.random_range(0..Q))).collect();
    Polynomial::<ZqI64<Q>, N>::new(coeffs)
}

/// Sample a polynomial with coefficients from B_eta
pub(crate) fn sample_poly_b_eta<const Q: i64, const N: usize, R: rand::RngExt + ?Sized>(
    eta: usize,
    rng: &mut R,
) -> Polynomial<ZqI64<Q>, N> {
    let coeffs = (0..N).map(|_| sample_b_eta::<Q, R>(eta, rng)).collect();
    Polynomial::<ZqI64<Q>, N>::new(coeffs)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn mlwe_encrypt_decrypt_roundtrip() {
        let mut rng = rand::rng();
        const Q: i64 = 3109;
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
            let (pk, sk) = keygen::<Q, N, _>(eta, &mut rng);
            let ct = pk.encrypt(&m, eta, &mut rng);
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
