#[allow(unused_attributes)]
#[rustversion::attr(not(nightly), ignore)]
#[test]
fn no_std() {
    let t = trybuild::TestCases::new();

    t.pass("tests/no_std/enum_prefix.rs");
    t.pass("tests/no_std/multi_line_allow.rs");
    t.pass("tests/no_std/multi_line.rs");
    t.pass("tests/no_std/multiple.rs");
    t.pass("tests/no_std/with.rs");

    #[cfg(not(feature = "std"))]
    {
        t.compile_fail("tests/no_std/without.rs");
        t.compile_fail("tests/no_std/enum_prefix_missing.rs");
    }

    #[cfg(feature = "std")]
    {
        t.compile_fail("tests/std/without.rs");
        t.compile_fail("tests/std/enum_prefix_missing.rs");
    }
}
