mod asset_collection;
mod asset_state;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

use crate::{asset_collection::impl_asset_collection, asset_state::impl_asset_state};

#[proc_macro_derive(AssetCollection, attributes(asset))]
pub fn derive_asset_collection(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    impl_asset_collection(input)
        .unwrap_or_else(to_compile_error)
        .into()
}

#[proc_macro_derive(AssetState, attributes(assets))]
pub fn derive_asset_state(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    impl_asset_state(input)
        .unwrap_or_else(to_compile_error)
        .into()
}

fn to_compile_error(error: syn::Error) -> proc_macro2::TokenStream {
    error.to_compile_error()
}
