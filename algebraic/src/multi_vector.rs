//! TODO
use algebraic_gen::generate_geometric_product;
use std::{
    borrow::{Borrow, BorrowMut},
    convert::{AsMut, AsRef},
    fmt::Debug,
    ops::{Add, Div, Index, IndexMut, Mul, Sub},
};

/// Implemented by the multi vectors of the geometric algebras of different
/// dimensions.
///
/// The [`Add`], [`Sub`], [`Mul<T>`], and [`Div<T>`] implementations are
/// component-wise. `Mul<Self>` (with another MultiVector) is the geometric
/// product.
///
/// All implementations are just light wrappers around arrays of size
/// `NUMBER_OF_OBJECTS` aka. `2^ALGEBRA_DIMENSION`.
pub trait MultiVector<T>:
    Copy
    + Default
    + Debug
    + IntoIterator<Item = T>
    + IndexMut<usize, Output = T>
    + AsRef<[T]>
    + AsMut<[T]>
    + BorrowMut<[T]>
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Mul<T, Output = Self>
    + Div<T, Output = Self>
    + private::Sealed
{
    /// FIXME: Document.
    const ALGEBRA_DIMENSION: usize;

    /// The size of the base: `2^ALGEBRA_DIMENSION`
    const BASE_SIZE: usize = 1 << Self::ALGEBRA_DIMENSION;
}

macro_rules! generate_multivector_boilerplate {
    (
        $(#[$outer:meta])*
        $name:ident,
        $product:ident,
        $dimension:literal,
        $($idx:literal),+
    ) => {

        $(#[$outer])*
        pub struct $name<T>(pub [T; 1 << $dimension]);

        impl<T> IntoIterator for $name<T> {
            type Item = T;
            type IntoIter = <[T; 1 << $dimension] as IntoIterator>::IntoIter;
            fn into_iter(self) -> Self::IntoIter {
                self.0.into_iter()
            }
        }

        impl<T> Index<usize> for $name<T> {
            type Output = T;
            fn index(&self, index: usize) -> &Self::Output {
                &self.0[index]
            }
        }

        impl<T> IndexMut<usize> for $name<T> {
            fn index_mut(&mut self, index: usize) -> &mut Self::Output {
                &mut self.0[index]
            }
        }

        impl<T> AsRef<[T]> for $name<T> {
            fn as_ref(&self) -> &[T] {
                self.0.as_ref()
            }
        }

        impl<T> AsMut<[T]> for $name<T> {
            fn as_mut(&mut self) -> &mut [T] {
                self.0.as_mut()
            }
        }

        impl<T> Borrow<[T]> for $name<T> {
            fn borrow(&self) -> &[T] {
                self.0.borrow()
            }
        }

        impl<T> BorrowMut<[T]> for $name<T> {
            fn borrow_mut(&mut self) -> &mut [T] {
                self.0.borrow_mut()
            }
        }

        impl<T: Copy> Copy for $name<T> {}

        impl<T: Clone> Clone for $name<T> {
            fn clone(&self) -> Self {
                Self(self.0.clone())
            }
        }

        impl<T: Debug> Debug for $name<T> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.0.fmt(f)
            }
        }

        /// Arrays implement Default only up to size 32.
        /// So, this is using the inner Default.
        impl<T: Copy + Default> Default for $name<T> {
            fn default() -> Self {
                Self([T::default(); 1 << $dimension])
            }
        }

        /// Per coefficient.
        impl<T: Copy + Add<T, Output = T>> Add for $name<T> {
            type Output = Self;

            fn add(self, rhs: Self) -> Self::Output {
                Self([$(self[$idx] + rhs[$idx],)*])
            }
        }

        /// Per coefficient.
        impl<T: Copy + Sub<T, Output = T>> Sub for $name<T> {
            type Output = Self;

            fn sub(self, rhs: Self) -> Self::Output {
                Self([$(self[$idx] - rhs[$idx],)*])
            }
        }

        /// Per ceofficient.
        impl<T: Copy + Mul<T, Output = T>> Mul<T> for $name<T> {
            type Output = Self;

            fn mul(self, rhs: T) -> Self::Output {
                Self([$(self[$idx] * rhs,)*])
            }
        }

        /// Per ceofficient.
        impl<T: Copy + Div<T, Output = T>> Div<T> for $name<T> {
            type Output = Self;

            fn div(self, rhs: T) -> Self::Output {
                Self([$(self[$idx] / rhs,)*])
            }
        }

        generate_geometric_product!($product, $dimension);

        /// This is implemented via [generate_geometric_product].
        impl<T: Copy + Add<T, Output = T> + Sub<T, Output = T> + Mul<T, Output = T>> Mul<Self>
            for $name<T>
        {
            type Output = Self;

            fn mul(self, rhs: Self) -> Self::Output {
                Self($product(&self, &rhs))
            }
        }

        /// Apart from the [MultiVector::ALGEBRA_DIMENSION] there is nothing to implement directly.
        impl<
                T: Copy
                    + Default
                    + Debug
                    + Add<T, Output = T>
                    + Sub<T, Output = T>
                    + Mul<T, Output = T>
                    + Div<T, Output = T>,
            > MultiVector<T> for $name<T>
        {
            const ALGEBRA_DIMENSION: usize = $dimension;
        }
    };
}

