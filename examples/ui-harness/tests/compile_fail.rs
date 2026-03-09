#[test]
fn compiler_diagnostics_stay_part_of_the_book() {
    let tests = trybuild::TestCases::new();
    tests.compile_fail("tests/ui/*.rs");
}
