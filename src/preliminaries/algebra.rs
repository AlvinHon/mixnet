use num::{One, Zero};
use poly_ring_xnp1::{Polynomial, zq::ZqI64};

/// Constructs the gadget matrix G = (I_N | 2*I_N | ... | 2^{k-2}*I_N | -2^{k-1}*I_N) as matrix of size N x (N*k).
pub fn gadget_matrix<const N: usize, const Q: i64>(k: usize) -> Vec<Vec<Polynomial<ZqI64<Q>, N>>> {
    let mut g = Vec::with_capacity(N);
    for i in 0..N {
        let mut row = Vec::with_capacity(N * k);
        for j in 0..(N * k) {
            let power = j / N;
            let coeff = if power < k - 1 {
                ZqI64::new(1 << power)
            } else {
                ZqI64::new(-(1 << (k - 1)))
            };
            if j % N == i {
                // This is the diagonal element, set to coeff
                let mut poly = Polynomial::<ZqI64<Q>, N>::one();
                poly.coeffs_mut(|c| *c = coeff.clone());
                row.push(poly);
            } else {
                // This is the off-diagonal element, set to 0
                row.push(Polynomial::<ZqI64<Q>, N>::zero());
            }
        }
        g.push(row);
    }
    g
}

/// B^(eta) = (I | ... | I | -I | ... | -I) where there are eta I and eta -I, as a matrix of size l x (2*eta).
pub fn binomial_matrix<const N: usize, const Q: i64>(
    l: usize,
    eta: usize,
) -> Vec<Vec<Polynomial<ZqI64<Q>, N>>> {
    let mut b = Vec::with_capacity(l);
    for i in 0..l {
        let mut row = Vec::with_capacity(2 * eta);
        for j in 0..(2 * eta) {
            let coeff = if j < eta {
                ZqI64::new(1)
            } else {
                ZqI64::new(-1)
            };
            if j % l == i {
                // This is the diagonal element, set to coeff
                let mut poly = Polynomial::<ZqI64<Q>, N>::one();
                poly.coeffs_mut(|c| *c = coeff.clone());
                row.push(poly);
            } else {
                // This is the off-diagonal element, set to 0
                row.push(Polynomial::<ZqI64<Q>, N>::zero());
            }
        }
        b.push(row);
    }
    b
}

/// Decomposes a vector of polynomials into b binary vectors whose coefficients are the binary
/// decomposition of the coefficients of the input polynomials in two's complement form, and
/// then stacks the b binary vectors into one vector of polynomials.
///
/// Combining ring_to_bin with stack in this function because in the paper, the binary vectors are always
/// stacked after decomposition.
///
/// ```text
/// E.g. [[1,2], [3,-4]] with b=3 (in range [-4, 4) ),
/// First bit of the coefficients in [1, 2] is [1, 0] and in [3, -4] is [1, 0] (since -4 is 0b100 in two's complement).
/// So the first output vector d(0) is [[1, 0], [1, 0]].
/// As a result, d(1) is [[0, 1], [1, 0]] and d(2) is [[0, 0], [1, 1]].
/// Output = stack([d(0), d(1), d(2)])
///        = stack([[[1, 0], [1, 0]], [[0, 1], [1, 0]], [[0, 0], [1, 1]]])
///        = [[1, 0], [1, 0], [0, 1], [1, 0], [0, 0], [1, 1]].
/// ```
pub fn stacked_ring_to_bin<const Q: i64, const N: usize>(
    polys: &[Polynomial<ZqI64<Q>, N>],
    bits: usize,
) -> Vec<Polynomial<ZqI64<Q>, N>> {
    let mut bin_polys = Vec::with_capacity(polys.len() * bits);
    for k in 0..bits {
        for poly in polys {
            let coeffs = (0..N)
                .map(|l| {
                    let coeff = poly.coefficient(l);
                    let bit = (i64::from(coeff) >> k) & 1;
                    ZqI64::new(bit)
                })
                .collect();
            bin_polys.push(Polynomial::<ZqI64<Q>, N>::new(coeffs));
        }
    }
    bin_polys
}

/// Converts an integer to its binary representation as a vector of bits (LSB first).
pub fn int_to_bin<const N: usize, const Q: i64>(
    mut x: u64,
    bits: usize,
) -> Polynomial<ZqI64<Q>, N> {
    let mut coeffs = Vec::with_capacity(bits);
    for _ in 0..bits {
        coeffs.push(ZqI64::new((x & 1) as i64));
        x >>= 1;
    }
    // Pad with zeros if bits < N
    coeffs.resize(N, ZqI64::new(0));
    Polynomial::<ZqI64<Q>, N>::new(coeffs)
}

