#![feature(trait_alias)]

/// Test trait aliases

mod confined {
    pub_source::make_public! {
        trait TestTrait = std::fmt::Debug;
    }
}

fn use_dgb<T: confined::TestTrait>(v: &T) {
    let _ = format!("{:?}", v);
}

fn main() {
    use_dgb(&1);
}
