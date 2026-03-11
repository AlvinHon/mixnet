use poly_ring_xnp1::{Polynomial, zq::ZqI64};
use rand::RngExt;

use crate::ajtai::{commitment::AjtaiCommitment, opening::AjtaiCommitmentOpening};

/// Commitment key: public matrices A1 (N x t), A2 (N x m)
pub struct AjtaiCommitmentKey<const Q: i64, const N: usize, const T: usize, const M: usize> {
    pub(crate) a1: [Polynomial<ZqI64<Q>, N>; T],
    pub(crate) a2: [Polynomial<ZqI64<Q>, N>; M],
}

impl<const Q: i64, const N: usize, const T: usize, const M: usize> AjtaiCommitmentKey<Q, N, T, M> {
    /// Commit to message m (vector of t polynomials), returns (commitment, opening)
    pub fn commit(
        &self,
        m: &[Polynomial<ZqI64<Q>, N>; T],
    ) -> (AjtaiCommitment<Q, N>, AjtaiCommitmentOpening<Q, N, M>) {
        let mut rng = rand::rng();
        let mut r = Vec::with_capacity(M);
        for _ in 0..M {
            let coeffs = (0..N).map(|_| ZqI64::new(rng.random_range(0..Q))).collect();
            r.push(Polynomial::<ZqI64<Q>, N>::new(coeffs));
        }
        // c = sum_i A1_i * m_i + sum_j A2_j * r_j
        let mut c = Polynomial::<ZqI64<Q>, N>::new(vec![ZqI64::new(0); N]);
        for (ai, mi) in self.a1.iter().zip(m.iter()) {
            c = c + ai.clone() * mi.clone();
        }
        for (aj, rj) in self.a2.iter().zip(r.iter()) {
            c = c + aj.clone() * rj.clone();
        }
        (
            AjtaiCommitment { c },
            AjtaiCommitmentOpening {
                r: r.try_into().unwrap(),
            },
        )
    }

    /// Verify commitment
    pub fn verify(
        &self,
        m: &[Polynomial<ZqI64<Q>, N>; T],
        com: &AjtaiCommitment<Q, N>,
        open: &AjtaiCommitmentOpening<Q, N, M>,
    ) -> bool {
        let mut c_check = Polynomial::<ZqI64<Q>, N>::new(vec![ZqI64::new(0); N]);
        for (ai, mi) in self.a1.iter().zip(m.iter()) {
            c_check = c_check + ai.clone() * mi.clone();
        }
        for (aj, rj) in self.a2.iter().zip(open.r.iter()) {
            c_check = c_check + aj.clone() * rj.clone();
        }
        c_check == com.c
    }
}
