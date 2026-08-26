#[must_use]
pub fn parse_strs_to_ts2_vec(
    v: crate::domain_types::ParseTokenStreamStrings,
    uuid: crate::domain_types::ParseErrorIdRef<'_>,
) -> crate::domain_types::ProcMacro2GeneratedRustTokenStreamVec {
    v.into_generated_vec(uuid)
}
