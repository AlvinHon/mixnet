use poly_ring_xnp1::{Polynomial, zq::ZqI64};

use crate::preliminaries::algebra::sample_poly;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct HpkeMessage<const Q: i64, const N: usize, const L: usize> {
    pub(crate) m: [Polynomial<ZqI64<Q>, N>; L],
}

impl<const Q: i64, const N: usize, const L: usize> HpkeMessage<Q, N, L> {
    pub fn random<R: rand::Rng + ?Sized>(rng: &mut R) -> Self {
        let m = [(); L].map(|_| sample_poly(rng));
        Self { m }
    }
}

impl<const Q: i64, const N: usize, const L: usize> From<[Polynomial<ZqI64<Q>, N>; L]>
    for HpkeMessage<Q, N, L>
{
    fn from(m: [Polynomial<ZqI64<Q>, N>; L]) -> Self {
        Self { m }
    }
}

impl<const Q: i64, const N: usize, const L: usize> TryFrom<[Vec<i64>; L]> for HpkeMessage<Q, N, L> {
    type Error = &'static str;

    fn try_from(m_vec: [Vec<i64>; L]) -> Result<Self, Self::Error> {
        // all elements must be in \[-Q/2, Q/2\]
        if m_vec
            .iter()
            .any(|m_i| m_i.iter().any(|&x| x < -Q / 2 || x > Q / 2))
        {
            return Err("Message elements must be in [-Q/2, Q/2]");
        }

        let m = m_vec.map(|m_i| {
            Polynomial::from_coeffs(m_i.into_iter().map(ZqI64::<Q>::new).collect::<Vec<_>>())
        });
        Ok(Self { m })
    }
}

impl<const Q: i64, const N: usize, const L: usize> TryInto<[Vec<i64>; L]> for HpkeMessage<Q, N, L> {
    type Error = &'static str;

    fn try_into(self) -> Result<[Vec<i64>; L], Self::Error> {
        use num::ToPrimitive;
        let v = self
            .m
            .into_iter()
            .map(|m_i| m_i.to_coeffs())
            .map(|coeffs| {
                coeffs
                    .into_iter()
                    .map(|x| x.to_i64().unwrap()) // it is safe to unwrap since ZqI64 always fits in i64
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        if v.len() != L {
            return Err("Message length mismatch");
        }

        let arr = core::array::from_fn(|i| v[i].clone());
        Ok(arr)
    }
}
