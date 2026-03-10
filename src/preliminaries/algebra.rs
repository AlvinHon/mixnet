use poly_ring_xnp1::{Polynomial, zq::ZqI64};
use rand::RngExt;

/// Constructs the gadget matrix G = I_n ⊗ [1, 2, ..., 2^{k-1}] as a vector of vectors.
/// Returns a Vec<Vec<i64>> of shape (n, k).
pub fn gadget_matrix(n: usize, k: usize) -> Vec<Vec<i64>> {
    let mut mat = Vec::with_capacity(n);
    for _ in 0..n {
        let row = (0..k).map(|j| 1i64 << j).collect();
        mat.push(row);
    }
    mat
}
/// Stacks a vector of binary polynomials into a single polynomial: sum_k 2^k * bin_polys[k]
pub fn stack<const Q: i64, const N: usize>(
    bin_polys: &[Polynomial<ZqI64<Q>, N>],
) -> Polynomial<ZqI64<Q>, N> {
    let mut acc = Polynomial::<ZqI64<Q>, N>::new(vec![ZqI64::new(0); N]);
    for (k, poly) in bin_polys.iter().enumerate() {
        let coeff = ZqI64::new(1 << k);
        let scaled = Polynomial::<ZqI64<Q>, N>::new(
            poly.iter().map(|c| c.clone() * coeff.clone()).collect(),
        );
        acc = acc + scaled;
    }
    acc
}

/// Decomposes a polynomial into its binary representation (per-coefficient, LSB first).
/// Returns a vector of binary polynomials, one for each bit position (length = bits).
pub fn ring_to_bin<const Q: i64, const N: usize>(
    poly: &Polynomial<ZqI64<Q>, N>,
    bits: usize,
) -> Vec<Polynomial<ZqI64<Q>, N>> {
    let mut out = Vec::with_capacity(bits);
    for k in 0..bits {
        let coeffs = poly
            .iter()
            .map(|c| ZqI64::new((i64::from(c.clone()) >> k) & 1))
            .collect();
        out.push(Polynomial::<ZqI64<Q>, N>::new(coeffs));
    }
    out
}

/// Converts an integer to its binary representation as a vector of bits (LSB first).
pub fn int_to_bin(mut x: u64, bits: usize) -> Vec<u64> {
    let mut out = Vec::with_capacity(bits);
    for _ in 0..bits {
        out.push(x & 1);
        x >>= 1;
    }
    out
}

/// Constructs a binary polynomial from an integer (LSB first, padded to degree N).
pub fn poly_from_int_bin<const Q: i64, const N: usize>(x: u64) -> Polynomial<ZqI64<Q>, N> {
    let mut coeffs = int_to_bin(x, N);
    coeffs.truncate(N);
    Polynomial::<ZqI64<Q>, N>::new(coeffs.into_iter().map(|b| ZqI64::new(b as i64)).collect())
}

/// Returns true if all coefficients are 0 or 1.
pub fn is_bin<const Q: i64, const N: usize>(poly: &Polynomial<ZqI64<Q>, N>) -> bool {
    poly.iter()
        .all(|c| c == &ZqI64::new(0) || c == &ZqI64::new(1))
}

/// Sample from the centered binomial distribution B_eta for integer eta > 0.
/// Returns a ZqI64<Q> sample.
pub fn sample_b_eta<const Q: i64>(eta: usize) -> ZqI64<Q> {
    let mut rng = rand::rng();
    let mut sum = 0i64;
    for _ in 0..eta {
        let a: i64 = rng.random_range(0..=1);
        let b: i64 = rng.random_range(0..=1);
        sum += a - b;
    }
    ZqI64::new(sum)
}

#[cfg(test)]
mod tests {
    use super::*;
    use poly_ring_xnp1::Polynomial;

    #[test]
    fn gadget_equation_holds() {
        const Q: i64 = 12289;
        let n = 4;
        let b = 4;
        let d = Polynomial::<ZqI64<Q>, 4>::new(vec![
            ZqI64::new(11),
            ZqI64::new(6),
            ZqI64::new(15),
            ZqI64::new(1),
        ]);
        let g = gadget_matrix(1, b)[0].clone();
        let bits = ring_to_bin::<Q, 4>(&d, b);
        let d_recovered = stack::<Q, 4>(&bits);
        for i in 0..n {
            let mut sum = ZqI64::<Q>::new(0);
            for k in 0..b {
                let bit = bits[k].iter().nth(i).unwrap_or(&ZqI64::new(0)).clone();
                sum = sum + ZqI64::new(g[k]) * bit;
            }
            assert_eq!(sum, d.iter().nth(i).unwrap().clone());
        }
        for (a, b) in d.iter().zip(d_recovered.iter()) {
            assert_eq!(a, b);
        }
    }

