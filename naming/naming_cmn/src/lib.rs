naming_cmn_macros::case_trait_pair!(AsRefStrToUccStr, AsRefStrToUccTs, AsRef<str>, |self_ref| {
    str_case(self_ref.as_ref(), convert_case::Case::UpperCamel)
});
naming_cmn_macros::case_trait_pair!(AsRefStrToScStr, AsRefStrToScTs, AsRef<str>, |self_ref| {
    str_case(self_ref.as_ref(), convert_case::Case::Snake)
});
naming_cmn_macros::case_trait_pair!(
    AsRefStrToUpperScStr,
    AsRefStrToUpperScTs,
    AsRef<str>,
    |self_ref| str_case(self_ref.as_ref(), convert_case::Case::UpperSnake)
);
naming_cmn_macros::case_trait_pair!(
    DisplayToUccStr,
    DisplayToUccTs,
    std::fmt::Display,
    |self_ref| { display_case_str(self_ref, convert_case::Case::UpperCamel) }
);
naming_cmn_macros::case_trait_pair!(
    DisplayToScStr,
    DisplayToScTs,
    std::fmt::Display,
    |self_ref| { display_case_str(self_ref, convert_case::Case::Snake) }
);
naming_cmn_macros::case_trait_pair!(
    DisplayToUpperScStr,
    DisplayToUpperScTs,
    std::fmt::Display,
    |self_ref| display_case_str(self_ref, convert_case::Case::UpperSnake)
);
naming_cmn_macros::case_trait_pair!(
    ToTokensToUccStr,
    ToTokensToUccTs,
    quote::ToTokens,
    |self_ref| { tokenized_case_str(self_ref, convert_case::Case::UpperCamel) }
);
naming_cmn_macros::case_trait_pair!(
    ToTokensToScStr,
    ToTokensToScTs,
    quote::ToTokens,
    |self_ref| { tokenized_case_str(self_ref, convert_case::Case::Snake) }
);
naming_cmn_macros::case_trait_pair!(
    ToTokensToUpperScStr,
    ToTokensToUpperScTs,
    quote::ToTokens,
    |self_ref| tokenized_case_str(self_ref, convert_case::Case::UpperSnake)
);
fn to_ts_or_panic<T>(v: &T) -> proc_macro2::TokenStream
where
    T: std::fmt::Display + ?Sized,
{
    match v.to_string().parse::<proc_macro2::TokenStream>() {
        Ok(parsed_ts) => parsed_ts,
        Err(er) => {
            let msg = er.to_string();
            quote::quote! {compile_error!(#msg);}
        }
    }
}
fn case_from_string(v: &str, case: convert_case::Case<'_>) -> String {
    str_case(v, case)
}
fn display_case_str<T>(v: &T, case: convert_case::Case<'_>) -> String
where
    T: std::fmt::Display,
{
    let stringified = v.to_string();
    case_from_string(&stringified, case)
}
fn tokenized_case_str<T>(v: &T, case: convert_case::Case<'_>) -> String
where
    T: quote::ToTokens,
{
    let tokenized = quote::quote! {#v}.to_string();
    case_from_string(&tokenized, case)
}
fn str_case(v: &str, case: convert_case::Case<'_>) -> String {
    convert_case::Casing::to_case(&v, case)
}
#[cfg(test)]
mod tests {
    fn assert_case_triplet<S>(to_ucc: S, to_sc: S, to_upper_sc: S)
    where
        S: AsRef<str>,
    {
        assert_eq!(to_ucc.as_ref(), "HelloWorld");
        assert_eq!(to_sc.as_ref(), "hello_world");
        assert_eq!(to_upper_sc.as_ref(), "HELLO_WORLD");
    }
    #[test]
    fn as_ref_case_conversions_are_expected() {
        assert_case_triplet(
            super::AsRefStrToUccStr::case(&"hello_world"),
            super::AsRefStrToScStr::case(&"HelloWorld"),
            super::AsRefStrToUpperScStr::case(&"helloWorld"),
        );
    }
    #[test]
    fn ts_case_conversions_are_expected() {
        assert_case_triplet(
            super::AsRefStrToUccTs::case_or_panic(&"hello_world").to_string(),
            super::AsRefStrToScTs::case_or_panic(&"HelloWorld").to_string(),
            super::AsRefStrToUpperScTs::case_or_panic(&"helloWorld").to_string(),
        );
    }
    #[test]
    fn display_and_tokens_conversion_are_expected() {
        assert_case_triplet(
            super::DisplayToUccStr::case(&"hello_world"),
            super::DisplayToScStr::case(&"HelloWorld"),
            super::DisplayToUpperScStr::case(&"helloWorld"),
        );
        assert_case_triplet(
            super::ToTokensToUccStr::case(&quote::quote! {hello_world}),
            super::ToTokensToScStr::case(&quote::quote! {HelloWorld}),
            super::ToTokensToUpperScStr::case(&quote::quote! {helloWorld}),
        );
    }
    #[test]
    fn display_and_tokens_ts_conversion_are_expected() {
        assert_case_triplet(
            super::DisplayToUccTs::case_or_panic(&"hello_world").to_string(),
            super::DisplayToScTs::case_or_panic(&"HelloWorld").to_string(),
            super::DisplayToUpperScTs::case_or_panic(&"helloWorld").to_string(),
        );
        assert_case_triplet(
            super::ToTokensToUccTs::case_or_panic(&quote::quote! {hello_world}).to_string(),
            super::ToTokensToScTs::case_or_panic(&quote::quote! {HelloWorld}).to_string(),
            super::ToTokensToUpperScTs::case_or_panic(&quote::quote! {helloWorld}).to_string(),
        );
    }
}
