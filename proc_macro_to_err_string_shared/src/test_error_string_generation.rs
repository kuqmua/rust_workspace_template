#[test]
fn test_closure_error_string_generation_preserves_each_type_and_body() {
    let generated = crate::impl_to_err_string_with(quote::quote! {
        std::io::Error, std::fmt::Error => |value| value.to_string()
    });
    let expected = quote::quote! {
        impl crate::to_err_string::ToErrString for std::io::Error {
            fn to_err_string(&self) -> crate::error_text::ErrorText {
                let value = self;
                crate::error_text::ErrorText::try_from(value.to_string())
                    .unwrap_or_else(crate::error_text::ErrorText::from)
            }
        }
        impl crate::to_err_string::ToErrString for std::fmt::Error {
            fn to_err_string(&self) -> crate::error_text::ErrorText {
                let value = self;
                crate::error_text::ErrorText::try_from(value.to_string())
                    .unwrap_or_else(crate::error_text::ErrorText::from)
            }
        }
    };
    assert_eq!(generated.to_string(), expected.to_string());
}

#[test]
fn test_closure_error_string_generation_distinguishes_invalid_input_shapes() {
    assert!(
        [
            (
                quote::quote! { std::io::Error },
                constants_str::COMPILE_ERROR_CE_062,
            ),
            (
                quote::quote! { std::io::Error => value },
                constants_str::COMPILE_ERROR_CE_061,
            ),
            (
                quote::quote! { std::io::Error => || value },
                constants_str::COMPILE_ERROR_CE_061,
            ),
        ]
        .into_iter()
        .all(|(token_stream, diagnostic)| {
            let generated = crate::impl_to_err_string_with(token_stream);
            let expected =
                workspace_macro_helpers::compile_error_token_stream::compile_error_token_stream(
                    diagnostic,
                );
            generated.to_string() == expected.into_inner().to_string()
        })
    );
}

#[test]
fn test_constant_error_string_generation_preserves_messages_and_skips_empty_parts() {
    let generated = crate::impl_to_err_string_const(quote::quote! {
        , std::io::Error => constants_str::COMPILE_ERROR_CE_060,
        , std::fmt::Error => constants_str::COMPILE_ERROR_CE_061,
    });
    let expected = quote::quote! {
        impl crate::to_err_string::ToErrString for std::io::Error {
            fn to_err_string(&self) -> crate::error_text::ErrorText {
                crate::static_str_to_owned::static_str_to_owned(
                    crate::static_str_to_owned_input::StaticStrToOwnedInput::from(constants_str::COMPILE_ERROR_CE_060),
                )
            }
        }
        impl crate::to_err_string::ToErrString for std::fmt::Error {
            fn to_err_string(&self) -> crate::error_text::ErrorText {
                crate::static_str_to_owned::static_str_to_owned(
                    crate::static_str_to_owned_input::StaticStrToOwnedInput::from(constants_str::COMPILE_ERROR_CE_061),
                )
            }
        }
    };
    assert_eq!(generated.to_string(), expected.to_string());
}

#[test]
fn test_constant_error_string_generation_rejects_later_invalid_pair_atomically() {
    let generated = crate::impl_to_err_string_const(quote::quote! {
        std::io::Error => constants_str::COMPILE_ERROR_CE_060, std::fmt::Error
    });
    let expected = workspace_macro_helpers::compile_error_token_stream::compile_error_token_stream(
        constants_str::COMPILE_ERROR_CE_060,
    );
    assert_eq!(generated.to_string(), expected.into_inner().to_string());
}

#[test]
fn test_borrowed_error_string_generation_preserves_types_and_skips_empty_parts() {
    let generated = crate::impl_to_err_string_as_ref_str(quote::quote! {
        , std::io::Error, , std::fmt::Error,
    });
    let expected = quote::quote! {
        impl crate::to_err_string::ToErrString for std::io::Error {
            fn to_err_string(&self) -> crate::error_text::ErrorText {
                crate::as_ref_str_to_owned::as_ref_str_to_owned(self)
            }
        }
        impl crate::to_err_string::ToErrString for std::fmt::Error {
            fn to_err_string(&self) -> crate::error_text::ErrorText {
                crate::as_ref_str_to_owned::as_ref_str_to_owned(self)
            }
        }
    };
    assert_eq!(generated.to_string(), expected.to_string());
}

#[test]
fn test_error_string_generation_preserves_empty_input_behavior() {
    assert!(crate::impl_to_err_string_const(quote::quote! { , , }).is_empty());
    assert!(crate::impl_to_err_string_as_ref_str(quote::quote! { , , }).is_empty());
    assert!(
        crate::impl_to_err_string_with(quote::quote! { => |value| value.to_string() }).is_empty()
    );
}
