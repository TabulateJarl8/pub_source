/// Test funcs, bare structs, enums, types, consts, and statics

mod confined {
    pub_source::make_public! {
        fn test_func() {}
        struct TestStruct;
        enum TestEnum { TestVariant }
        type TestType = Vec<u32>;
        const TEST_CONST: f32 = 3.14;
        static TEST_STATIC: [u8; 1] = [1];
    }
}

fn main() {
    confined::test_func();

    let _ = confined::TestStruct;
    let _: confined::TestType = vec![0_u32];
    let _ = confined::TestEnum::TestVariant;

    let _ = confined::TEST_CONST;
    let _ = confined::TEST_STATIC;
}
