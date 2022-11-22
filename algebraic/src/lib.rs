use algebraic_gen::generate_geometric_product;
use std::ops::{Add, Mul, Sub};

generate_geometric_product!(0);
generate_geometric_product!(1);
generate_geometric_product!(2);
generate_geometric_product!(3);
generate_geometric_product!(4);
generate_geometric_product!(5);
generate_geometric_product!(6);

mod trait_for_arrays;

#[cfg(test)]
#[generic_tests::define]
mod tests {
    use super::*;
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

    const SAMPLES: usize = 100;
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

    // ab == (ab + ba) / 2 + (ab - ba) / 2
    #[test]
    fn antisymmetry<T: GeometricProduct<f64>>() {
        let samples: Vec<T> = random_samples(&mut setup_rng());
        for i in 0..SAMPLES / 2 {
            let a = &samples[i];
            let b = &samples[i + SAMPLES / 2];
            let ab = T::geometric_product(a, b);
            let ba = T::geometric_product(b, a);

            assert!(approx(
                ab,
                add(scale(add(ab, ba), 0.5), scale(sub(ab, ba), 0.5),)
            ));
        }
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
