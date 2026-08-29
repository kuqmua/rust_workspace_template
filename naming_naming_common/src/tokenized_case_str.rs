pub(super) fn tokenized_case_str<T>(
    v: &T,
    case: crate::convert_case_kind::ConvertCaseKind,
) -> crate::case_string::CaseString
where
    T: quote::ToTokens,
{
    let tokenized = quote::quote! {#v}.to_string();
    crate::case_from_string::case_from_string(tokenized, case)
}
