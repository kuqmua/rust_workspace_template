pub(super) fn str_case<S>(
    s: S,
    convert_case_kind: crate::convert_case_kind::ConvertCaseKind,
) -> crate::case_string::CaseString
where
    S: AsRef<str>,
{
    crate::case_string::CaseString::try_from(convert_case::Casing::to_case(
        &s.as_ref(),
        convert_case_kind.get(),
    ))
    .unwrap_or_else(crate::case_string::CaseString::from)
}
