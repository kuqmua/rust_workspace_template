pub(super) fn tokenized_case_str<T>(
    t: &T,
    convert_case_kind: crate::convert_case_kind::ConvertCaseKind,
) -> crate::case_string::CaseString
where
    T: quote::ToTokens,
{
    let tokenized = quote::quote! {#t}.to_string();
    crate::case_from_string::case_from_string(tokenized, convert_case_kind)
}
