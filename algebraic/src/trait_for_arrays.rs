use super::*;
use std::ops::{Add, Mul, Sub};
use std::{
    borrow::BorrowMut,
    convert::{AsMut, AsRef},
    fmt::Debug,
    ops::IndexMut,
};

pub trait GeometricProduct<E>:
    Copy
    + Debug
    + Default
    + IntoIterator<Item = E>
    + IndexMut<usize, Output = E>
    + PartialOrd
    + AsRef<[E]>
    + AsMut<[E]>
    + BorrowMut<[E]>
{
    const D: usize;
    const N: usize = 1 << Self::D;
    fn geometric_product(a: &Self, b: &Self) -> Self;
}

impl<
        E: Copy + Debug + Default + PartialOrd + Mul<Output = E> + Add<Output = E> + Sub<Output = E>,
    > GeometricProduct<E> for [E; 1]
{
    const D: usize = 0;
    fn geometric_product(a: &Self, b: &Self) -> Self {
        geometric_product_0(a, b)
    }
}

impl<
        E: Copy + Debug + Default + PartialOrd + Mul<Output = E> + Add<Output = E> + Sub<Output = E>,
    > GeometricProduct<E> for [E; 2]
{
    const D: usize = 1;
    fn geometric_product(a: &Self, b: &Self) -> Self {
        geometric_product_1(a, b)
    }
}

impl<
        E: Copy + Debug + Default + PartialOrd + Mul<Output = E> + Add<Output = E> + Sub<Output = E>,
    > GeometricProduct<E> for [E; 4]
{
    const D: usize = 2;
    fn geometric_product(a: &Self, b: &Self) -> Self {
        geometric_product_2(a, b)
    }
}

impl<
        E: Copy + Debug + Default + PartialOrd + Mul<Output = E> + Add<Output = E> + Sub<Output = E>,
    > GeometricProduct<E> for [E; 8]
{
    const D: usize = 3;
    fn geometric_product(a: &Self, b: &Self) -> Self {
        geometric_product_3(a, b)
    }
}

impl<
        E: Copy + Debug + Default + PartialOrd + Mul<Output = E> + Add<Output = E> + Sub<Output = E>,
    > GeometricProduct<E> for [E; 16]
{
    const D: usize = 4;
    fn geometric_product(a: &Self, b: &Self) -> Self {
        geometric_product_4(a, b)
    }
}

impl<
        E: Copy + Debug + Default + PartialOrd + Mul<Output = E> + Add<Output = E> + Sub<Output = E>,
    > GeometricProduct<E> for [E; 32]
{
    const D: usize = 5;
    fn geometric_product(a: &Self, b: &Self) -> Self {
        geometric_product_5(a, b)
    }
}
