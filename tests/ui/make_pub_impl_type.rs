#![feature(inherent_associated_types)]
#![allow(incomplete_features)]

/// Test (unstable) associated impl types

#[allow(incomplete_features)]
mod confined {
    pub_source::make_public! {
        struct TestStruct;

        impl TestStruct {
            type TestType = u8;
        }
    }
}

fn main() {
    let _: confined::TestStruct::TestType = 0_u8;
}
