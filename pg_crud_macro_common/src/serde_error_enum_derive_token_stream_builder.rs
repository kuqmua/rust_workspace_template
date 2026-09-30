#[must_use]
pub fn serde_error_enum_derive_token_stream_builder()
-> macro_helpers::derive_token_stream_builder::DeriveTokenStreamBuilder {
    macro_helpers::derive_token_stream_builder::DeriveTokenStreamBuilder::new()
        .make_pub()
        .derive_debug()
        .derive_serde_serialize()
        .derive_serde_deserialize()
        .derive_thiserror_error()
        .derive_proc_macro_location_derive_location_location()
}
