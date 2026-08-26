#[test]
fn query_part_error_generation_is_not_empty() {
    let generated = super::generate_query_part_error_write_into_buffer_token_stream(
        crate::domain_types::Import::Crate,
    );
    assert!(!generated.as_ref().is_empty());
}
