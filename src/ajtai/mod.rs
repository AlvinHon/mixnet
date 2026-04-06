//! Ajtai Commitment scheme module

pub mod commitment;
pub use commitment::AjtaiCommitment;
pub mod key;
pub use key::AjtaiCommitmentKey;
pub mod message;
pub use message::AjtaiMessage;
pub mod opening;
pub use opening::AjtaiCommitmentOpening;

#[cfg(test)]
mod tests {
    use super::*;
    use poly_ring_xnp1::{Polynomial, zq::ZqI64, zqi64_vec};

    #[test]
    fn ajtai_commit_verify_roundtrip() {
        const Q: i64 = 3109;
        const N: usize = 4;
        const K1: usize = 2;
        const K2: usize = 4;
        const L: usize = 4;
        let eta = 2;

        let mut rng = rand::rng();
        let key = AjtaiCommitmentKey::<Q, N, K1, K2, L>::new(&mut rng);
        let m = AjtaiMessage::<Q, N, L>::from_polynomials(vec![
            Polynomial::<ZqI64<Q>, N>::new(zqi64_vec![1, 0, 1, 0; Q]),
            Polynomial::<ZqI64<Q>, N>::new(zqi64_vec![0, 1, 0, 1; Q]),
        ]);
        let (com, open) = key.commit(&m, eta, &mut rng);
        assert!(key.verify(&m, &com, &open, eta));
    }
}
