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
mod unit_tests;