generate_multivector_boilerplate!(
    /// Element of the geometric algebra of dimension 0.
    /// Contains only scalars.
    MultiVector0,
    geometric_product_0,
    0,
    0
);

generate_multivector_boilerplate!(
    /// Element of the geometric algebra of dimension 1.
    /// Contains scalars and vectors (with one dimension).
    MultiVector1,
    geometric_product_1,
    1,
    0,
    1
);

generate_multivector_boilerplate!(
    /// Element of the geometric algebra of dimension 2.
    /// Contains scalars, vectors, and bivectors which are also pseudoscalars.
    MultiVector2,
    geometric_product_2,
    2,
    0,
    1,
    2,
    3
);

generate_multivector_boilerplate!(
    /// Element of the geometric algebra of dimension 3.
    /// Contains scalars, vectors, bivectors, and trivectors which are also pseudoscalars.
    MultiVector3,
    geometric_product_3,
    3,
    0,
    1,
    2,
    3,
    4,
    5,
    6,
    7
);

generate_multivector_boilerplate!(
    /// Element of the geometric algebra of dimension 4.
    /// Contains scalars, vectors, bivectors, trivectors, and quadvectors which are also pseudoscalars.
    MultiVector4,
    geometric_product_4,
    4,
    0,
    1,
    2,
    3,
    4,
    5,
    6,
    7,
    8,
    9,
    10,
    11,
    12,
    13,
    14,
    15
);

generate_multivector_boilerplate!(
    /// Element of the geometric algebra of dimension 5.
    /// Contains scalars, vectors, bivectors, trivectors, quadvectors, and pentavectors which are also pseudoscalars.
    MultiVector5,
    geometric_product_5,
    5,
    0,
    1,
    2,
    3,
    4,
    5,
    6,
    7,
    8,
    9,
    10,
    11,
    12,
    13,
    14,
    15,
    16,
    17,
    18,
    19,
    20,
    21,
    22,
    23,
    24,
    25,
    26,
    27,
    28,
    29,
    30,
    31
);

generate_multivector_boilerplate!(
    /// Element of the geometric algebra of dimension 6.
    /// Contains scalars, vectors, bivectors, trivectors, quadvectors, pentavectors, and hexvectors which are also pseudoscalars.
    MultiVector6,
    geometric_product_6,
    6,
    0,
    1,
    2,
    3,
    4,
    5,
    6,
    7,
    8,
    9,
    10,
    11,
    12,
    13,
    14,
    15,
    16,
    17,
    18,
    19,
    20,
    21,
    22,
    23,
    24,
    25,
    26,
    27,
    28,
    29,
    30,
    31,
    32,
    33,
    34,
    35,
    36,
    37,
    38,
    39,
    40,
    41,
    42,
    43,
    44,
    45,
    46,
    47,
    48,
    49,
    50,
    51,
    52,
    53,
    54,
    55,
    56,
    57,
    58,
    59,
    60,
    61,
    62,
    63
);

mod private {
    pub trait Sealed {}
    impl<T> Sealed for super::MultiVector0<T> {}
    impl<T> Sealed for super::MultiVector1<T> {}
    impl<T> Sealed for super::MultiVector2<T> {}
    impl<T> Sealed for super::MultiVector3<T> {}
    impl<T> Sealed for super::MultiVector4<T> {}
    impl<T> Sealed for super::MultiVector5<T> {}
    impl<T> Sealed for super::MultiVector6<T> {}
}
