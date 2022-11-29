use std::{
    borrow::{Borrow, BorrowMut},
    convert::{AsMut, AsRef},
    fmt::Debug,
    ops::{Add, Div, Index, IndexMut, Mul, Sub},
};

use algebraic_gen::generate_geometric_product;
generate_geometric_product!(product, 1);

use super::MultiVector;

pub struct MultiVector1<T>(pub [T; 2]);

impl<T> IntoIterator for MultiVector1<T> {
    type Item = T;
    type IntoIter = <[T; 2] as IntoIterator>::IntoIter;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<T> Index<usize> for MultiVector1<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<T> IndexMut<usize> for MultiVector1<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl<T> AsRef<[T]> for MultiVector1<T> {
    fn as_ref(&self) -> &[T] {
        self.0.as_ref()
    }
}

impl<T> AsMut<[T]> for MultiVector1<T> {
    fn as_mut(&mut self) -> &mut [T] {
        self.0.as_mut()
    }
}

impl<T> Borrow<[T]> for MultiVector1<T> {
    fn borrow(&self) -> &[T] {
        self.0.borrow()
    }
}

impl<T> BorrowMut<[T]> for MultiVector1<T> {
    fn borrow_mut(&mut self) -> &mut [T] {
        self.0.borrow_mut()
    }
}

impl<T: Copy + Add<T, Output = T>> Add for MultiVector1<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self([self[0] + rhs[0], self[1] + rhs[1]])
    }
}

impl<T: Copy + Sub<T, Output = T>> Sub for MultiVector1<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self([self[0] - rhs[0], self[1] - rhs[1]])
    }
}

impl<T: Copy + Add<T, Output = T> + Sub<T, Output = T> + Mul<T, Output = T>> Mul<Self>
    for MultiVector1<T>
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(product(&self, &rhs))
    }
}

impl<T: Copy + Mul<T, Output = T>> Mul<T> for MultiVector1<T> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self([self[0] * rhs, self[1] * rhs])
    }
}

impl<T: Copy + Div<T, Output = T>> Div<T> for MultiVector1<T> {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Self([self[0] / rhs, self[1] / rhs])
    }
}

impl<T: Copy> Copy for MultiVector1<T> {}

impl<T: Clone> Clone for MultiVector1<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T: Default> Default for MultiVector1<T> {
    fn default() -> Self {
        Self(<[T; 2]>::default())
    }
}

impl<T: Debug> Debug for MultiVector1<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl<
        T: Copy
            + Default
            + Debug
            + Add<T, Output = T>
            + Sub<T, Output = T>
            + Mul<T, Output = T>
            + Div<T, Output = T>,
    > MultiVector<T> for MultiVector1<T>
{
    const ALGEBRA_DIMENSION: usize = 1;
}
