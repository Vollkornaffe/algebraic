use std::{
    borrow::{Borrow, BorrowMut},
    convert::{AsMut, AsRef},
    fmt::Debug,
    ops::{Add, Div, Index, IndexMut, Mul, Sub},
};

use algebraic_gen::generate_geometric_product;
generate_geometric_product!(4);

use super::MultiVector;

pub struct MultiVector4<T>(pub [T; 16]);

impl<T> IntoIterator for MultiVector4<T> {
    type Item = T;
    type IntoIter = <[T; 16] as IntoIterator>::IntoIter;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<T> Index<usize> for MultiVector4<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<T> IndexMut<usize> for MultiVector4<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl<T> AsRef<[T]> for MultiVector4<T> {
    fn as_ref(&self) -> &[T] {
        self.0.as_ref()
    }
}

impl<T> AsMut<[T]> for MultiVector4<T> {
    fn as_mut(&mut self) -> &mut [T] {
        self.0.as_mut()
    }
}

impl<T> Borrow<[T]> for MultiVector4<T> {
    fn borrow(&self) -> &[T] {
        self.0.borrow()
    }
}

impl<T> BorrowMut<[T]> for MultiVector4<T> {
    fn borrow_mut(&mut self) -> &mut [T] {
        self.0.borrow_mut()
    }
}

impl<T: Copy + Add<T, Output = T>> Add for MultiVector4<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self([
            self[00] + rhs[00],
            self[01] + rhs[01],
            self[02] + rhs[02],
            self[03] + rhs[03],
            self[04] + rhs[04],
            self[05] + rhs[05],
            self[06] + rhs[06],
            self[07] + rhs[07],
            self[08] + rhs[08],
            self[09] + rhs[09],
            self[10] + rhs[10],
            self[11] + rhs[11],
            self[12] + rhs[12],
            self[13] + rhs[13],
            self[14] + rhs[14],
            self[15] + rhs[15],
        ])
    }
}

impl<T: Copy + Sub<T, Output = T>> Sub for MultiVector4<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self([
            self[00] - rhs[00],
            self[01] - rhs[01],
            self[02] - rhs[02],
            self[03] - rhs[03],
            self[04] - rhs[04],
            self[05] - rhs[05],
            self[06] - rhs[06],
            self[07] - rhs[07],
            self[08] - rhs[08],
            self[09] - rhs[09],
            self[10] - rhs[10],
            self[11] - rhs[11],
            self[12] - rhs[12],
            self[13] - rhs[13],
            self[14] - rhs[14],
            self[15] - rhs[15],
        ])
    }
}

impl<T: Copy + Add<T, Output = T> + Sub<T, Output = T> + Mul<T, Output = T>> Mul<Self>
    for MultiVector4<T>
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(geometric_product_4(&self, &rhs))
    }
}

impl<T: Copy + Mul<T, Output = T>> Mul<T> for MultiVector4<T> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self([
            self[00] * rhs,
            self[01] * rhs,
            self[02] * rhs,
            self[03] * rhs,
            self[04] * rhs,
            self[05] * rhs,
            self[06] * rhs,
            self[07] * rhs,
            self[08] * rhs,
            self[09] * rhs,
            self[10] * rhs,
            self[11] * rhs,
            self[12] * rhs,
            self[13] * rhs,
            self[14] * rhs,
            self[15] * rhs,
        ])
    }
}

impl<T: Copy + Div<T, Output = T>> Div<T> for MultiVector4<T> {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Self([
            self[00] / rhs,
            self[01] / rhs,
            self[02] / rhs,
            self[03] / rhs,
            self[04] / rhs,
            self[05] / rhs,
            self[06] / rhs,
            self[07] / rhs,
            self[08] / rhs,
            self[09] / rhs,
            self[10] / rhs,
            self[11] / rhs,
            self[12] / rhs,
            self[13] / rhs,
            self[14] / rhs,
            self[15] / rhs,
        ])
    }
}

impl<T: Copy> Copy for MultiVector4<T> {}

impl<T: Clone> Clone for MultiVector4<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T: Copy + Default> Default for MultiVector4<T> {
    fn default() -> Self {
        Self([T::default(); 16])
    }
}

impl<T: Debug> Debug for MultiVector4<T> {
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
    > MultiVector<T> for MultiVector4<T>
{
    const ALGEBRA_DIMENSION: usize = 4;
}
