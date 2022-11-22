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

    #[test]
    fn antisymmetry<const N: usize>() {
        println!("Testing for N = {}", N)
    }

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
