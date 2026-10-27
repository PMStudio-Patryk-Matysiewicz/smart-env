use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_derive(Config)]
pub fn derive_config(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;

    let fields = match input.data {
        Data::Struct(data_struct) => match data_struct.fields {
            Fields::Named(fields_named) => fields_named.named,
            Fields::Unnamed(_) => {
                panic!("#[derive(Config)] does not support tuple structs (e.g., struct Foo(u32)).");
            }
            Fields::Unit => {
                panic!("#[derive(Config)] requires a struct with named fields.");
            }
        },
        Data::Enum(_) => {
            panic!("#[derive(Config)] can only be applied to structs, not enums.");
        }
        Data::Union(_) => {
            panic!("#[derive(Config)] does not support unions.");
        }
    };

    let field_initializers = fields.iter().map(|f| {
        let field_name = f.ident.as_ref().expect("Named field must have an identifier");
        let field_type = &f.ty;
        let env_key = field_name.to_string().to_uppercase();

        quote! {
            #field_name: store.get_parsed::<#field_type>(#env_key).unwrap()
        }
    });

    let expanded = quote! {
        impl #struct_name {
            pub fn load() -> Self {
                let store = smart_env_core::ConfigBuilder::new()
                    .load_env_file(".env")
                    .load_system_env()
                    .build();

                Self {
                    #(#field_initializers),*
                }
            }
        }
    };
    TokenStream::from(expanded)
}
