#[must_use]
pub fn parse_strs_to_ts2_vec(
    v: crate::parse_token_stream_strings::ParseTokenStreamStrings,
    uuid: crate::parse_error_id_ref::ParseErrorIdRef<'_>,
) -> crate::proc_macro2_generated_rust_token_stream_vec::ProcMacro2GeneratedRustTokenStreamVec {
    v.into_generated_vec(uuid)
}
