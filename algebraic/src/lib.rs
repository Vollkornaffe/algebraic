use algebraic_gen::generate_geometric_product;
use std::ops::{Add, Mul, Sub};

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
    use rand_chacha::{rand_core::SeedableRng, ChaCha8Rng};
    use rand_core::RngCore;

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

    fn random_vector<const N: usize>(rng: &mut ChaCha8Rng) -> [f64; N] {
        let mut v = [0.0; N];
        for i in (0..log_2(N)).map(|d| 1 << d) {
            v[i] = random_float(rng);
        }
        v
    }

    fn random_multi_vector<const N: usize>(rng: &mut ChaCha8Rng) -> [f64; N] {
        let mut mv = [0.0; N];
        for i in 0..N {
            mv[i] = random_float(rng);
        }
        mv
    }

    fn scale<const N: usize>(a: &[f64; N], s: f64) -> [f64; N] {
        let mut b = a.clone();
        for i in 0..N {
            b[i] *= s;
        }
        b
    }

    fn add<const N: usize>(a: &[f64; N], b: &[f64; N]) -> [f64; N] {
        let mut c = a.clone();
        for i in 0..N {
            c[i] += b[i];
        }
        c
    }

    fn sub<const N: usize>(a: &[f64; N], b: &[f64; N]) -> [f64; N] {
        let mut c = a.clone();
        for i in 0..N {
            c[i] -= b[i];
        }
        c
    }

    fn approx<const N: usize>(a: &[f64; N], b: &[f64; N]) -> bool {
        a.iter().zip(b.iter()).all(|(a, b)| (a - b).abs() < 0.0001)
    }

    #[test]
    fn antisymmetry<const N: usize>() {}

    #[instantiate_tests(<1>)]
    mod d0 {}
    #[instantiate_tests(<2>)]
    mod d1 {}
    #[instantiate_tests(<4>)]
    mod d2 {}
    #[instantiate_tests(<8>)]
    mod d3 {}
    #[instantiate_tests(<16>)]
    mod d4 {}
    #[instantiate_tests(<32>)]
    mod d5 {}
    #[instantiate_tests(<64>)]
    mod d6 {}
}
