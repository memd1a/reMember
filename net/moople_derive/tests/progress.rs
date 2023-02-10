#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/01-parse.rs");
    //t.pass("tests/08-escape-hatch.rs");
}