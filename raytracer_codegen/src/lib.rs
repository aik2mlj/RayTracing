// #![allow(clippy::all)]
mod scene_gen;
mod shared_tools;
mod vec3;

// extern crate proc_macro;
// use proc_macro2::TokenStream;
// use syn::DeriveInput;
// use quote::quote;
use scene_gen::*;

#[proc_macro]
pub fn bvhnode_impl(_input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    build_static_scenes()
}