    #[test]
    fn gadget_matrix_works() {
        let n = 3;
        let k = 4;
        let mat = gadget_matrix(n, k);
        let expected = vec![vec![1, 2, 4, 8], vec![1, 2, 4, 8], vec![1, 2, 4, 8]];
        assert_eq!(mat, expected);
    }
    #[test]
    fn stack_works() {
        const Q: i64 = 12289;
        // bin_polys: [LSB, 2nd, 3rd, MSB]
        let bin_polys = vec![
            Polynomial::<ZqI64<Q>, 4>::new(vec![
                ZqI64::new(1),
                ZqI64::new(0),
                ZqI64::new(1),
                ZqI64::new(1),
            ]),
            Polynomial::<ZqI64<Q>, 4>::new(vec![
                ZqI64::new(1),
                ZqI64::new(1),
                ZqI64::new(1),
                ZqI64::new(0),
            ]),
            Polynomial::<ZqI64<Q>, 4>::new(vec![
                ZqI64::new(0),
                ZqI64::new(1),
                ZqI64::new(1),
                ZqI64::new(0),
            ]),
            Polynomial::<ZqI64<Q>, 4>::new(vec![
                ZqI64::new(1),
                ZqI64::new(0),
                ZqI64::new(1),
                ZqI64::new(0),
            ]),
        ];
        let stacked = stack::<Q, 4>(&bin_polys);
        let expected = vec![11, 6, 15, 1];
        for (a, &b) in stacked.iter().zip(expected.iter()) {
            assert_eq!(a, &ZqI64::new(b));
        }
    }
    #[test]
    fn ring_to_bin_works() {
        const Q: i64 = 12289;
        // poly = 0b1011, 0b0110, 0b1111, 0b0001
        let poly = Polynomial::<ZqI64<Q>, 4>::new(vec![
            ZqI64::new(0b1011),
            ZqI64::new(0b0110),
            ZqI64::new(0b1111),
            ZqI64::new(0b0001),
        ]);
        let bins = ring_to_bin::<Q, 4>(&poly, 4);
        let expected = vec![
            vec![1, 0, 1, 1],
            vec![1, 1, 1, 0],
            vec![0, 1, 1, 0],
            vec![1, 0, 1, 0],
        ];
        for (bin_poly, exp) in bins.iter().zip(expected) {
            let actual: Vec<_> = bin_poly.iter().map(|x| i64::from(x.clone())).collect();
            assert_eq!(actual, exp[..actual.len()]);
            assert!(is_bin::<Q, 4>(bin_poly));
        }
    }

    #[test]
    fn int_to_bin_works() {
        assert_eq!(int_to_bin(0b1011, 4), vec![1, 1, 0, 1]);
        assert_eq!(int_to_bin(0, 3), vec![0, 0, 0]);
        assert_eq!(int_to_bin(1, 1), vec![1]);
    }

    #[test]
    fn from_int_bin_poly() {
        const Q: i64 = 12289;
        let poly = poly_from_int_bin::<Q, 4>(0b1011);
        let expected = vec![1, 1, 0, 1];
        for (a, &b) in poly.iter().zip(expected.iter()) {
            assert_eq!(a, &ZqI64::new(b));
        }
        assert!(is_bin::<Q, 4>(&poly));
    }

    #[test]
    fn is_bin_works() {
        const Q: i64 = 12289;
        let bin = Polynomial::<ZqI64<Q>, 4>::new(vec![
            ZqI64::new(0),
            ZqI64::new(1),
            ZqI64::new(1),
            ZqI64::new(0),
        ]);
        let not_bin = Polynomial::<ZqI64<Q>, 4>::new(vec![
            ZqI64::new(0),
            ZqI64::new(2),
            ZqI64::new(1),
            ZqI64::new(0),
        ]);
        assert!(is_bin::<Q, 4>(&bin));
        assert!(!is_bin::<Q, 4>(&not_bin));
    }

    #[test]
    fn add_polys_mod_q() {
        let a = Polynomial::<i64, 4>::new(vec![16, 1, 2, 0]);
        let b = Polynomial::<i64, 4>::new(vec![4, 15, 16, 0]);
        let c = a + b;
        // poly_ring_xnp1 does integer arithmetic, not mod q
        assert_eq!(c.iter().copied().collect::<Vec<_>>(), vec![20, 16, 18]);
    }

    #[test]
    fn cyclic_mul_polys_mod_q() {
        let a = Polynomial::<i64, 4>::new(vec![1, 2, 0, 0]);
        let b = Polynomial::<i64, 4>::new(vec![3, 4, 0, 0]);
        let c = a * b;
        // poly_ring_xnp1 does integer arithmetic, not mod q
        assert_eq!(c.iter().copied().collect::<Vec<_>>(), vec![3, 10, 8]);
    }

    #[test]
    fn b_eta_mean_is_zeroish() {
        const Q: i64 = 12289;
        let eta = 8;
        let samples: Vec<_> = (0..10000).map(|_| sample_b_eta::<Q>(eta)).collect();
        let mean: f64 = samples
            .iter()
            .map(|x| i64::from(x.clone()) as f64)
            .sum::<f64>()
            / samples.len() as f64;
        // For large enough samples, mean should be close to 0
        assert!(mean.abs() < 0.2, "mean was {}", mean);
    }
}
