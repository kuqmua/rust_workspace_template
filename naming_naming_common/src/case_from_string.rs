pub(super) fn case_from_string<S>(
    s: S,
    convert_case_kind: crate::convert_case_kind::ConvertCaseKind,
) -> Result<crate::case_string::CaseString, crate::case_string::CaseStringTryFromStringError>
where
    S: AsRef<str>,
{
    crate::str_case::str_case(s, convert_case_kind)
}
