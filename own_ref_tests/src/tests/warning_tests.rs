#[test]
#[ignore]
fn should_compile() {
    let test = trybuild::TestCases::new();

    test.compile_fail("src/tests/warning_tests/manual_own_ref_creation.rs");
}
