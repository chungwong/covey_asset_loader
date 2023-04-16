use quote::quote;
use syn::{parse::Result, Data, Fields, LitStr, Meta};

const ASSET_ATTRIBUTE: &str = "asset";
const PATH_ATTRIBUTE: &str = "path";

pub(crate) fn impl_asset_collection(input: syn::DeriveInput) -> Result<proc_macro2::TokenStream> {
    let ident = &input.ident;

    let mut paths = vec![];

    if let Data::Struct(data_struct) = input.data {
        if let Fields::Named(ref named_fields) = data_struct.fields {
            for field in named_fields.named.iter() {
                let field_name = &field.ident;
                for attr in field.attrs.iter() {
                    if let Meta::List(ref meta_list) = attr.meta {
                        if *meta_list.path.get_ident().unwrap() != ASSET_ATTRIBUTE {
                            continue;
                        }

                        attr.parse_nested_meta(|meta| {
                            if meta.path.is_ident(PATH_ATTRIBUTE) {
                                let value: LitStr = meta.value()?.parse()?;

                                paths.push(quote! {
                                    #field_name: asset_server.load(#value),
                                });

                                Ok(())
                            } else {
                                Err(meta.error("unsupported asset attribute"))
                            }
                        })?;
                    }
                }
            }
        }
    }

    Ok(quote! {
        impl FromWorld for #ident {
            fn from_world(world: &mut World) -> Self {
                let asset_name = std::any::type_name::<Self>();

                debug!("initializing {} in form_world", asset_name);

                let Some(asset_server) = world.get_resource::<AssetServer>() else {
                    panic!("failed to load AssetServer for {}", asset_name);
                };

                Self {
                    #(#paths)*
                }
            }
        }
    })
}
