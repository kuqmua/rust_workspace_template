pub(super) fn display_case_str<T>(v: &T, case: super::ConvertCaseKind) -> super::CaseString
where
    T: std::fmt::Display,
{
    let stringified = v.to_string();
    super::case_from_string(stringified, case)
}
