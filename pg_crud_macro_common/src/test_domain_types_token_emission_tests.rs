#[test]
fn test_domain_type_token_emission_tests() {
    let generated = crate::generate_query_part_error_write_into_buffer_token_stream::generate_query_part_error_write_into_buffer_token_stream(
        crate::import::Import::Crate,
    );
    assert!(!generated.as_ref().is_empty());
}
