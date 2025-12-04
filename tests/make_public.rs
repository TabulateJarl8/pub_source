use trybuild::TestCases;

#[test]
fn test_compilation() {
    let t = TestCases::new();
    t.pass("tests/ui/make_pub_basic.rs");
    t.pass("tests/ui/make_pub_struct.rs");
    t.pass("tests/ui/make_pub_trait.rs");
    t.pass("tests/ui/make_pub_impl_type.rs");
    t.pass("tests/ui/make_pub_mod.rs");
    t.pass("tests/ui/make_pub_union.rs");
    t.pass("tests/ui/make_pub_trait_alias.rs");
}
