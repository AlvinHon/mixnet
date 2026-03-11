//! Ajtai Commitment scheme module

pub mod commitment;
pub mod key;
pub mod opening;

use crate::ajtai::key::AjtaiCommitmentKey;
use poly_ring_xnp1::{Polynomial, zq::ZqI64};
use rand::RngExt;

/// Key generation: sample random A1, A2
pub fn keygen<const Q: i64, const N: usize, const T: usize, const M: usize>()
-> AjtaiCommitmentKey<Q, N, T, M> {
    let mut rng = rand::rng();
    let mut a1 = Vec::with_capacity(T);
    for _ in 0..T {
        let coeffs = (0..N).map(|_| ZqI64::new(rng.random_range(0..Q))).collect();
        a1.push(Polynomial::<ZqI64<Q>, N>::new(coeffs));
    }
    let mut a2 = Vec::with_capacity(M);
    for _ in 0..M {
        let coeffs = (0..N).map(|_| ZqI64::new(rng.random_range(0..Q))).collect();
        a2.push(Polynomial::<ZqI64<Q>, N>::new(coeffs));
    }
    AjtaiCommitmentKey {
        a1: a1.try_into().unwrap(),
        a2: a2.try_into().unwrap(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ajtai_commit_verify_roundtrip() {
        const Q: i64 = 12289;
        const N: usize = 4;
        const T: usize = 2;
        const M: usize = 2;
        let key = keygen::<Q, N, T, M>();
        let m = [
            Polynomial::<ZqI64<Q>, N>::new(vec![
                ZqI64::new(1),
                ZqI64::new(2),
                ZqI64::new(3),
                ZqI64::new(4),
            ]),
            Polynomial::<ZqI64<Q>, N>::new(vec![
                ZqI64::new(5),
                ZqI64::new(6),
                ZqI64::new(7),
                ZqI64::new(8),
            ]),
        ];
        let (com, open) = key.commit(&m);
        assert!(key.verify(&m, &com, &open));
    }
}
