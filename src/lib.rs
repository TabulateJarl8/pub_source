#![doc = include_str!("../README.md")]
#![cfg_attr(not(feature = "std"), no_std)]
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, parse_quote, File, Item};

/// Recursively makes all top level items in a source public.
///
/// Accepts a block of Rust code and makes it so that:
///
/// - Functions become `pub fn`
/// - Structs become `pub struct`, and all of their fields become public
/// - Enums, types, traits (and trait aliases), unions, statics, and constants become `pub`
/// - Modules become `pub mod` and everything underneath them recursively becomes `pub`
/// - `impl` blocks have all inner items made public (`fn`, `const`, `type`), unless implementing a
///   trait
///
///
/// # Example
///
/// ```rust
/// pub_source::make_public! {
///     fn test() {}
///
///     struct Thing {
///         field1: u64,
///         field2: String,
///     }
///
///     impl Thing {
///         fn fancy(&self) {
///             let _ = self.field1;
///         }
///     }
/// }
/// ```
///
/// Expands to:
///
/// ```rust
/// pub fn test() {}
///
/// pub struct Thing {
///     pub field1: u64,
///     pub field2: String,
/// }
///
/// impl Thing {
///     pub fn fancy(&self) {
///         let _ = self.field1;
///     }
/// }
/// ```
#[proc_macro]
pub fn make_public(input: TokenStream) -> TokenStream {
    let mut ast = parse_macro_input!(input as File);

    for item in &mut ast.items {
        make_item_public(item);
    }

    quote! { #ast }.into()
}

fn make_item_public(item: &mut Item) {
    match item {
        Item::Fn(func) => func.vis = parse_quote!(pub),
        Item::Mod(m) => {
            m.vis = parse_quote!(pub);
            if let Some((_, items)) = &mut m.content {
                for sub in items {
                    make_item_public(sub);
                }
            }
        }
        Item::Struct(s) => {
            s.vis = parse_quote!(pub);
            s.fields.iter_mut().for_each(|f| f.vis = parse_quote!(pub));
        }
        Item::Impl(i) => {
            if i.trait_.is_none() {
                // only make non-trait impls public
                i.items.iter_mut().for_each(|item| match item {
                    syn::ImplItem::Const(impl_item_const) => {
                        impl_item_const.vis = parse_quote!(pub);
                    }
                    syn::ImplItem::Fn(impl_item_fn) => impl_item_fn.vis = parse_quote!(pub),
                    syn::ImplItem::Type(impl_item_type) => impl_item_type.vis = parse_quote!(pub),
                    _ => (),
                });
            }
        }
        Item::Const(item_const) => item_const.vis = parse_quote!(pub),
        Item::Enum(item_enum) => item_enum.vis = parse_quote!(pub),
        Item::Static(item_static) => item_static.vis = parse_quote!(pub),
        Item::Trait(item_trait) => item_trait.vis = parse_quote!(pub),
        Item::Type(item_type) => item_type.vis = parse_quote!(pub),
        Item::Union(item_union) => {
            item_union.vis = parse_quote!(pub);
            item_union
                .fields
                .named
                .iter_mut()
                .for_each(|f| f.vis = parse_quote!(pub));
        }
        Item::TraitAlias(item_trait_alias) => item_trait_alias.vis = parse_quote!(pub),
        Item::Macro(_)
        | Item::ForeignMod(_)
        | Item::Use(_)
        | Item::Verbatim(_)
        | Item::ExternCrate(_)
        | _ => (),
    }
}
