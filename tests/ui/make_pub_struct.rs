/// Test Struct, struct fields, impl funcs, impl associated consts

mod confined {
    pub_source::make_public! {
        struct TestStruct {
            field1: u32,
            field2: u32,
        }

        impl TestStruct {
            const TEST_CONST: u8 = 2;

            fn add(&self) -> u32 {
                self.field1 + self.field2
            }
        }
    }
}

fn main() {
    let s = confined::TestStruct {
        field1: 3,
        field2: 6,
    };

    let _ = s.field1;
    let _ = s.field2;

    assert_eq!(9, s.add());

    let _ = confined::TestStruct::TEST_CONST;
}
