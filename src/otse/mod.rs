//! One time symmetric scheme

pub mod key;
pub use key::OTSEKey;
pub mod params;
pub use params::OTSEParams;
pub mod encoded;
pub use encoded::OTSEEncoded;
pub mod message;
pub use message::OTSEMessage;

#[cfg(test)]
mod tests {
    use poly_ring_xnp1::zq::ZqI64;

    use crate::preliminaries::{algebra::sample_poly_range, mat::Mat};

    use super::*;

    #[test]
    fn test_otse() {
        const Q: i64 = 3109;
        const N: usize = 512;
        const KE: usize = 2;
        const Z: i64 = 64; // 2^6
        const E: i64 = 16; // 2^(2*2)
        const KR: usize = 1;
        const L: usize = 2;
        let mut rng = rand::rng();
        let params = OTSEParams::<Q, N, KE, Z, E, KR, L>::new(2, &mut rng);
        let key = OTSEKey::<Q, N, KE, Z, E, KR, L>::new(&mut rng);
        let m = OTSEMessage::<Q, N, L> {
            m: Mat::<ZqI64<Q>, N, L, 1>::from_fn(|| sample_poly_range::<Q, N, _>(0, 1, &mut rng)),
        };
        let c = key.encode(&m, &params);
        let m_decoded = key.decode(&c, &params);
        assert_eq!(m, m_decoded);
    }
}
