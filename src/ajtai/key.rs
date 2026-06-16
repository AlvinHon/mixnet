use poly_ring_xnp1::zq::ZqI64;
use serde_derive::{Deserialize, Serialize};

use crate::{
    ajtai::{commitment::AjtaiCommitment, message::AjtaiMessage, opening::AjtaiCommitmentOpening},
    preliminaries::{
        algebra::{sample_poly, sample_poly_b_eta},
        mat::Mat,
    },
};

/// Commitment key: public matrices A1 and A2 for the commitment scheme.
/// Parameters:
/// - Q: modulus
/// - N: degree of polynomials
/// - K1: number of columns in A1
/// - K2: number of columns in A2, must be = K1 * 2
/// - L: number of polynomials in the message vector
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AjtaiCommitmentKey<
    const Q: i64,
    const N: usize,
    const K1: usize,
    const K2: usize,
    const L: usize,
> {
    pub(crate) a1: Mat<ZqI64<Q>, N, K1, L>,
    pub(crate) a2: Mat<ZqI64<Q>, N, K1, K2>,
}

impl<const Q: i64, const N: usize, const K1: usize, const K2: usize, const L: usize>
    AjtaiCommitmentKey<Q, N, K1, K2, L>
{
    /// Key generation: sample random A1, A2
    pub fn new<R: rand::RngExt + ?Sized>(rng: &mut R) -> Self {
        let a1 = Mat::<ZqI64<Q>, N, K1, L>::from_fn(|| sample_poly(rng));
        let a2 = Mat::<ZqI64<Q>, N, K1, K2>::from_fn(|| sample_poly(rng));
        Self { a1, a2 }
    }

    /// Commit to message m (vector of t polynomials), returns (commitment, opening)
    pub fn commit<R: rand::RngExt + ?Sized>(
        &self,
        m: &AjtaiMessage<Q, N, L>,
        eta: usize,
        rng: &mut R,
    ) -> (AjtaiCommitment<Q, N, K1>, AjtaiCommitmentOpening<Q, N, K2>) {
        let r = Mat::<ZqI64<Q>, N, K2, 1>::from_fn(|| sample_poly_b_eta(eta, rng));
        let c = self.a1.dot(&m.m).add(&self.a2.dot(&r));
        (AjtaiCommitment { c }, AjtaiCommitmentOpening { r })
    }

    /// Verify commitment
    pub fn verify(
        &self,
        m: &AjtaiMessage<Q, N, L>,
        com: &AjtaiCommitment<Q, N, K1>,
        open: &AjtaiCommitmentOpening<Q, N, K2>,
        eta: usize,
    ) -> bool {
        Self::validate_constraint(m, open, eta) && {
            let c_check = self.a1.dot(&m.m).add(&self.a2.dot(&open.r));
            c_check == com.c
        }
    }

    /// Check the constraint in verify function:
    /// ```text
    /// Set B := Sqrt(n*L + n * 2 * k * η^2) and
    /// ||(m, r)|| <= B
    /// ```
    fn validate_constraint(
        m: &AjtaiMessage<Q, N, L>,
        open: &AjtaiCommitmentOpening<Q, N, K2>,
        eta: usize,
    ) -> bool {
        // combine m and r into a single vector of polynomials
        let mut combined = Vec::with_capacity(L + K2);
        m.m.polynomials.iter().for_each(|col| {
            combined.extend_from_slice(col);
        });
        open.r.polynomials.iter().for_each(|col| {
            combined.extend_from_slice(col);
        });
        // calculate the norm of the combined vector
        let norm_squared: i64 = combined
            .iter()
            .map(|p| p.iter().map(|c| i64::from(c.clone()).pow(2)).sum::<i64>())
            .sum();
        norm_squared <= (N as i64) * (L as i64) + (N as i64) * 2 * (K2 as i64) * (eta as i64).pow(2)
    }
}
