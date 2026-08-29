pub(super) fn display_case_str<T>(
    v: &T,
    case: crate::convert_case_kind::ConvertCaseKind,
) -> crate::case_string::CaseString
where
    T: std::fmt::Display,
{
    let stringified = v.to_string();
    crate::case_from_string::case_from_string(stringified, case)
}
