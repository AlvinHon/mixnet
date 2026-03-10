//! MLWE encryption scheme implementation

use crate::preliminaries::algebra::sample_b_eta;
use poly_ring_xnp1::Polynomial;
use poly_ring_xnp1::zq::ZqI64;
use rand::RngExt;

/// MLWE public key type
pub struct MlwePublicKey<const Q: i64, const N: usize> {
    pub a: Polynomial<ZqI64<Q>, N>,
    pub b: Polynomial<ZqI64<Q>, N>,
}

/// MLWE secret key type
pub struct MlweSecretKey<const Q: i64, const N: usize> {
    pub s: Polynomial<ZqI64<Q>, N>,
}

/// MLWE ciphertext type
pub struct MlweCiphertext<const Q: i64, const N: usize> {
    pub u: Polynomial<ZqI64<Q>, N>,
    pub v: Polynomial<ZqI64<Q>, N>,
}

/// Sample a random polynomial in Zq[X]/(X^N+1)
pub fn sample_poly<const Q: i64, const N: usize>() -> Polynomial<ZqI64<Q>, N> {
    let mut rng = rand::rng();
    let coeffs = (0..N).map(|_| ZqI64::new(rng.random_range(0..Q))).collect();
    Polynomial::<ZqI64<Q>, N>::new(coeffs)
}

/// Sample a polynomial with coefficients from B_eta
pub fn sample_poly_b_eta<const Q: i64, const N: usize>(eta: usize) -> Polynomial<ZqI64<Q>, N> {
    let coeffs = (0..N).map(|_| sample_b_eta::<Q>(eta)).collect();
    Polynomial::<ZqI64<Q>, N>::new(coeffs)
}

/// Key generation for MLWE
pub fn mlwe_keygen<const Q: i64, const N: usize>(
    eta: usize,
) -> (MlwePublicKey<Q, N>, MlweSecretKey<Q, N>) {
    let a = sample_poly::<Q, N>();
    let s = sample_poly_b_eta::<Q, N>(eta);
    let e = sample_poly_b_eta::<Q, N>(eta);
    let b = a.clone() * s.clone() + e;
    (MlwePublicKey { a, b }, MlweSecretKey { s })
}

/// MLWE encryption: encrypts message m
pub fn mlwe_encrypt<const Q: i64, const N: usize>(
    pk: &MlwePublicKey<Q, N>,
    m: &Polynomial<ZqI64<Q>, N>,
    eta: usize,
) -> MlweCiphertext<Q, N> {
    let r = sample_poly_b_eta::<Q, N>(eta);
    let e1 = sample_poly_b_eta::<Q, N>(eta);
    let e2 = sample_poly_b_eta::<Q, N>(eta);
    let u = pk.a.clone() * r.clone() + e1;
    let v = pk.b.clone() * r + e2 + m.clone();
    MlweCiphertext { u, v }
}

/// MLWE decryption: recovers message m
pub fn mlwe_decrypt<const Q: i64, const N: usize>(
    sk: &MlweSecretKey<Q, N>,
    ct: &MlweCiphertext<Q, N>,
) -> Polynomial<ZqI64<Q>, N> {
    ct.v.clone() - sk.s.clone() * ct.u.clone()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn mlwe_encrypt_decrypt_roundtrip() {
        const Q: i64 = 12289;
        const N: usize = 4;
        let eta = 4;
        let (pk, sk) = mlwe_keygen::<Q, N>(eta);
        let m = sample_poly::<Q, N>();
        let ct = mlwe_encrypt::<Q, N>(&pk, &m, eta);
        let m_rec = mlwe_decrypt::<Q, N>(&sk, &ct);
        for (a, b) in m.iter().zip(m_rec.iter()) {
            assert!((i64::from(a.clone()) - i64::from(b.clone())).abs() < Q / 8);
        }
    }
}
