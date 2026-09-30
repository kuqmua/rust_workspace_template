#[must_use]
pub fn common_derive_token_stream_builder()
-> macro_helpers::derive_token_stream_builder::DeriveTokenStreamBuilder {
    macro_helpers::derive_token_stream_builder::DeriveTokenStreamBuilder::new()
        .make_pub()
        .derive_debug()
        .derive_clone()
        .derive_partial_eq()
        .derive_serde_serialize()
        .derive_serde_deserialize()
}
