pub(super) fn case_from_string<S>(
    v: S,
    case: crate::convert_case_kind::ConvertCaseKind,
) -> crate::case_string::CaseString
where
    S: AsRef<str>,
{
    crate::str_case::str_case(v, case)
}
