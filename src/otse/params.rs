use poly_ring_xnp1::{Polynomial, zq::ZqI64};

use crate::preliminaries::{algebra::sample_poly, mat::matrix_from_fn};

#[derive(Clone)]
pub struct OTSEParams<
    const Q: i64,
    const N: usize,
    const KE: usize, // K_lwe
    const Z: i64,    // 2^z
    const E: i64,    // 2^(2*eta)
    const KR: usize, // K_lwr
    const L: usize,
> {
    pub eta: usize,
    pub h: Vec<Vec<Polynomial<ZqI64<Z>, N>>>,
    pub h1: Vec<Vec<Polynomial<ZqI64<Q>, N>>>,
}

impl<
    const Q: i64,
    const N: usize,
    const KE: usize,
    const Z: i64,
    const E: i64,
    const KR: usize,
    const L: usize,
> OTSEParams<Q, N, KE, Z, E, KR, L>
{
    pub fn new<R: rand::RngExt + ?Sized>(eta: usize, rng: &mut R) -> Self {
        assert_eq!(E, 2_i64.pow(2 * eta as u32));
        let h = matrix_from_fn(KE + L, KR, || sample_poly::<Z, N, R>(rng));
        let h1 = matrix_from_fn(L, KE, || sample_poly::<Q, N, R>(rng));
        Self { eta, h, h1 }
    }
}

pub fn create_default_params<const L: usize, R: rand::RngExt + ?Sized>(
    rng: &mut R,
) -> OTSEParams<3109, 512, 2, 64, 16, 2, L> {
    OTSEParams::<3109, 512, 2, 64, 16, 2, L>::new(2, rng)
}
