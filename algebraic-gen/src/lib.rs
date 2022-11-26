//! Proc macro crate for [algebraic](https://crates.io/crates/algebraic)
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
//! # Examples
//!
//! Pass a (small) positive integer to the macro and it generates the product function.
//!
//! ```
//! use algebraic_gen::generate_geometric_product;
//! use std::ops::{Add, Mul, Sub};
//!
//! generate_geometric_product!(3);
//! ```
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
        The arrays are coefficients for the following {} basis elements.",
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
