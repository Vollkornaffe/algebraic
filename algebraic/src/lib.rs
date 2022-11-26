//! Uses [algebraic_gen](https://crates.io/crates/algebraic-gen) to generate performant
//! geometric products for up to 6 dimensions of space.
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
//! <https://en.wikipedia.org/wiki/Geometric_algebra>
//!
//! A geometric algebra is always defined wrt. a given dimension ```D``` of space. In such an
//! algebra there exist different objects. For one there are a ```D``` orthogonal unit vectors
//! (X,Y,Z in 3D). But there are also other, more exotic objects like *Bivectors*.
//! However, no matter how exotic, any number and all types of objects within a geometric
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
//! The documentation of the choice of base is attached to the respective product.
//! For details regarding the generation, please have a look at the generating crate.
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
//! # Even Higher Dimensions
//!
//! You can easily use [algebraic-gen](https://crates.io/crates/algebraic-gen) directly to go as
//! high as you need. Be aware of the exponential increase of size, though.
//!

pub mod multi_vector;
pub use multi_vector::MultiVector;
pub use multi_vector::MultiVector0;
pub use multi_vector::MultiVector1;
pub use multi_vector::MultiVector2;
pub use multi_vector::MultiVector3;
pub use multi_vector::MultiVector4;
pub use multi_vector::MultiVector5;
pub use multi_vector::MultiVector6;

#[cfg(test)]
mod unit_tests;
