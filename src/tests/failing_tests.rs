#[test]
fn should_compile() {
    let t = trybuild::TestCases::new();
    //pass("./failing_tests/good.rs");
    t.pass("src/tests/failing_tests/good.rs");
    t.compile_fail("src/tests/failing_tests/bad.rs");
    //t.compile_fail("./failing_tests/bad.rs");
}
