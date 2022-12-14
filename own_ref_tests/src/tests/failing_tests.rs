#[test]
#[ignore]
fn should_compile() {
    let test = trybuild::TestCases::new();

    test.compile_fail("src/tests/failing_tests/immutable_own_ref.rs");
    test.compile_fail("src/tests/failing_tests/immutable_variable.rs");
    test.compile_fail("src/tests/failing_tests/outlive_function.rs");
    test.compile_fail("src/tests/failing_tests/outlive_scope.rs");
    test.compile_fail("src/tests/failing_tests/use_after_own_ref.rs");
}
