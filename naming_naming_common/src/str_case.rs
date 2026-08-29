pub(super) fn str_case<S>(
    v: S,
    case: crate::convert_case_kind::ConvertCaseKind,
) -> crate::case_string::CaseString
where
    S: AsRef<str>,
{
    crate::case_string::CaseString::try_from(convert_case::Casing::to_case(&v.as_ref(), case.0))
        .unwrap_or_else(crate::case_string::CaseString::from)
}
