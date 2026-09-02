#[must_use]
pub fn parse_strs_to_ts2_vec(
    parse_token_stream_strings: crate::parse_token_stream_strings::ParseTokenStreamStrings,
    parse_error_id_ref: crate::parse_error_id_ref::ParseErrorIdRef<'_>,
) -> crate::proc_macro2_generated_rust_token_stream_vec::ProcMacro2GeneratedRustTokenStreamVec {
    parse_token_stream_strings.into_generated_vec(parse_error_id_ref)
}
