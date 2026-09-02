pub(super) fn display_case_str<T>(
    t: &T,
    convert_case_kind: crate::convert_case_kind::ConvertCaseKind,
) -> crate::case_string::CaseString
where
    T: std::fmt::Display,
{
    let stringified = t.to_string();
    crate::case_from_string::case_from_string(stringified, convert_case_kind)
}
