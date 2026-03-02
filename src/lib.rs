//! `pub_source` provides the [`make_public!`] procedural macro, which rewrites a block
//! of Rust source code so that all top level items become public.
//!
//! This macro parses the input as a full [`syn::File`] and rewrites the following kinds
//! of items to `pub`:
//!
//! - functions
//! - structs and all of their fields
//! - enums
//! - type aliases
//! - constants and statics
//! - traits
//! - modules (recursively)
//! - impl blocks (functions, consts, type items inside them)
//! - unions
//!
//! Non-items such as `use`, macros, or foreign modules are left unchanged.
//!
//! This crate is also, `unwrap`, `expect`, and `panic!()` deny use.
//!
//! ## Use case
//!
//! This was originally written to be injected around user-submitted code in a code runner
//! so that unit tests could access everything the user wrote. There may be other uses but
//! I'm not quite sure what they might be yet.
//!
//! ## Feature Flags
//!
//! This crate provides two feature flags:
//!
//! - `std` - used for enabling stdlib support, enabled by default
//! - `unstable` - used for enabling unstable features (trait aliases, impl-associated types) on nightly compilers that are using these features
//!
//! ## Example
//!
//! Input:
//!
//! ```rust
//! pub_source::make_public! {
//!     fn hidden() {}
//!
//!     struct Thing {
//!         a: u32,
//!         b: String,
//!     }
//!
//!     impl Thing {
//!         fn show(&self) {
//!             println!("{}", self.b);
//!         }
//!     }
//! }
//! ```
//!
//! Expands to code equivalent to:
//!
//! ```rust
//! pub fn hidden() {}
//!
//! pub struct Thing {
//!     pub a: u32,
//!     pub b: String,
//! }
//!
//! impl Thing {
//!     pub fn show(&self) {
//!         println!("{}", self.b);
//!     }
//! }
//! ```

#![cfg_attr(not(feature = "std"), no_std)]
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, token, File, Item, Visibility};

/// Recursively makes all top level items in a source public.
///
/// Accepts a block of Rust code and makes it so that:
///
/// - Functions become `pub fn`
/// - Structs become `pub struct`, and all of their fields become public
/// - Enums, types, traits (and trait aliases if the `unstable` feature is enabled), unions, statics, and constants become `pub`
/// - Modules become `pub mod` and everything underneath them recursively becomes `pub`
/// - `impl` blocks have all inner items made public (`fn`, `const`, `type`), unless implementing a
///   trait
/// - `impl`-associated types are made `pub` if the `unstable` feature is enabled
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
    ast.items.iter_mut().for_each(make_item_public);
    quote! { #ast }.into()
}

fn make_item_public(item: &mut Item) {
    let p = Visibility::Public(token::Pub::default());

    match item {
        Item::Fn(func) => func.vis = p.clone(),
        Item::Mod(m) => {
            m.vis = p.clone();
            if let Some((_, items)) = &mut m.content {
                items.iter_mut().for_each(make_item_public);
            }
        }
        Item::Struct(s) => {
            s.vis = p.clone();
            s.fields.iter_mut().for_each(|f| f.vis = p.clone());
        }
        Item::Impl(i) => {
            if i.trait_.is_none() {
                // only make non-trait impls public
                i.items.iter_mut().for_each(|item| match item {
                    syn::ImplItem::Const(impl_item_const) => {
                        impl_item_const.vis = p.clone();
                    }
                    syn::ImplItem::Fn(impl_item_fn) => impl_item_fn.vis = p.clone(),

                    // impl-associated types are unstable
                    #[cfg(feature = "unstable")]
                    syn::ImplItem::Type(impl_item_type) => impl_item_type.vis = p.clone(),

                    _ => (),
                });
            }
        }
        Item::Const(item_const) => item_const.vis = p.clone(),
        Item::Enum(item_enum) => item_enum.vis = p.clone(),
        Item::Static(item_static) => item_static.vis = p.clone(),
        Item::Trait(item_trait) => item_trait.vis = p.clone(),
        Item::Type(item_type) => item_type.vis = p.clone(),
        Item::Union(item_union) => {
            item_union.vis = p.clone();
            item_union
                .fields
                .named
                .iter_mut()
                .for_each(|f| f.vis = p.clone());
        }

        // trait aliases are unstable
        #[cfg(feature = "unstable")]
        Item::TraitAlias(item_trait_alias) => item_trait_alias.vis = p.clone(),

        Item::Macro(_)
        | Item::ForeignMod(_)
        | Item::Use(_)
        | Item::Verbatim(_)
        | Item::ExternCrate(_)
        | _ => (),
    }
}
