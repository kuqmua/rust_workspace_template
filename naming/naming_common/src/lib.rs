const CASE_STRING_MAX_LEN: usize = 1_048_576;
naming_common_macros::case_trait_pair!(
    AsRefStrToUpperCamelCaseStr,
    AsRefStrToUpperCamelCaseTokenStream,
    AsRef<str>,
    |self_ref| {
        str_case(
            self_ref.as_ref(),
            ConvertCaseKind(convert_case::Case::UpperCamel),
        )
        .0
    }
);
naming_common_macros::case_trait_pair!(
    AsRefStrToSnakeCaseStr,
    AsRefStrToSnakeCaseTokenStream,
    AsRef<str>,
    |self_ref| {
        str_case(
            self_ref.as_ref(),
            ConvertCaseKind(convert_case::Case::Snake),
        )
        .0
    }
);
naming_common_macros::case_trait_pair!(
    AsRefStrToUpperSnakeCaseStr,
    AsRefStrToUpperSnakeCaseTokenStream,
    AsRef<str>,
    |self_ref| str_case(
        self_ref.as_ref(),
        ConvertCaseKind(convert_case::Case::UpperSnake)
    )
    .0
);
naming_common_macros::case_trait_pair!(
    DisplayToUpperCamelCaseStr,
    DisplayToUpperCamelCaseTokenStream,
    std::fmt::Display,
    |self_ref| { display_case_str(self_ref, ConvertCaseKind(convert_case::Case::UpperCamel)).0 }
);
naming_common_macros::case_trait_pair!(
    DisplayToSnakeCaseStr,
    DisplayToSnakeCaseTokenStream,
    std::fmt::Display,
    |self_ref| { display_case_str(self_ref, ConvertCaseKind(convert_case::Case::Snake)).0 }
);
naming_common_macros::case_trait_pair!(
    DisplayToUpperSnakeCaseStr,
    DisplayToUpperSnakeCaseTokenStream,
    std::fmt::Display,
    |self_ref| display_case_str(self_ref, ConvertCaseKind(convert_case::Case::UpperSnake)).0
);
naming_common_macros::case_trait_pair!(
    ToTokensToUpperCamelCaseStr,
    ToTokensToUpperCamelCaseTokenStream,
    quote::ToTokens,
    |self_ref| { tokenized_case_str(self_ref, ConvertCaseKind(convert_case::Case::UpperCamel)).0 }
);
naming_common_macros::case_trait_pair!(
    ToTokensToSnakeCaseStr,
    ToTokensToSnakeCaseTokenStream,
    quote::ToTokens,
    |self_ref| { tokenized_case_str(self_ref, ConvertCaseKind(convert_case::Case::Snake)).0 }
);
naming_common_macros::case_trait_pair!(
    ToTokensToUpperSnakeCaseStr,
    ToTokensToUpperSnakeCaseTokenStream,
    quote::ToTokens,
    |self_ref| tokenized_case_str(self_ref, ConvertCaseKind(convert_case::Case::UpperSnake)).0
);
#[derive(Debug, Clone, Copy)]
#[derive(newtype::FromInner)]
struct ConvertCaseKind(convert_case::Case<'static>);
#[derive(
    Debug, Clone, PartialEq, Eq, newtype::BoundedString, newtype::AsRefStr, newtype::Display,
)]
#[bounded_string(max = CASE_STRING_MAX_LEN )]
struct CaseString(String);
#[derive(Debug, Clone)]
#[derive(newtype::FromInner)]
struct ProcMacro2CaseTokenStream(proc_macro2::TokenStream);
fn to_token_stream_or_panic<T>(v: &T) -> ProcMacro2CaseTokenStream
where
    T: std::fmt::Display + ?Sized,
{
    ProcMacro2CaseTokenStream::from(match v.to_string().parse::<proc_macro2::TokenStream>() {
        Ok(parsed_token_stream) => parsed_token_stream,
        Err(error) => {
            let message = error.to_string();
            quote::quote! {compile_error!(#message);}
        }
    })
}
fn case_from_string<S>(v: S, case: ConvertCaseKind) -> CaseString
where
    S: AsRef<str>,
{
    str_case(v, case)
}
fn display_case_str<T>(v: &T, case: ConvertCaseKind) -> CaseString
where
    T: std::fmt::Display,
{
    let stringified = v.to_string();
    case_from_string(stringified, case)
}
fn tokenized_case_str<T>(v: &T, case: ConvertCaseKind) -> CaseString
where
    T: quote::ToTokens,
{
    let tokenized = quote::quote! {#v}.to_string();
    case_from_string(tokenized, case)
}
fn str_case<S>(v: S, case: ConvertCaseKind) -> CaseString
where
    S: AsRef<str>,
{
    CaseString::try_from(convert_case::Casing::to_case(&v.as_ref(), case.0))
        .unwrap_or_else(CaseString::from)
}
#[cfg(test)]
mod tests {
    fn assert_case_triplet<S>(to_upper_camel_case: S, to_snake_case: S, to_upper_snake_case: S)
    where
        S: AsRef<str>,
    {
        assert_eq!(to_upper_camel_case.as_ref(), "HelloWorld");
        assert_eq!(to_snake_case.as_ref(), "hello_world");
        assert_eq!(to_upper_snake_case.as_ref(), "HELLO_WORLD");
    }
    #[test]
    fn as_ref_case_conversions_are_expected() {
        assert_case_triplet(
            super::AsRefStrToUpperCamelCaseStr::case(&str_constants::HELLO_WORLD_ALT),
            super::AsRefStrToSnakeCaseStr::case(&str_constants::HELLOWORLD),
            super::AsRefStrToUpperSnakeCaseStr::case(&str_constants::HELLOWORLD_ALT),
        );
    }
    #[test]
    fn ts_case_conversions_are_expected() {
        assert_case_triplet(
            super::AsRefStrToUpperCamelCaseTokenStream::case_or_panic(
                &str_constants::HELLO_WORLD_ALT,
            )
            .to_string(),
            super::AsRefStrToSnakeCaseTokenStream::case_or_panic(&str_constants::HELLOWORLD)
                .to_string(),
            super::AsRefStrToUpperSnakeCaseTokenStream::case_or_panic(
                &str_constants::HELLOWORLD_ALT,
            )
            .to_string(),
        );
    }
    #[test]
    fn display_and_tokens_conversion_are_expected() {
        assert_case_triplet(
            super::DisplayToUpperCamelCaseStr::case(&str_constants::HELLO_WORLD_ALT),
            super::DisplayToSnakeCaseStr::case(&str_constants::HELLOWORLD),
            super::DisplayToUpperSnakeCaseStr::case(&str_constants::HELLOWORLD_ALT),
        );
        assert_case_triplet(
            super::ToTokensToUpperCamelCaseStr::case(&quote::quote! {hello_world}),
            super::ToTokensToSnakeCaseStr::case(&quote::quote! {HelloWorld}),
            super::ToTokensToUpperSnakeCaseStr::case(&quote::quote! {helloWorld}),
        );
    }
    #[test]
    fn display_and_tokens_token_stream_conversion_are_expected() {
        assert_case_triplet(
            super::DisplayToUpperCamelCaseTokenStream::case_or_panic(
                &str_constants::HELLO_WORLD_ALT,
            )
            .to_string(),
            super::DisplayToSnakeCaseTokenStream::case_or_panic(&str_constants::HELLOWORLD)
                .to_string(),
            super::DisplayToUpperSnakeCaseTokenStream::case_or_panic(
                &str_constants::HELLOWORLD_ALT,
            )
            .to_string(),
        );
        assert_case_triplet(
            super::ToTokensToUpperCamelCaseTokenStream::case_or_panic(&quote::quote! {hello_world})
                .to_string(),
            super::ToTokensToSnakeCaseTokenStream::case_or_panic(&quote::quote! {HelloWorld})
                .to_string(),
            super::ToTokensToUpperSnakeCaseTokenStream::case_or_panic(&quote::quote! {helloWorld})
                .to_string(),
        );
    }
}
