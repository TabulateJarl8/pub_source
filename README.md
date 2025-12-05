# `pub_source` - Make Everything Public

`pub_source` provides the `make_public!` procedural macro, which rewrites a block of Rust source code so that all top level items become public.

This macro parses the input as a full `syn::File` and rewrites the following kinds of items to `pub`:

- functions
- structs and all of their fields
- enums
- type aliases
- constants and statics
- traits
- modules (recursively)
- impl blocks (functions, consts, type items inside them)
- unions

Non-items such as `use`, macros, or foreign modules are left unchanged.

This crate also denies use of `unwrap`, `expect`, and `panic!()`.

## Usage

```sh
cargo add pub_source
```

Then the crate can be used as follows:

```rust
pub_source::make_public! {
    fn hidden() {}

    struct Thing {
        a: u32,
        b: String,
    }

    impl Thing {
        fn show(&self) {
            println!("{}", self.b);
        }
    }
}
```

Expands to code equivalent to:

```rust
pub fn hidden() {}

pub struct Thing {
    pub a: u32,
    pub b: String,
}

impl Thing {
    pub fn show(&self) {
        println!("{}", self.b);
    }
}
```

This was originally written to be injected around user-submitted code in a code runner so that unit tests could access everything the user wrote. There may be other uses but I'm not quite sure what they might be yet.

## Feature Flags

This crate provides two feature flags:

- `std` - used for enabling stdlib support, enabled by default
- `unstable` - used for enabling unstable features (trait aliases, impl-associated types) on nightly compilers that are using these features

## License

This crate is licensed under the MIT license
