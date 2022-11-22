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

    let gen = quote! {
        pub fn #function_ident<E>(a: &[E; #array_length], b: &[E; #array_length]) -> [E; #array_length]
        where
            E: Copy + Mul<E, Output = E> + Add<E, Output = E> + Sub<E, Output = E>,
        {
            #product
        }
    };

    gen.into()
}
