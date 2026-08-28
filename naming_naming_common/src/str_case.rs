pub(super) fn str_case<S>(v: S, case: super::ConvertCaseKind) -> super::CaseString
where
    S: AsRef<str>,
{
    super::CaseString::try_from(convert_case::Casing::to_case(&v.as_ref(), case.0))
        .unwrap_or_else(super::CaseString::from)
}
