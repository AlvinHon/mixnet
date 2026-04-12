use poly_ring_xnp1::{Polynomial, zq::ZqI64};

use crate::{
    otse::{OTSEEncoded, OTSEMessage, OTSEParams},
    preliminaries::{
        algebra::{binomial_matrix, sample_poly_range, stacked_ring_to_bin},
        mat::{Mat, matrix_change_modulus, matrix_extend_with, matrix_identity, matrix_multiply},
    },
};

/// One time symmetric scheme key
pub struct OTSEKey<
    const Q: i64,
    const N: usize,
    const KE: usize, // K_lwe
    const Z: i64,    // 2^z
    const E: i64,    // 2^(2*eta)
    const KR: usize, // K_lwr
    const L: usize,
> {
    pub(crate) s: Mat<ZqI64<Q>, N, KR, 1>,
}

impl<
    const Q: i64,
    const N: usize,
    const KE: usize,
    const Z: i64,
    const E: i64,
    const KR: usize,
    const L: usize,
> OTSEKey<Q, N, KE, Z, E, KR, L>
{
    pub fn new<R: rand::RngExt + ?Sized>(rng: &mut R) -> Self {
        Self {
            s: Mat::<ZqI64<Q>, N, KR, 1>::from_fn(|| sample_poly_range::<Q, N, R>(0, 1, rng)),
        }
    }

    pub fn encode(
        &self,
        message: &OTSEMessage<Q, N, L>,
        params: &OTSEParams<Q, N, KE, Z, E, KR, L>,
    ) -> OTSEEncoded<Q, N, L> {
        // d = [h * s]_{Z -> E}
        let d: Vec<Polynomial<ZqI64<E>, N>> = {
            // Change to modulus Q for multiplication with s
            let h = matrix_change_modulus(params.h.clone());
            // compute [H * s]
            let t = matrix_multiply(&h, &Vec::from(self.s.clone()));
            // perform rounding: [H *s]_{Z -> E}
            let d: Vec<Vec<Polynomial<ZqI64<E>, N>>> = matrix_change_modulus(t);
            // d has dimension (KE + L) x 1
            let mut r = Vec::with_capacity(KE + L);
            for i in 0..(KE + L) {
                r.push(d[i][0].clone());
            }
            r
        };
        // h' = (h" | I_L)
        let mut h1 = params.h1.clone();
        let i = matrix_identity(L);
        matrix_extend_with(&i, &mut h1);
        // dc = stack((d(0),...,d(2*eta-1))) = stack(ring_to_bin(d, 2*eta))
        let dc = {
            let t = stacked_ring_to_bin(&d, 2 * params.eta);
            // convert to matrix of size (2*eta) x 1
            let mut r = Vec::with_capacity(2 * params.eta);
            for i in 0..(2 * params.eta) {
                r.push(vec![t[i].clone()]);
            }
            // Change to modulus Q for multiplication with h'
            matrix_change_modulus(r)
        };
        // a = H' * B^(eta)_N * dc
        let a = {
            let bn = binomial_matrix::<N, Q>(KE + L, params.eta);
            // h1: L x (KE + L), bn: (KE + L) x (2*eta), dc: (2*eta) x 1
            let t = matrix_multiply(&h1, &bn);
            Mat::from(matrix_multiply(&t, &dc))
        };
        OTSEEncoded {
            c: message.m.add(&a),
        }
    }

    pub fn decode(
        &self,
        encoded: &OTSEEncoded<Q, N, L>,
        params: &OTSEParams<Q, N, KE, Z, E, KR, L>,
    ) -> OTSEMessage<Q, N, L> {
        // d = [h * s]_{Z -> E}
        let d: Vec<Polynomial<ZqI64<E>, N>> = {
            // Change to modulus Q for multiplication with s
            let h = matrix_change_modulus(params.h.clone());
            // compute [H * s]
            let t = matrix_multiply(&h, &Vec::from(self.s.clone()));
            // perform rounding: [H *s]_{Z -> E}
            let d: Vec<Vec<Polynomial<ZqI64<E>, N>>> = matrix_change_modulus(t);
            // d has dimension (KE + L) x 1
            let mut r = Vec::with_capacity(KE + L);
            for i in 0..(KE + L) {
                r.push(d[i][0].clone());
            }
            r
        };
        // h' = (h" | I_L)
        let mut h1 = params.h1.clone();
        let i = matrix_identity(L);
        matrix_extend_with(&i, &mut h1);
        // dc = stack((d(0),...,d(2*eta-1))) = stack(ring_to_bin(d, 2*eta))
        let dc = {
            let t = stacked_ring_to_bin(&d, 2 * params.eta);
            // convert to matrix of size (2*eta) x 1
            let mut r = Vec::with_capacity(2 * params.eta);
            for i in 0..(2 * params.eta) {
                r.push(vec![t[i].clone()]);
            }
            // Change to modulus Q for multiplication with h'
            matrix_change_modulus(r)
        };
        // a = H' * B^(eta)_N * dc
        let a = {
            let bn = binomial_matrix::<N, Q>(KE + L, params.eta);
            // h1: L x (KE + L), bn: (KE + L) x (2*eta), dc: (2*eta) x 1
            let t = matrix_multiply(&h1, &bn);
            Mat::from(matrix_multiply(&t, &dc))
        };
        OTSEMessage {
            m: encoded.c.sub(&a),
        }
    }
}
