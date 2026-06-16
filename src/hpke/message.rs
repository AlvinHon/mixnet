use poly_ring_xnp1::{Polynomial, zq::ZqI64};
use serde::Serialize;

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

impl<const Q: i64, const N: usize, const L: usize> Serialize for HpkeMessage<Q, N, L> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let m_vecs = self.m.to_vec();
        m_vecs.serialize(serializer)
    }
}

impl<'de, const Q: i64, const N: usize, const L: usize> serde::Deserialize<'de>
    for HpkeMessage<Q, N, L>
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let m_vecs: Vec<Polynomial<ZqI64<Q>, N>> = Vec::deserialize(deserializer)?;
        if m_vecs.len() != L {
            return Err(serde::de::Error::custom(format!(
                "Expected {} polynomials, got {}",
                L,
                m_vecs.len()
            )));
        }
        let m = core::array::from_fn(|i| m_vecs[i].clone());
        Ok(Self { m })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize_deserialize() {
        let msg = HpkeMessage::<17, 4, 2> {
            m: [
                Polynomial::from_coeffs(vec![1.into(), 2.into(), 3.into(), 4.into()]),
                Polynomial::from_coeffs(vec![5.into(), 6.into(), 7.into(), 8.into()]),
            ],
        };
        let serialized = serde_json::to_string(&msg).unwrap();
        let deserialized: HpkeMessage<17, 4, 2> = serde_json::from_str(&serialized).unwrap();
        assert_eq!(msg, deserialized);
    }
}
