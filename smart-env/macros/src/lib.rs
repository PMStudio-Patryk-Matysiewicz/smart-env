use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, Data, DeriveInput, Fields, GenericArgument, PathArguments, Type, TypePath,
};

fn extract_inner_option_type(ty: &Type) -> Option<&Type> {
    if let Type::Path(TypePath { path, .. }) = ty {
        if let Some(segment) = path.segments.last() {
            if segment.ident == "Option" {
                if let PathArguments::AngleBracketed(args) = &segment.arguments {
                    if let Some(GenericArgument::Type(inner_type)) = args.args.first() {
                        return Some(inner_type);
                    }
                }
            }
        }
    }
    None
}

#[proc_macro_derive(Config)]
pub fn derive_config(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;

    let fields = match input.data {
        Data::Struct(data_struct) => match data_struct.fields {
            Fields::Named(fields_named) => fields_named.named,
            Fields::Unnamed(_) => {
                panic!("#[derive(Config)] does not support tuple structs.");
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

        if let Some(inner_type) = extract_inner_option_type(field_type) {
            quote! {
                #field_name: store.get_parsed::<#inner_type>(#env_key).ok()
            }
        } else {
            let error_msg = format!(
                "Failed to load required environment variable '{}' for field '{}'",
                env_key, field_name
            );
            quote! {
                #field_name: store.get_parsed::<#field_type>(#env_key).expect(#error_msg)
            }
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
