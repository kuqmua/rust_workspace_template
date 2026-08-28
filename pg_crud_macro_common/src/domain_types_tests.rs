#[test]
fn domain_type_tests() {
    assert_eq!(super::Import::Crate.sc_str().as_ref(), constants_str::CRATE);
}
