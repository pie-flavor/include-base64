extern crate include_base64;

const EX1_BASE64: &str = include_base64::include_base64!("tests/example1.txt");
const EX2_BASE64_STD: &str = include_base64::include_base64_std!("tests/example2.txt");
const EX2_BASE64_URL_SAFE: &str = include_base64::include_base64!("tests/example2.txt");

#[test]
fn simple() {
    assert_eq!(EX1_BASE64, include_str!("example1_base64.txt"));
    assert_eq!(EX2_BASE64_STD, include_str!("example2_base64.txt"));
    assert_eq!(
        EX2_BASE64_URL_SAFE,
        include_str!("example2_base64_url_safe.txt")
    );
}
