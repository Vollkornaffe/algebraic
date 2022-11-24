//! Uses [algebraic-gen](https://crates.io/crates/algebraic-gen) to generate geometric products for
//! up to 6 dimensions of space.
//!
//! The products are free functions with (pseudo) signature
//!
//! ```fn geometric_product_D<T>(a: &[T; 2.pow(D)], b: &[T; 2.pow(D)]) -> [T; 2.pow(D)];```
//!
//! Where ```D``` is the respective dimension and ```T``` can be any floating type.
//!
//! # Example
//!
//! The functions are straightforward to use:
//!
//! ```
//! use algebraic::geometric_product_3;
//!
//! fn main() {
//!   let a = [1., 2., 3., 4., 5., 6., 7., 8.];
//!   let b = [8., 7., 6., 5., 4., 3., 2., 1.];
//!
//!   let c = geometric_product_3::<f32>(&a, &b);
//!
//!   println!("The geometric product of {:?} and {:?} is {:?}", a, b, c);
//! }
//! ```
//!
//! # Geometric Algebra
//!
//! A geometric algebra is always defined wrt. a given dimension ```D``` of space. In such an
//! algebra there exist different objects. For one there are a ```D``` orthogonal unit vectors
//! (X,Y,Z in 3D). But there are also other, more exotic objects like *Bivectors*.
//! However no matter how different, any number and all types of objects within a geometric
//! algebra can be scaled and added to form a *Multivector* which is then also the most general
//! type of object.
//!
//! And, critically for this crate, any Multivector can be represented uniquely as a linear
//! combination of elements of a chosen base with ```2^D``` different objects.
//! Meaning we can represent Multivectors as arrays (of floating point numbers) of that size.
//!
//! The types ```[T; 1]```, ```[T; 2]```, ```[T; 4]```, ```[T; 8]```, etc. are exactly those
//! representations of Multivectors of geometric algebras of spaces with 0, 1, 2, 3, etc.
//! dimensions, respectively.
//!
//! # Choice of Base
//!
//! Sticking to 3D for example, this is the choice of base in this crate:
//!
//! 1 Scalar (```S```), 3 Vectors (```X```, ```Y```, ```Z```), 3 Bivectors (```X∧Y```, ```X∧Z```, ```Y∧X```), and 1 Pseudoscalar (```X∧Y∧Z```)
//!
//! In terms of coefficients in the ```geometric_product_3```
//!
//! ```[S, X, Y, X∧Y, Z, X∧Z, Y∧Z, X∧Y∧Z]```
//!
//! or, as in the documentation attached to the respective product
//!
//! ```[[], [0], [1], [0,1], [2], [0,2], [1,2], [0,1,2]]```
//!
//! More general, the rules for generation/ordering are
//! * Start with scalar and generate the other objects peu à peu
//! * Add another unit vector if nothing new can be generated
//! * Flip order of unit vectors in product until sorted
//!
//! # Even Higher Dimensions
//!
//! You can easily use [algebraic-gen](https://crates.io/crates/algebraic-gen) directly to go as
//! high as you need. Be aware of the exponential increase of size, though.
//!
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
