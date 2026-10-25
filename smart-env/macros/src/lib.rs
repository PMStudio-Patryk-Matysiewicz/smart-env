use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

#[proc_macro_derive(Config)]
pub fn derive_config(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;
    let expanded = quote! {
        impl #struct_name {
            pub fn load() -> Self {
                unimplemented!("Macro #[derive(Config)] for {} is in construction.", stringify!(#struct_name));
            }
        }
    };
    TokenStream::from(expanded)
}
