/// Test traits

mod confined {
    pub_source::make_public! {
        trait TestTrait {
            fn ret() -> u8;
        }
    }
}

fn main() {
    // test that we can access the trait
    struct Tmp;
    impl confined::TestTrait for Tmp {
        fn ret() -> u8 {
            0
        }
    }
}
