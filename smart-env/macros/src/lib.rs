use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, Data, DeriveInput, Field, Fields, GenericArgument, PathArguments, Type, TypePath,
};

#[derive(Default)]
struct FieldConfig {
    default: Option<String>,
    env: Option<String>,
    nested: bool,
    prefix: Option<String>,
}

fn parse_field_config(field: &Field) -> FieldConfig {
    let mut cfg = FieldConfig::default();

    for attr in &field.attrs {
        if attr.path().is_ident("config") {
            let _ = attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("default") {
                    let lit: syn::LitStr = meta.value()?.parse()?;
                    cfg.default = Some(lit.value());
                    Ok(())
                } else if meta.path.is_ident("env") {
                    let lit: syn::LitStr = meta.value()?.parse()?;
                    cfg.env = Some(lit.value());
                    Ok(())
                } else if meta.path.is_ident("nested") {
                    cfg.nested = true;
                    Ok(())
                } else if meta.path.is_ident("prefix") {
                    let lit: syn::LitStr = meta.value()?.parse()?;
                    cfg.prefix = Some(lit.value());
                    Ok(())
                } else {
                    Err(meta.error("unrecognized #[config(...)] attribute key"))
                }
            });
        }
    }
    cfg
}

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

#[proc_macro_derive(Config, attributes(config))]
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

let from_store_initializers: Vec<_> = fields
    .iter()
    .map(|f| {
        let field_name = f.ident.as_ref().expect("Named field must have an identifier");
        let field_type = &f.ty;
        let field_cfg = parse_field_config(f);

        if field_cfg.nested {
            let nested_prefix = field_cfg.prefix.unwrap_or_else(|| {
                format!("{}_", field_name.to_string().to_uppercase())
            });
            quote! {
                #field_name: #field_type::from_store(store, &format!("{}{}", prefix, #nested_prefix))
            }
        } else {
            let env_key = field_cfg
                .env
                .unwrap_or_else(|| field_name.to_string().to_uppercase());

            if let Some(inner_type) = extract_inner_option_type(field_type) {
                if let Some(default_val) = field_cfg.default {
                    quote! {
                        #field_name: store.get_parsed::<#inner_type>(&format!("{}{}", prefix, #env_key))
                            .ok()
                            .or_else(|| smart_env_core::parser::FromEnv::from_env(#default_val).ok())
                    }
                } else {
                    quote! {
                        #field_name: store.get_parsed::<#inner_type>(&format!("{}{}", prefix, #env_key)).ok()
                    }
                }
            } else if let Some(default_val) = field_cfg.default {
                let parse_err_msg = format!(
                    "Failed to parse default value '{}' for field '{}'",
                    default_val, field_name
                );
                quote! {
                    #field_name: store.get_parsed::<#field_type>(&format!("{}{}", prefix, #env_key))
                        .unwrap_or_else(|_| {
                            smart_env_core::parser::FromEnv::from_env(#default_val)
                                .expect(#parse_err_msg)
                        })
                }
            } else {
                let missing_err_msg = format!(
                    "Failed to load required environment variable '{}' for field '{}'",
                    env_key, field_name
                );
                quote! {
                    #field_name: store.get_parsed::<#field_type>(&format!("{}{}", prefix, #env_key)).expect(#missing_err_msg)
                }
            }
        }
    }).collect();

    let try_from_store_initializers: Vec<_> = fields
    .iter()
    .map(|f| {
        let field_name = f.ident.as_ref().expect("Named field must have an identifier");
        let field_type = &f.ty;
        let field_cfg = parse_field_config(f);

        if field_cfg.nested {
            let nested_prefix = field_cfg.prefix.unwrap_or_else(|| {
                format!("{}_", field_name.to_string().to_uppercase())
            });
            quote! {
                #field_name: #field_type::try_from_store(store, &format!("{}{}", prefix, #nested_prefix))?
            }
        } else {
            let env_key = field_cfg
                .env
                .unwrap_or_else(|| field_name.to_string().to_uppercase());

            if let Some(inner_type) = extract_inner_option_type(field_type) {
                if let Some(default_val) = field_cfg.default {
                    quote! {
                        #field_name: store.get_parsed::<#inner_type>(&format!("{}{}", prefix, #env_key))
                            .ok()
                            .or_else(|| smart_env_core::parser::FromEnv::from_env(#default_val).ok())
                    }
                } else {
                    quote! {
                        #field_name: store.get_parsed::<#inner_type>(&format!("{}{}", prefix, #env_key)).ok()
                    }
                }
            } else if let Some(default_val) = field_cfg.default {
                quote! {
                    #field_name: store.get_parsed::<#field_type>(&format!("{}{}", prefix, #env_key))
                        .or_else(|_| smart_env_core::parser::FromEnv::from_env(#default_val))?
                }
            } else {
                quote! {
                    #field_name: store.get_parsed::<#field_type>(&format!("{}{}", prefix, #env_key))?
                }
            }
        }
    }).collect();

    let expanded = quote! {
        impl #struct_name {
            pub fn from_store(store: &smart_env_core::EnvStore, prefix: &str) -> Self {
                Self {
                    #(#from_store_initializers),*
                }
            }

            pub fn try_from_store(store: &smart_env_core::EnvStore, prefix: &str) -> Result<Self, smart_env_core::EnvError> {
                Ok(Self {
                    #(#try_from_store_initializers),*
                })
            }
            pub fn load() -> Self {
                let store = smart_env_core::ConfigBuilder::new()
                    .load_env_file(".env")
                    .load_system_env()
                    .build();

                Self::from_store(&store, "")
            }

            pub fn load_from_vars<K, V, I>(vars: I) -> Self
            where
                K: Into<String>,
                V: Into<String>,
                I: IntoIterator<Item = (K, V)>,
            {
                let store = smart_env_core::ConfigBuilder::new()
                    .with_vars(vars)
                    .build();

                Self::from_store(&store, "")
            }

            pub fn try_load() -> Result<Self, smart_env_core::EnvError> {
                let store = smart_env_core::ConfigBuilder::new()
                    .load_env_file(".env")
                    .load_system_env()
                    .build();

                Self::try_from_store(&store, "")
            }

            pub fn try_load_from_vars<K, V, I>(vars: I) -> Result<Self, smart_env_core::EnvError>
            where
                K: Into<String>,
                V: Into<String>,
                I: IntoIterator<Item = (K, V)>,
            {
                let store = smart_env_core::ConfigBuilder::new()
                    .with_vars(vars)
                    .build();

                Self::try_from_store(&store, "")
            }
        }
    };

    TokenStream::from(expanded)
}