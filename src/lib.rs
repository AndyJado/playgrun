use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Makro)]
pub fn ma_cro(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let expanded = quote! {
        impl #name {
            fn maka(&self) {
                println!("maka")
            }
        }
    };

    TokenStream::from(expanded)
}
