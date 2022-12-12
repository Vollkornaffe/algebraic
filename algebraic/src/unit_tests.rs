use crate::multi_vector::MultiVector;

use super::*;

use itertools::Itertools;
use rand_chacha::{rand_core::SeedableRng, ChaCha8Rng};
use rand_core::RngCore;

fn setup_rng() -> ChaCha8Rng {
    rand_chacha::ChaCha8Rng::seed_from_u64(42)
}

fn random_float(rng: &mut ChaCha8Rng) -> f64 {
    rng.next_u64() as f64 / std::u64::MAX as f64 * 20.0 - 10.0
}

fn random_vector<T: MultiVector<f64>>(rng: &mut ChaCha8Rng) -> T {
    let mut v = T::default();
    for i in (0..T::ALGEBRA_DIMENSION).map(|d| 1 << d) {
        v[i] = random_float(rng);
    }
    v
}

fn random_multi_vector<T: MultiVector<f64>>(rng: &mut ChaCha8Rng) -> T {
    let mut mv = T::default();
    for i in 0..T::BASE_SIZE {
        mv[i] = random_float(rng);
    }
    mv
}

const SAMPLES: usize = 50;
fn random_samples<T: MultiVector<f64>>(rng: &mut ChaCha8Rng) -> Vec<T> {
    (0..SAMPLES).map(|_| random_multi_vector(rng)).collect()
}

fn approx<T: MultiVector<f64>>(a: T, b: T) -> bool {
    a.into_iter()
        .zip(b.into_iter())
        .all(|(a, b)| (a - b).abs() < 0.0001)
}

fn permutation_even(permutation: &[usize]) -> bool {
    permutation
        .iter()
        .enumerate()
        .combinations(2)
        .filter(|combination| {
            let (i, d_i) = combination[0];
            let (j, d_j) = combination[1];
            (d_i < d_j && i > j) || (d_i > d_j && i < j)
        })
        .count()
        % 2
        == 0
}

fn factorial(n: usize) -> usize {
    (1..=n).fold(1, |f, i| f * i)
}

#[generic_tests::define]
mod tests {
    use super::*;

    // ab == (ab + ba) / 2 + (ab - ba) / 2
    #[test]
    fn antisymmetry<T: MultiVector<f64>>() {
        let samples: Vec<T> = random_samples(&mut setup_rng());
        for &a in &samples {
            for &b in &samples {
                let ab = a * b;
                let ba = b * a;

                assert!(approx(ab, (ab + ba) * 0.5 + (ab - ba) * 0.5));
            }
        }
    }

    // (ab)c == a(bc)
    #[test]
    fn associativity<T: MultiVector<f64>>() {
        let samples: Vec<T> = random_samples(&mut setup_rng());
        for &a in &samples {
            for &b in &samples {
                for &c in &samples {
                    assert!(approx((a * b) * c, a * (b * c)));
                }
            }
        }
    }

    // a(b + c) == ab + ac
    #[test]
    fn distributivity<T: MultiVector<f64>>() {
        let samples: Vec<T> = random_samples(&mut setup_rng());
        for &a in &samples {
            for &b in &samples {
                for &c in &samples {
                    assert!(approx(a * (b + c), a * b + a * c,));
                }
            }
        }
    }

    // aa == scalar, where a is a vector
    #[test]
    fn vector_square_scalar<T: MultiVector<f64>>() {
        let mut rng = setup_rng();
        for _ in 0..SAMPLES {
            let a: T = random_vector(&mut rng);
            let mut aa = a * a;
            aa[0] = 0.0;

            assert!(approx(aa, T::default()));
        }
    }

    // a == ab b/bb, where b is a vector
    #[test]
    fn vector_inverse<T: MultiVector<f64>>() {
        let mut rng = setup_rng();
        for _ in 0..SAMPLES {
            let a: T = random_multi_vector(&mut rng);
            let b: T = random_vector(&mut rng);

            let bb = (b * b)[0];
            if bb == 0.0 {
                continue;
            }

            assert!(approx(a, a * b * b / bb));
        }
    }

    // pseudoscalar = 1/d! sum (-1 if perm is odd) (unit vector permutations)
    #[test]
    fn permutations_pseudoscalar<T: MultiVector<f64>>() {
        let pseudoscalar = (0..T::ALGEBRA_DIMENSION)
            .permutations(T::ALGEBRA_DIMENSION)
            .map(|permutation| {
                (
                    permutation_even(&permutation),
                    permutation.iter().fold(
                        {
                            // hacky unit scalar
                            let mut s = T::default();
                            s[0] = 1.0;
                            s
                        },
                        |prod, d| {
                            prod * {
                                // hacky unit vector
                                let mut u = T::default();
                                u[1 << d] = 1.0;
                                u
                            }
                        },
                    ),
                )
            })
            .fold(
                T::default(),
                |sum, (even, prod)| {
                    if even {
                        sum + prod
                    } else {
                        sum - prod
                    }
                },
            )
            / factorial(T::ALGEBRA_DIMENSION) as f64;

        let mut reference = T::default();
        reference[T::BASE_SIZE - 1] = 1.0;

        assert!(approx(pseudoscalar, reference));
    }

    #[instantiate_tests(<MultiVector0<f64>>)]
    mod d0 {}
    #[instantiate_tests(<MultiVector1<f64>>)]
    mod d1 {}
    #[instantiate_tests(<MultiVector2<f64>>)]
    mod d2 {}
    #[instantiate_tests(<MultiVector3<f64>>)]
    mod d3 {}
    #[instantiate_tests(<MultiVector4<f64>>)]
    mod d4 {}
    #[instantiate_tests(<MultiVector5<f64>>)]
    mod d5 {}
    #[instantiate_tests(<MultiVector6<f64>>)]
    mod d6 {}
}
