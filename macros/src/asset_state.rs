use quote::quote;
use syn::{parse::Result, Data, Ident};

const ASSET_ATTRIBUTE: &str = "assets";

pub(crate) fn impl_asset_state(input: syn::DeriveInput) -> Result<proc_macro2::TokenStream> {
    let ident = &input.ident;

    let mut state_assets = vec![];

    if let Data::Enum(data_enum) = input.data {
        for variant in data_enum.variants.iter() {
            let variant_name = &variant.ident;
            for attr in variant.attrs.iter() {
                if *attr.path().get_ident().unwrap() != ASSET_ATTRIBUTE {
                    continue;
                }

                let asset_struct: Ident = attr.parse_args()?;

                state_assets.push(quote! {
                    #ident::#variant_name => commands.init_assets::<#asset_struct>(),
                });
            }
        }
    }

    Ok(quote! {
        impl AssetState for #ident {
            fn load_assets(&self, mut commands: Commands, mut next_state: ResMut<NextState<Self>>) {
                match self {
                    #(#state_assets)*
                    // Update NextState if the state requires no assets.
                    _ => {
                        debug!(
                            "no assets required for State {:?}, transiting to it directly",
                            self
                        );
                        next_state.set(*self)
                    }
                }
            }
        }
    })
}
