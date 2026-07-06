use convert_case::{Case, Casing as _};
use naming_cmn_macros::case_trait_pair;
use proc_macro2::TokenStream as Ts2;
use quote::{ToTokens, quote};
use std::fmt::Display;
case_trait_pair!(AsRefStrToUccStr, AsRefStrToUccTs, AsRef<str>, |self_ref| {
    str_case(self_ref.as_ref(), Case::UpperCamel)
});
case_trait_pair!(AsRefStrToScStr, AsRefStrToScTs, AsRef<str>, |self_ref| {
    str_case(self_ref.as_ref(), Case::Snake)
});
case_trait_pair!(
    AsRefStrToUpperScStr,
    AsRefStrToUpperScTs,
    AsRef<str>,
    |self_ref| str_case(self_ref.as_ref(), Case::UpperSnake)
);
case_trait_pair!(DisplayToUccStr, DisplayToUccTs, Display, |self_ref| {
    display_case_str(self_ref, Case::UpperCamel)
});
case_trait_pair!(DisplayToScStr, DisplayToScTs, Display, |self_ref| {
    display_case_str(self_ref, Case::Snake)
});
case_trait_pair!(
    DisplayToUpperScStr,
    DisplayToUpperScTs,
    Display,
    |self_ref| display_case_str(self_ref, Case::UpperSnake)
);
case_trait_pair!(ToTokensToUccStr, ToTokensToUccTs, ToTokens, |self_ref| {
    tokenized_case_str(self_ref, Case::UpperCamel)
});
case_trait_pair!(ToTokensToScStr, ToTokensToScTs, ToTokens, |self_ref| {
    tokenized_case_str(self_ref, Case::Snake)
});
case_trait_pair!(
    ToTokensToUpperScStr,
    ToTokensToUpperScTs,
    ToTokens,
    |self_ref| tokenized_case_str(self_ref, Case::UpperSnake)
);
fn to_ts_or_panic<T>(v: &T) -> Ts2
where
    T: Display + ?Sized,
{
    match v.to_string().parse::<Ts2>() {
        Ok(parsed_ts) => parsed_ts,
        Err(er) => {
            let msg = er.to_string();
            quote! {compile_error!(#msg);}
        }
    }
}
fn case_from_string(v: &str, case: Case<'_>) -> String {
    str_case(v, case)
}
fn display_case_str<T>(v: &T, case: Case<'_>) -> String
where
    T: Display,
{
    let stringified = v.to_string();
    case_from_string(&stringified, case)
}
fn tokenized_case_str<T>(v: &T, case: Case<'_>) -> String
where
    T: ToTokens,
{
    let tokenized = quote! {#v}.to_string();
    case_from_string(&tokenized, case)
}
fn str_case(v: &str, case: Case<'_>) -> String {
    v.to_case(case)
}
#[cfg(test)]
mod tests {
    use super::{
        AsRefStrToScStr, AsRefStrToScTs, AsRefStrToUccStr, AsRefStrToUccTs, AsRefStrToUpperScStr,
        AsRefStrToUpperScTs, DisplayToScStr, DisplayToScTs, DisplayToUccStr, DisplayToUccTs,
        DisplayToUpperScStr, DisplayToUpperScTs, ToTokensToScStr, ToTokensToScTs, ToTokensToUccStr,
        ToTokensToUccTs, ToTokensToUpperScStr, ToTokensToUpperScTs,
    };
    use quote::quote;
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
            AsRefStrToUccStr::case(&"hello_world"),
            AsRefStrToScStr::case(&"HelloWorld"),
            AsRefStrToUpperScStr::case(&"helloWorld"),
        );
    }
    #[test]
    fn ts_case_conversions_are_expected() {
        assert_case_triplet(
            AsRefStrToUccTs::case_or_panic(&"hello_world").to_string(),
            AsRefStrToScTs::case_or_panic(&"HelloWorld").to_string(),
            AsRefStrToUpperScTs::case_or_panic(&"helloWorld").to_string(),
        );
    }
    #[test]
    fn display_and_tokens_conversion_are_expected() {
        assert_case_triplet(
            DisplayToUccStr::case(&"hello_world"),
            DisplayToScStr::case(&"HelloWorld"),
            DisplayToUpperScStr::case(&"helloWorld"),
        );
        assert_case_triplet(
            ToTokensToUccStr::case(&quote! {hello_world}),
            ToTokensToScStr::case(&quote! {HelloWorld}),
            ToTokensToUpperScStr::case(&quote! {helloWorld}),
        );
    }
    #[test]
    fn display_and_tokens_ts_conversion_are_expected() {
        assert_case_triplet(
            DisplayToUccTs::case_or_panic(&"hello_world").to_string(),
            DisplayToScTs::case_or_panic(&"HelloWorld").to_string(),
            DisplayToUpperScTs::case_or_panic(&"helloWorld").to_string(),
        );
        assert_case_triplet(
            ToTokensToUccTs::case_or_panic(&quote! {hello_world}).to_string(),
            ToTokensToScTs::case_or_panic(&quote! {HelloWorld}).to_string(),
            ToTokensToUpperScTs::case_or_panic(&quote! {helloWorld}).to_string(),
        );
    }
}
