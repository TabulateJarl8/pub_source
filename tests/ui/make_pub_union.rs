/// Test unions

mod confined {
    pub_source::make_public! {
        union IntOrFloat {
            i: u32,
            f: f32,
        }
    }
}

fn main() {
    let u1 = confined::IntOrFloat { i: 4 };
    assert_eq!(unsafe { u1.i }, 4);

    let u2 = confined::IntOrFloat { f: 4.0 };
    assert_eq!(unsafe { u2.f }, 4.0);
}
