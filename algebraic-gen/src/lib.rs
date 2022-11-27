//! Generates performant geometric products for any dimension of space.
//!
//! The products are free functions with (pseudo) signature
//!
//! ```fn geometric_product_D<A, B, T>(a: &A, b: &B) -> [T; 2.pow(D)];```
//!
//! Where `D` is the respective dimension and `T` can be any floating type.
//! `A` and `B` just need to implement `Index<usize, Output = T>`. E.g. `[f64; 8]` works for 3D.
//!
//! # Example
//!
//! The functions are straightforward to generate and use.
//! Pass a (small) positive integer to the macro and it generates the product function.
//! (check it out with `cargo expand`)
//!
//! ```
//! use algebraic_gen::generate_geometric_product;
//! use std::ops::{Add, Index, Mul, Sub};
//! generate_geometric_product!(3);
//!
//! fn main() {
//!   let a: [f64; 8] = [1., 2., 3., 4., 5., 6., 7., 8.];
//!   let b: [f64; 8] = [8., 7., 6., 5., 4., 3., 2., 1.];
//!
//!   let c = geometric_product_3(&a, &b);
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
//! The return types ```[T; 1]```, ```[T; 2]```, ```[T; 4]```, ```[T; 8]```, etc. are exactly those
//! representations of Multivectors of geometric algebras of spaces with 0, 1, 2, 3, etc.
//! dimensions, respectively.
//!
//! # Choice of Base
//!
//! The documentation of the choice of base is generated with the respective product function.
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
//! # On the Generation
//!
//! Maybe there is better way to get the products that doesn't use the \'sledgehammer\' proc macro
//! approach.
//! The generation is not performant, but we're talking build time.
//!
//! Definitely inspired by [All Hail Geometric Algebra!](https://crypto.stanford.edu/~blynn/haskell/ga.html)
//!
//! Brief description of the how the macro works:
//! * Generates elements (base)
//!   * Product is simple concatination here
//!   * Canonization with Bubblesort & a kind of duplication elimination
//! * Generates product sums
//!   * Expand terms
//!   * More products & canonization
//!   * Adding/Subtracting based on sign returned by canonization
//! * A bit of formatting
//!
//! More details in the source code: [algebra_generation.rs](https://github.com/Vollkornaffe/algebraic/blob/main/algebraic-gen/src/algebra_generation.rs).
//!
mod algebra_generation;
use algebra_generation::{generate_elements, generate_product_sums};
use core::str::FromStr;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::ExprArray;

fn generate_product_string(product_sums: &[Vec<(bool, usize, usize)>]) -> String {
    format!(
        "[{}]",
        product_sums
            .iter()
            .map(|sum| {
                sum.iter()
                    .enumerate()
                    .map(|(i, (n, a, b))| {
                        format!(
                            "{}a[{a}] * b[{b}]",
                            if i == 0 {
                                ""
                            } else if *n {
                                "- "
                            } else {
                                "+ "
                            },
                        )
                    })
                    .collect::<Vec<_>>()
                    .join(" ")
            })
            .collect::<Vec<_>>()
            .join(","),
    )
}

fn generate_base_string(elements: &[Vec<usize>]) -> String {
    let dimension = elements.len().trailing_zeros() as usize;
    elements.iter().enumerate().fold(
        "\n\n|Index|Outer Product|Type|\n|-|-|-|".to_string(),
        |bases, (i, element)| {
            format!(
                "{bases}\n|{}|{:?}|{}|",
                i,
                element,
                match element.len() {
                    0 => "Scalar".to_string(),
                    k if k == dimension => "Pseudoscalar".to_string(),
                    1 => "Vector".to_string(),
                    2 => "Bivector".to_string(),
                    3 => "Trivector".to_string(),
                    4 => "Quadvector".to_string(),
                    5 => "Quintvector".to_string(),
                    k => format!("{}-Vector", k),
                }
            )
        },
    )
}

/// The one macro exported by this crate
#[proc_macro]
pub fn generate_geometric_product(input: TokenStream) -> TokenStream {
    let lit: syn::LitInt = syn::parse(input).unwrap();
    let dimension = lit.base10_parse::<usize>().unwrap();

    let function_ident = format_ident!("geometric_product_{}", dimension);

    let elements = generate_elements(dimension);
    let product_sums = generate_product_sums(&elements);
    let product_string = generate_product_string(&product_sums);

    let array_length = elements.len();

    // this is the really crazy part, I don't know how to do it better
    // first put the array expression into a string, then parse it
    // and finally put it into a function via quote
    let product_stream = TokenStream::from_str(&product_string).unwrap();
    let product: ExprArray = syn::parse(product_stream).unwrap();

    let basis = generate_base_string(&elements);
    let dimension = format!("{}", dimension);
    let documentation = format!(
        "Calculates the geometric product for multivectors of {dimension}-dimensional space.
        The arrays are coefficient representations wrt. the following {} basis elements.",
        array_length
    );

    let gen = quote! {
        #[doc = #documentation]
        #[doc = #basis]
        pub fn #function_ident<A, B, T>(a: &A, b: &B) -> [T; #array_length]
        where
            A: Index<usize, Output = T>,
            B: Index<usize, Output = T>,
            T: Copy + Mul<T, Output = T> + Add<T, Output = T> + Sub<T, Output = T>,
        {
            #product
        }
    };

    gen.into()
}