/// Returns true if all coefficients are 0 or 1.
pub fn is_bin<const Q: i64, const N: usize>(poly: &Polynomial<ZqI64<Q>, N>) -> bool {
    poly.iter()
        .all(|c| c == &ZqI64::new(0) || c == &ZqI64::new(1))
}

/// Sample from the centered binomial distribution B_eta for integer eta > 0.
/// Returns a ZqI64<Q> sample.
pub fn sample_b_eta<const Q: i64, R: rand::RngExt + ?Sized>(eta: usize, rng: &mut R) -> ZqI64<Q> {
    let mut sum = 0i64;
    for _ in 0..eta {
        let a: i64 = rng.random_range(0..=1);
        let b: i64 = rng.random_range(0..=1);
        sum += a - b;
    }
    ZqI64::new(sum)
}

/// Sample a random polynomial in Zq[X]/(X^N+1)
pub(crate) fn sample_poly<const Q: i64, const N: usize, R: rand::RngExt + ?Sized>(
    rng: &mut R,
) -> Polynomial<ZqI64<Q>, N> {
    let coeffs = (0..N).map(|_| ZqI64::new(rng.random_range(0..Q))).collect();
    Polynomial::<ZqI64<Q>, N>::new(coeffs)
}

/// Sample a polynomial with coefficients from B_eta
pub(crate) fn sample_poly_b_eta<const Q: i64, const N: usize, R: rand::RngExt + ?Sized>(
    eta: usize,
    rng: &mut R,
) -> Polynomial<ZqI64<Q>, N> {
    let coeffs = (0..N).map(|_| sample_b_eta::<Q, R>(eta, rng)).collect();
    Polynomial::<ZqI64<Q>, N>::new(coeffs)
}

/// Sample a polynomial with coefficients within a range
pub(crate) fn sample_poly_range<const Q: i64, const N: usize, R: rand::RngExt + ?Sized>(
    low: i64,
    high: i64,
    rng: &mut R,
) -> Polynomial<ZqI64<Q>, N> {
    let coeffs = (0..N)
        .map(|_| ZqI64::new(rng.random_range(low..=high)))
        .collect();
    Polynomial::<ZqI64<Q>, N>::new(coeffs)
}

/// Multiplies each coefficient of the polynomial with the closest integer to q/2.
pub(crate) fn multiply_by_q_div_2<const Q: i64, const N: usize>(
    p: Polynomial<ZqI64<Q>, N>,
) -> Polynomial<ZqI64<Q>, N> {
    let q_div_2 = ZqI64::<Q>::new(Q / 2);
    let mut p = p;
    p.coeffs_mut(|c| *c = q_div_2.clone() * c.clone());
    p
}

/// Converts each coefficient of the polynomial to either 0 or 1 by checking whether it
/// is closer to 0 or q/2.
pub(crate) fn cloest_q_div_2_to_bin<const Q: i64, const N: usize>(
    p: Polynomial<ZqI64<Q>, N>,
) -> Polynomial<ZqI64<Q>, N> {
    let q_div_4 = ZqI64::<Q>::new(Q / 4);
    let mut p = p;
    p.coeffs_mut(|c| {
        let abs_c = if c.clone() > ZqI64::new(0) {
            c.clone()
        } else {
            ZqI64::new(0) - c.clone()
        };
        *c = if abs_c.gt(&q_div_4) {
            ZqI64::new(1)
        } else {
            ZqI64::new(0)
        };
    });
    p
}

#[cfg(test)]
mod tests {
    use super::*;
    use poly_ring_xnp1::{Polynomial, zqi64_vec};

    #[test]
    fn gadget_equation_holds() {
        // check the formula d = G^(b)_N * stack(ring_to_bin(d, b))
        const Q: i64 = 12289;
        const N: usize = 4;
        const B: usize = 4;
        // All coefficients in [-8, 7] (valid for 4-bit two's complement)
        let d = vec![
            Polynomial::<ZqI64<Q>, N>::new(zqi64_vec![3, -1, 0, 5; Q]),
            Polynomial::<ZqI64<Q>, N>::new(zqi64_vec![0, 1, -7, 2; Q]),
            Polynomial::<ZqI64<Q>, N>::new(zqi64_vec![-1, 4, -3, 5; Q]),
            Polynomial::<ZqI64<Q>, N>::new(zqi64_vec![6, -4, 2, -5; Q]),
        ];
        let bin_polys = stacked_ring_to_bin::<Q, N>(&d, B);
        let g = gadget_matrix::<N, Q>(B);
        // compute matrix-vector product G^(b)_N * stack(ring_to_bin(d, b))
        let mut g_stack = Vec::with_capacity(N);
        for i in 0..N {
            let mut sum = Polynomial::<ZqI64<Q>, N>::zero();
            for k in 0..B {
                let j = k * N + i;
                sum = sum + g[i][j].clone() * bin_polys[j].clone();
            }
            g_stack.push(sum);
        }
        assert_eq!(g_stack.len(), d.len());
        for (a, b) in g_stack.iter().zip(d.iter()) {
            assert_eq!(a, b);
        }
    }

