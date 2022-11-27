pub mod algebra_dimension_0;
pub use algebra_dimension_0::MultiVector0;

pub mod algebra_dimension_1;
pub use algebra_dimension_1::MultiVector1;

pub mod algebra_dimension_2;
pub use algebra_dimension_2::MultiVector2;

pub mod algebra_dimension_3;
pub use algebra_dimension_3::MultiVector3;

pub mod algebra_dimension_4;
pub use algebra_dimension_4::MultiVector4;

pub mod algebra_dimension_5;
pub use algebra_dimension_5::MultiVector5;

pub mod algebra_dimension_6;
pub use algebra_dimension_6::MultiVector6;

use std::{
    borrow::BorrowMut,
    convert::{AsMut, AsRef},
    fmt::Debug,
    ops::{Add, Div, IndexMut, Mul, Sub},
};

/// Implemented by the multi vectors of the geometric algebras of different dimensions.
///
/// The `Add`, `Sub`, `Mul<T>`, and `Div<T>` implementations are component-wise.
/// `Mul<Self>` (with another MultiVector) is the geometric product.
///
/// All implementations are just light wrappers around arrays of size `NUMBER_OF_OBJECTS` aka.
/// `2^ALGEBRA_DIMENSION`.
pub trait MultiVector<T>:
    Copy
    + Default
    + Debug
    + IntoIterator<Item = T>
    + IndexMut<usize, Output = T>
    + AsRef<[T]>
    + AsMut<[T]>
    + BorrowMut<[T]>
    + Add<Self, Output = Self>
    + Sub<Self, Output = Self>
    + Mul<Self, Output = Self>
    + Mul<T, Output = Self>
    + Div<T, Output = Self>
{
    const ALGEBRA_DIMENSION: usize;
    const NUMBER_OF_OBJECTS: usize = 1 << Self::ALGEBRA_DIMENSION;
}
