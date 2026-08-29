#[must_use]
pub fn error_enum_d_token_stream_builder()
-> macro_helpers::derive_token_stream_builder::DTokenStreamBuilder {
    macro_helpers::derive_token_stream_builder::DTokenStreamBuilder::new()
        .make_pub()
        .d_debug()
        .d_thiserror_error()
        .d_location_location()
}