    #[test]
    fn gadget_matrix_works() {
        const Q: i64 = 12289;
        const N: usize = 4;
        const B: usize = 4;
        let g = gadget_matrix::<N, Q>(B);
        assert_eq!(g.len(), N);
        for row in g.clone() {
            assert_eq!(row.len(), N * B);
        }
        // Check elements of g
        assert_eq!(
            g[0][0],
            Polynomial::<ZqI64<Q>, N>::new(zqi64_vec![1, 0, 0, 0; Q])
        );
        assert_eq!(
            g[0][4],
            Polynomial::<ZqI64<Q>, N>::new(zqi64_vec![2, 0, 0, 0; Q])
        );
        assert_eq!(
            g[0][8],
            Polynomial::<ZqI64<Q>, N>::new(zqi64_vec![4, 0, 0, 0; Q])
        );
        assert_eq!(
            g[0][12],
            Polynomial::<ZqI64<Q>, N>::new(zqi64_vec![-8, 0, 0, 0; Q])
        );
        assert_eq!(
            g[1][1],
            Polynomial::<ZqI64<Q>, N>::new(zqi64_vec![1, 0, 0, 0; Q])
        );
        assert_eq!(
            g[1][5],
            Polynomial::<ZqI64<Q>, N>::new(zqi64_vec![2, 0, 0, 0; Q])
        );
    }

    #[test]
    fn binomial_matrix_works() {
        const Q: i64 = 3109;
        const N: usize = 512;
        const ETA: usize = 2;
        let l: usize = 2;
        let b = binomial_matrix::<N, Q>(l, ETA);
        assert_eq!(b.len(), l);
        for row in b.clone() {
            assert_eq!(row.len(), 2 * ETA);
        }
        // Check elements of b
        assert_eq!(
            b[0][0],
            Polynomial::<ZqI64<Q>, N>::new(zqi64_vec![1, 0, 0, 0; Q])
        );
        assert_eq!(
            b[0][2],
            Polynomial::<ZqI64<Q>, N>::new(zqi64_vec![-1, 0, 0, 0; Q])
        );
    }

    #[test]
    fn stacked_ring_to_bin_works() {
        const Q: i64 = 12289;
        const N: usize = 4;
        let polys = vec![
            Polynomial::<ZqI64<Q>, N>::new(zqi64_vec![1, 2, 0, 0; Q]),
            Polynomial::<ZqI64<Q>, N>::new(zqi64_vec![3, -4, 0, 0; Q]),
        ];
        let bin_polys = stacked_ring_to_bin::<Q, N>(&polys, 3);
        assert_eq!(bin_polys.len(), polys.len() * 3);
        // Check the first bit of the coefficients
        assert_eq!(
            bin_polys[0],
            Polynomial::<ZqI64<Q>, N>::new(zqi64_vec![1, 0, 0, 0; Q])
        );
        assert_eq!(
            bin_polys[1],
            Polynomial::<ZqI64<Q>, N>::new(zqi64_vec![1, 0, 0, 0; Q])
        );
    }

    #[test]
    fn int_to_bin_works() {
        const Q: i64 = 12289;
        const N: usize = 4;
        let poly = int_to_bin::<N, Q>(0b1011, 4);
        let expected = vec![1, 1, 0, 1];
        for (a, &b) in poly.iter().zip(expected.iter()) {
            assert_eq!(a, &ZqI64::new(b));
        }
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
        let mut rng = rand::rng();
        const Q: i64 = 12289;
        let eta = 8;
        let samples: Vec<_> = (0..10000)
            .map(|_| sample_b_eta::<Q, _>(eta, &mut rng))
            .collect();
        let mean: f64 = samples
            .iter()
            .map(|x| i64::from(x.clone()) as f64)
            .sum::<f64>()
            / samples.len() as f64;
        // For large enough samples, mean should be close to 0
        assert!(mean.abs() < 0.2, "mean was {}", mean);
    }
}
