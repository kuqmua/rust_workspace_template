#[must_use]
pub fn common_d_token_stream_builder()
-> macro_helpers::domain_types::derive_token_stream_builder::DTokenStreamBuilder {
    macro_helpers::domain_types::derive_token_stream_builder::DTokenStreamBuilder::new()
        .make_pub()
        .d_debug()
        .d_clone()
        .d_partial_eq()
        .d_serde_serialize()
        .d_serde_deserialize()
}
