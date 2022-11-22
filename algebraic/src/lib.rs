use algebraic_gen::generate_geometric_product;
use std::ops::{Add, Mul, Sub};
pub mod trait_for_arrays;

generate_geometric_product!(0);
generate_geometric_product!(1);
generate_geometric_product!(2);
generate_geometric_product!(3);
generate_geometric_product!(4);
generate_geometric_product!(5);
generate_geometric_product!(6);

#[cfg(test)]
#[generic_tests::define]
mod tests {
    use super::*;
    use itertools::Itertools;
    use rand_chacha::{rand_core::SeedableRng, ChaCha8Rng};
    use rand_core::RngCore;
    use trait_for_arrays::GeometricProduct;

    fn log_2(n: usize) -> usize {
        assert!(n.is_power_of_two());
        n.trailing_zeros() as usize
    }

    fn setup_rng() -> ChaCha8Rng {
        rand_chacha::ChaCha8Rng::seed_from_u64(42)
    }

    fn random_float(rng: &mut ChaCha8Rng) -> f64 {
        rng.next_u64() as f64 / std::u64::MAX as f64 * 20.0 - 10.0
    }

    fn random_vector<T: GeometricProduct<f64>>(rng: &mut ChaCha8Rng) -> T {
        let mut v = T::default();
        for i in (0..log_2(T::N)).map(|d| 1 << d) {
            v[i] = random_float(rng);
        }
        v
    }

    fn random_multi_vector<T: GeometricProduct<f64>>(rng: &mut ChaCha8Rng) -> T {
        let mut mv = T::default();
        for i in 0..T::N {
            mv[i] = random_float(rng);
        }
        mv
    }

    const SAMPLES: usize = 50;
    fn random_samples<T: GeometricProduct<f64>>(rng: &mut ChaCha8Rng) -> Vec<T> {
        (0..SAMPLES).map(|_| random_multi_vector(rng)).collect()
    }

    fn scale<T: GeometricProduct<f64>>(a: T, s: f64) -> T {
        let mut b = a.clone();
        for i in 0..T::N {
            b[i] *= s;
        }
        b
    }

    fn add<T: GeometricProduct<f64>>(a: T, b: T) -> T {
        let mut c = a.clone();
        for i in 0..T::N {
            c[i] += b[i];
        }
        c
    }

    fn sub<T: GeometricProduct<f64>>(a: T, b: T) -> T {
        let mut c = a.clone();
        for i in 0..T::N {
            c[i] -= b[i];
        }
        c
    }

    fn approx<T: GeometricProduct<f64>>(a: T, b: T) -> bool {
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

    // ab == (ab + ba) / 2 + (ab - ba) / 2
    #[test]
    fn antisymmetry<T: GeometricProduct<f64>>() {
        let samples: Vec<T> = random_samples(&mut setup_rng());
        for a in &samples {
            for b in &samples {
                let ab = T::geometric_product(a, b);
                let ba = T::geometric_product(b, a);

                assert!(approx(
                    ab,
                    add(scale(add(ab, ba), 0.5), scale(sub(ab, ba), 0.5),)
                ));
            }
        }
    }

    // (ab)c == a(bc)
    #[test]
    fn associativity<T: GeometricProduct<f64>>() {
        let samples: Vec<T> = random_samples(&mut setup_rng());
        for a in &samples {
            for b in &samples {
                for c in &samples {
                    let ab = T::geometric_product(a, b);
                    let ab_c = T::geometric_product(&ab, c);
                    let bc = T::geometric_product(b, c);
                    let a_bc = T::geometric_product(a, &bc);
                    assert!(approx(ab_c, a_bc));
                }
            }
        }
    }

    // a(b + c) == ab + ac
    #[test]
    fn distributivity<T: GeometricProduct<f64>>() {
        let samples: Vec<T> = random_samples(&mut setup_rng());
        for a in &samples {
            for b in &samples {
                for c in &samples {
                    assert!(approx(
                        T::geometric_product(a, &add(*b, *c)),
                        add(T::geometric_product(a, b), T::geometric_product(a, c),),
                    ));
                }
            }
        }
    }

    // aa == scalar, where a is a vector
    #[test]
    fn vector_square_scalar<T: GeometricProduct<f64>>() {
        let mut rng = setup_rng();
        for _ in 0..SAMPLES {
            let a: T = random_vector(&mut rng);
            let mut aa = T::geometric_product(&a, &a);
            aa[0] = 0.0;

            assert!(approx(aa, T::default()));
        }
    }

    // a == ab b/bb, where b is a vector
    #[test]
    fn vector_inverse<T: GeometricProduct<f64>>() {
        let mut rng = setup_rng();
        for _ in 0..SAMPLES {
            let a: T = random_multi_vector(&mut rng);
            let b: T = random_vector(&mut rng);

            let ab = T::geometric_product(&a, &b);

            let bb = T::geometric_product(&b, &b)[0];
            if bb == 0.0 {
                continue;
            }

            let b_bb = scale(b, 1.0 / bb);

            assert!(approx(a, T::geometric_product(&ab, &b_bb)));
        }
    }

    // pseudoscalar = 1/d! sum (-1 if perm is odd) (unit vector permutations)
    #[test]
    fn permutations_pseudoscalar<T: GeometricProduct<f64>>() {
        let pseudoscalar = scale(
            (0..T::D)
                .permutations(T::D)
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
                                T::geometric_product(&prod, &{
                                    // hacky unit vector
                                    let mut u = T::default();
                                    u[1 << d] = 1.0;
                                    u
                                })
                            },
                        ),
                    )
                })
                .fold(T::default(), |sum, (even, prod)| {
                    if even {
                        add(sum, prod)
                    } else {
                        sub(sum, prod)
                    }
                }),
            1.0 / factorial(T::D) as f64,
        );

        println!("{:?}", pseudoscalar);

        let mut reference = T::default();
        reference[T::N - 1] = 1.0;

        assert!(approx(pseudoscalar, reference));
    }

    #[instantiate_tests(<[f64;1]>)]
    mod d0 {}
    #[instantiate_tests(<[f64;2]>)]
    mod d1 {}
    #[instantiate_tests(<[f64;4]>)]
    mod d2 {}
    #[instantiate_tests(<[f64;8]>)]
    mod d3 {}
    #[instantiate_tests(<[f64;16]>)]
    mod d4 {}
    #[instantiate_tests(<[f64;32]>)]
    mod d5 {}
}
