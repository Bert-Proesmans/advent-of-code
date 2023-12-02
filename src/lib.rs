use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, AttributeArgs, Ident, ItemFn, Lit, NestedMeta};

#[proc_macro_attribute]
pub fn main(args: TokenStream, input: TokenStream) -> TokenStream {
    // Load input stream from templated path
    let input_path = match &parse_macro_input!(args as AttributeArgs)[..] {
        [NestedMeta::Lit(Lit::Int(year)), NestedMeta::Lit(Lit::Int(day))] => {
            format!("../../inputs/{}/{}.txt", year.token(), day.token())
        }
        _ => panic!("Expected one integer argument"),
    };

    // Rename solution function, in case it's called main (which would clash with our added method)
    let mut aoc_solution = parse_macro_input!(input as ItemFn);
    aoc_solution.sig.ident = Ident::new("aoc_solution", aoc_solution.sig.ident.span());

    let tokens = quote! {
      const INPUT: &str = include_str!(#input_path);
      #aoc_solution
      fn main() {
        let result = aoc_solution(INPUT.trim_end());
        // We expect result, whatever it is, to implement fmt::Display
        println!("Result: {:}", result);
      }
    };
    TokenStream::from(tokens)
}
