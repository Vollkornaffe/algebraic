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
    let documentation = format!(
        "```{function_ident}``` calculates the geometric product of two multivectors (aka. Clifford
        numbers or k-blades) from a geometric algebra of {}-dimensional space. The arguments and
        return value are coefficient representations (```[T; {}]```) of the multivectors. ```T```
        is meant to be a floating point type. The coefficients map to the ```2^{}``` orthogonal
        basis elements which are listed below. The integers in the \'Outer Product\' column
        represent different unit vectors and their wedge product. ",
        dimension, array_length, dimension
    );

    let gen = quote! {
        #[doc = #documentation]
        #[doc = #basis]
        pub fn #function_ident<T>(a: &[T; #array_length], b: &[T; #array_length]) -> [T; #array_length]
        where
            T: Copy + Mul<T, Output = T> + Add<T, Output = T> + Sub<T, Output = T>,
        {
            #product
        }
    };

    gen.into()
}
