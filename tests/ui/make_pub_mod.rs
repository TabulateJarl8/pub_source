/// Test modules and nested modules

mod confined {
    pub_source::make_public! {
        mod submod {
            const TEST: u8 = 3;
        }

        mod nested1 {
            mod nested2 {
                fn secret() -> u8 { 9 }
            }
        }
    }
}

fn main() {
    assert_eq!(confined::submod::TEST, 3);
    assert_eq!(confined::nested1::nested2::secret(), 9);
}
