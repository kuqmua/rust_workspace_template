#[test]
fn facade_exposes_emission_types() {
    assert_eq!(super::Import::Crate.sc_str().as_ref(), constants_str::CRATE);
}
