//! Generalizes multi vectors of geometric algebras of different dimensions which are implemented
//! using [algebra-gen](https://docs.rs/algebraic-gen/latest/algebraic_gen).
//!
//! This crate provides the [MultiVector] trait with 7 implementations, i.e. for algebra dimensions
//! from Zero to Six.
//! This enables writing code that is generic in the algebra dimension.
//! The implementations provide algebraic structures that overload operators `+`, `-`, and `*`. The code for multiplying is generated using the
//! [generate_geometric_product](https://docs.rs/algebraic-gen/latest/algebraic_gen/macro.generate_geometric_product.html) macro.
//!
//! In a certain sense, this crate only exists because proc-macro crates cannot export anything but
//! proc-macros. [algebra-gen](https://docs.rs/crate/algebraic-gen/latest) doesn't really have any
//! unit-testing to speak of, testing happens here.
//!
//! # Example & More
//!
//! TODO
//!
pub mod multi_vector;
pub use multi_vector::{
    MultiVector, MultiVector0, MultiVector1, MultiVector2, MultiVector3, MultiVector4,
    MultiVector5, MultiVector6,
};

#[cfg(test)]
mod unit_tests;
