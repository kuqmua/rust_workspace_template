use core::fmt::Display;

macro_rules! case_trait_pair {
    (
        $string_trait:ident,
        $token_stream_trait:ident,
        $bound:path, |
        $self_reference:ident |
        $body:expr
    ) => {
        pub trait $string_trait {
            #[must_use]
            fn case(&self) -> String;
        }

        impl<T> $string_trait for T
        where
            T: $bound,
        {
            fn case(&self) -> String {
                let $self_reference = self;
                $body
            }
        }

        pub trait $token_stream_trait {
            #[must_use]
            fn case_or_panic(&self) -> proc_macro2::TokenStream;
        }

        impl<T> $token_stream_trait for T
        where
            T: $string_trait,
        {
            fn case_or_panic(&self) -> proc_macro2::TokenStream {
                to_token_stream_or_compile_error(&$string_trait::case(self))
            }
        }
    };
}

case_trait_pair!(AsRefStrToUccStr, AsRefStrToUccTs, AsRef<str>, |self_reference| {
    str_case(self_reference.as_ref(), convert_case::Case::UpperCamel)
});
case_trait_pair!(AsRefStrToScStr, AsRefStrToScTs, AsRef<str>, |self_reference| {
    str_case(self_reference.as_ref(), convert_case::Case::Snake)
});
case_trait_pair!(AsRefStrToUpperScStr, AsRefStrToUpperScTs, AsRef<str>, |self_reference| {
    str_case(self_reference.as_ref(), convert_case::Case::UpperSnake)
});
case_trait_pair!(DisplayToUccStr, DisplayToUccTs, Display, |self_reference| {
    display_case_str(self_reference, convert_case::Case::UpperCamel)
});
case_trait_pair!(DisplayToScStr, DisplayToScTs, Display, |self_reference| {
    display_case_str(self_reference, convert_case::Case::Snake)
});
case_trait_pair!(DisplayToUpperScStr, DisplayToUpperScTs, Display, |self_reference| {
    display_case_str(self_reference, convert_case::Case::UpperSnake)
});
case_trait_pair!(ToTokensToUccStr, ToTokensToUccTs, quote::ToTokens, |self_reference| {
    tokenized_case_str(self_reference, convert_case::Case::UpperCamel)
});
case_trait_pair!(ToTokensToScStr, ToTokensToScTs, quote::ToTokens, |self_reference| {
    tokenized_case_str(self_reference, convert_case::Case::Snake)
});
case_trait_pair!(ToTokensToUpperScStr, ToTokensToUpperScTs, quote::ToTokens, |self_reference| {
    tokenized_case_str(self_reference, convert_case::Case::UpperSnake)
});

fn to_token_stream_or_compile_error<DisplayValue>(value: &DisplayValue) -> proc_macro2::TokenStream
where
    DisplayValue: Display + ?Sized,
{
    match value.to_string().parse::<proc_macro2::TokenStream>() {
        Ok(token_stream) => token_stream,
        Err(parse_error) => {
            let error_message = parse_error.to_string();
            let escaped_message = format!("{error_message:?}");
            let compile_error = format!("compile_error!({escaped_message})");
            Result::unwrap_or_else(compile_error.parse::<proc_macro2::TokenStream>(), |_| {
                proc_macro2::TokenStream::new()
            })
        }
    }
}

fn case_from_string<StringValue>(value: &StringValue, case: convert_case::Case<'_>) -> String
where
    StringValue: AsRef<str> + ?Sized,
{
    str_case(value, case)
}

fn display_case_str<DisplayValue>(value: &DisplayValue, case: convert_case::Case<'_>) -> String
where
    DisplayValue: Display,
{
    let stringified = value.to_string();
    case_from_string(&stringified, case)
}

fn tokenized_case_str<TokenValue>(value: &TokenValue, case: convert_case::Case<'_>) -> String
where
    TokenValue: quote::ToTokens,
{
    let tokenized = quote::quote! {#value}.to_string();
    case_from_string(&tokenized, case)
}

fn str_case<StringValue>(value: &StringValue, case: convert_case::Case<'_>) -> String
where
    StringValue: AsRef<str> + ?Sized,
{
    convert_case::Casing::to_case(&value.as_ref(), case)
}

#[cfg(test)]
mod tests {
    fn assert_case_triplet<StringValue>(
        to_upper_camel_case: StringValue,
        to_snake_case: StringValue,
        to_upper_snake_case: StringValue,
    ) -> Result<(), String>
    where
        StringValue: AsRef<str>,
    {
        if to_upper_camel_case.as_ref() != "HelloWorld" {
            return Err(format!("{} != HelloWorld", to_upper_camel_case.as_ref()));
        }
        if to_snake_case.as_ref() != "hello_world" {
            return Err(format!("{} != hello_world", to_snake_case.as_ref()));
        }
        if to_upper_snake_case.as_ref() != "HELLO_WORLD" {
            return Err(format!("{} != HELLO_WORLD", to_upper_snake_case.as_ref()));
        }
        Ok(())
    }

    #[test]
    fn as_ref_case_conversions_are_expected() -> Result<(), String> {
        assert_case_triplet(
            crate::AsRefStrToUccStr::case(&"hello_world"),
            crate::AsRefStrToScStr::case(&"HelloWorld"),
            crate::AsRefStrToUpperScStr::case(&"helloWorld"),
        )
    }

    #[test]
    fn token_stream_case_conversions_are_expected() -> Result<(), String> {
        assert_case_triplet(
            crate::AsRefStrToUccTs::case_or_panic(&"hello_world").to_string(),
            crate::AsRefStrToScTs::case_or_panic(&"HelloWorld").to_string(),
            crate::AsRefStrToUpperScTs::case_or_panic(&"helloWorld").to_string(),
        )
    }

    #[test]
    fn display_and_tokens_conversion_are_expected() -> Result<(), String> {
        assert_case_triplet(
            crate::DisplayToUccStr::case(&"hello_world"),
            crate::DisplayToScStr::case(&"HelloWorld"),
            crate::DisplayToUpperScStr::case(&"helloWorld"),
        )?;
        assert_case_triplet(
            crate::ToTokensToUccStr::case(&quote::quote! {hello_world}),
            crate::ToTokensToScStr::case(&quote::quote! {HelloWorld}),
            crate::ToTokensToUpperScStr::case(&quote::quote! {helloWorld}),
        )
    }

    #[test]
    fn display_and_tokens_token_stream_conversion_are_expected() -> Result<(), String> {
        assert_case_triplet(
            crate::DisplayToUccTs::case_or_panic(&"hello_world").to_string(),
            crate::DisplayToScTs::case_or_panic(&"HelloWorld").to_string(),
            crate::DisplayToUpperScTs::case_or_panic(&"helloWorld").to_string(),
        )?;
        assert_case_triplet(
            crate::ToTokensToUccTs::case_or_panic(&quote::quote! {hello_world}).to_string(),
            crate::ToTokensToScTs::case_or_panic(&quote::quote! {HelloWorld}).to_string(),
            crate::ToTokensToUpperScTs::case_or_panic(&quote::quote! {helloWorld}).to_string(),
        )
    }
}
