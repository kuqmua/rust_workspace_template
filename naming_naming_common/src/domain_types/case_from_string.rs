pub(super) fn case_from_string<S>(v: S, case: super::ConvertCaseKind) -> super::CaseString
where
    S: AsRef<str>,
{
    super::str_case(v, case)
}
