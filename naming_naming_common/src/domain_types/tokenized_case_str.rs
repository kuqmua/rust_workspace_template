pub(super) fn tokenized_case_str<T>(v: &T, case: super::ConvertCaseKind) -> super::CaseString
where
    T: quote::ToTokens,
{
    let tokenized = quote::quote! {#v}.to_string();
    super::case_from_string(tokenized, case)
}
