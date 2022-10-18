#[test]
#[ignore]
fn should_compile() {
    let test = trybuild::TestCases::new();

    test.compile_fail("src/tests/failing_tests/immutable_own_ref.rs");
    test.compile_fail("src/tests/failing_tests/immutable_variable.rs");
}
