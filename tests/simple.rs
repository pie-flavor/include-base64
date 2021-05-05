extern crate include_base64;

const BASE64: &str = include_base64::include_base64!("tests/example.txt");

#[test]
fn simple() {
    assert_eq!(BASE64, include_str!("example_base64.txt"));
}
