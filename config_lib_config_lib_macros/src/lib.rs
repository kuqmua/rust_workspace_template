pub(crate) mod proc_macro2_try_from_parse_fixed_error_ty;
pub(crate) mod proc_macro2_try_from_parse_input;
pub(crate) mod proc_macro_try_from_parse_token_stream;

#[proc_macro]
pub fn impl_try_from_non_empty_string(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let parts = workspace_macro_helpers::split_top_level_commas::split_top_level_commas(
        workspace_macro_helpers::proc_macro2_macro_tokens::ProcMacro2MacroTokens::from_into(input),
    );
    if parts.len() != 2 {
        return workspace_macro_helpers::compile_error_token_stream::compile_error_token_stream(
            constants_str::catalog::COMPILE_ERROR_CE_065,
        )
        .into_inner()
        .into();
    }
    let Some(name_text) =
        workspace_macro_helpers::first_identifier_at::first_identifier_at(&parts, 0)
    else {
        return workspace_macro_helpers::compile_error_token_stream::compile_error_token_stream(
            constants_str::catalog::COMPILE_ERROR_CE_064,
        )
        .into_inner()
        .into();
    };
    let Some(error_name_text) =
        workspace_macro_helpers::first_identifier_at::first_identifier_at(&parts, 1)
    else {
        return workspace_macro_helpers::compile_error_token_stream::compile_error_token_stream(
            constants_str::catalog::COMPILE_ERROR_CE_063,
        )
        .into_inner()
        .into();
    };
    let name = quote::format_ident!("{name_text}");
    let error_name = quote::format_ident!("{error_name_text}");
    quote::quote! {
        #[derive(Debug, Clone, generate_accessor_traits_for_struct_fields::GenerateAccessorTrait, optimal_memory_layout::OptimalMemoryLayout)]
        pub struct #name(pub String);
        #[derive(Debug, Clone, Copy, thiserror::Error, optimal_memory_layout::OptimalMemoryLayout)]
        pub enum #error_name {
            #[error("{is_empty:?}")]
            IsEmpty { is_empty: &'static str },
        }
        impl crate::try_from_std_env_var_ok::TryFromStdEnvVarOk for #name {
            type Error = #error_name;
            fn try_from_std_env_var_ok(v: crate::std_env_var_ok::StdEnvVarOk) -> Result<Self, Self::Error> {
                crate::try_map_non_empty_env_value::try_map_non_empty_env_value(
                    v,
                    |is_empty| Self::Error::IsEmpty { is_empty },
                    Self,
                )
            }
        }
    }
    .into()
}
#[proc_macro]
pub fn impl_try_from_secret_url(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let parts = workspace_macro_helpers::split_top_level_commas::split_top_level_commas(
        workspace_macro_helpers::proc_macro2_macro_tokens::ProcMacro2MacroTokens::from_into(input),
    );
    if parts.len() != 2 {
        return workspace_macro_helpers::compile_error_token_stream::compile_error_token_stream(
            constants_str::catalog::COMPILE_ERROR_CE_074,
        )
        .into_inner()
        .into();
    }
    let Some(name_text) =
        workspace_macro_helpers::first_identifier_at::first_identifier_at(&parts, 0)
    else {
        return workspace_macro_helpers::compile_error_token_stream::compile_error_token_stream(
            constants_str::catalog::COMPILE_ERROR_CE_073,
        )
        .into_inner()
        .into();
    };
    let Some(error_name_text) =
        workspace_macro_helpers::first_identifier_at::first_identifier_at(&parts, 1)
    else {
        return workspace_macro_helpers::compile_error_token_stream::compile_error_token_stream(
            constants_str::catalog::COMPILE_ERROR_CE_072,
        )
        .into_inner()
        .into();
    };
    let name = quote::format_ident!("{name_text}");
    let error_name = quote::format_ident!("{error_name_text}");
    quote::quote! {
        #[derive(Debug, generate_accessor_traits_for_struct_fields::GenerateAccessorTrait, optimal_memory_layout::OptimalMemoryLayout)]
        pub struct #name(pub secrecy::SecretBox<crate::std_config_secret_string::StdConfigSecretString>);
        #[derive(Debug, Clone, Copy, thiserror::Error, optimal_memory_layout::OptimalMemoryLayout)]
        pub enum #error_name {
            #[error("{is_empty:?}")]
            IsEmpty { is_empty: &'static str },
            #[error("secret configuration value is too long")]
            TooLong,
        }
        impl crate::try_from_std_env_var_ok::TryFromStdEnvVarOk for #name {
            type Error = #error_name;
            fn try_from_std_env_var_ok(v: crate::std_env_var_ok::StdEnvVarOk) -> Result<Self, Self::Error> {
                if v.is_empty() {
                    return Err(Self::Error::IsEmpty {
                        is_empty: constants_str::catalog::CONFIG_ENV_VALUE_IS_EMPTY_MSG,
                    });
                }
                crate::std_config_secret_string::StdConfigSecretString::try_from(String::from(v))
                    .map(|bounded| Self(secrecy::SecretBox::new(Box::new(bounded))))
                    .map_err(|_error| Self::Error::TooLong)
            }
        }
    }
    .into()
}
#[proc_macro]
pub fn impl_try_from_parse(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro::TokenStream::from(impl_try_from_parse_with_error_ty(
        proc_macro2_try_from_parse_input::ProcMacro2TryFromParseInput::from(
            proc_macro2::TokenStream::from(input),
        ),
        proc_macro2_try_from_parse_fixed_error_ty::ProcMacro2TryFromParseFixedErrorTy::from(None),
    ))
}
#[proc_macro]
pub fn impl_try_from_parse_string_error(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro::TokenStream::from(impl_try_from_parse_with_error_ty(
        proc_macro2_try_from_parse_input::ProcMacro2TryFromParseInput::from(
            proc_macro2::TokenStream::from(input),
        ),
        proc_macro2_try_from_parse_fixed_error_ty::ProcMacro2TryFromParseFixedErrorTy::from(Some(
            quote::quote! {String},
        )),
    ))
}
fn impl_try_from_parse_with_error_ty(
    input: proc_macro2_try_from_parse_input::ProcMacro2TryFromParseInput,
    fixed_error_ty: proc_macro2_try_from_parse_fixed_error_ty::ProcMacro2TryFromParseFixedErrorTy,
) -> proc_macro_try_from_parse_token_stream::ProcMacroTryFromParseTokenStream {
    let parts = workspace_macro_helpers::split_top_level_commas::split_top_level_commas(
        proc_macro2::TokenStream::from(input),
    );
    let fixed_error_ty_opt = Option::<proc_macro2::TokenStream>::from(fixed_error_ty);
    let min_len = if fixed_error_ty_opt.is_some() { 5 } else { 6 };
    if parts.len() < min_len {
        return proc_macro_try_from_parse_token_stream::ProcMacroTryFromParseTokenStream::from(
            proc_macro::TokenStream::from(
                workspace_macro_helpers::compile_error_token_stream::compile_error_token_stream(
                    constants_str::catalog::COMPILE_ERROR_CE_071,
                )
                .into_inner(),
            ),
        );
    }
    let Some(name_text) =
        workspace_macro_helpers::first_identifier_at::first_identifier_at(&parts, 0)
    else {
        return proc_macro_try_from_parse_token_stream::ProcMacroTryFromParseTokenStream::from(
            proc_macro::TokenStream::from(
                workspace_macro_helpers::compile_error_token_stream::compile_error_token_stream(
                    constants_str::catalog::COMPILE_ERROR_CE_070,
                )
                .into_inner(),
            ),
        );
    };
    let Some(error_name_text) =
        workspace_macro_helpers::first_identifier_at::first_identifier_at(&parts, 1)
    else {
        return proc_macro_try_from_parse_token_stream::ProcMacroTryFromParseTokenStream::from(
            proc_macro::TokenStream::from(
                workspace_macro_helpers::compile_error_token_stream::compile_error_token_stream(
                    constants_str::catalog::COMPILE_ERROR_CE_067,
                )
                .into_inner(),
            ),
        );
    };
    let Some(error_variant_text) =
        workspace_macro_helpers::first_identifier_at::first_identifier_at(&parts, 3)
    else {
        return proc_macro_try_from_parse_token_stream::ProcMacroTryFromParseTokenStream::from(
            proc_macro::TokenStream::from(
                workspace_macro_helpers::compile_error_token_stream::compile_error_token_stream(
                    constants_str::catalog::COMPILE_ERROR_CE_068,
                )
                .into_inner(),
            ),
        );
    };
    let Some(error_field_text) =
        workspace_macro_helpers::first_identifier_at::first_identifier_at(&parts, 4)
    else {
        return proc_macro_try_from_parse_token_stream::ProcMacroTryFromParseTokenStream::from(
            proc_macro::TokenStream::from(
                workspace_macro_helpers::compile_error_token_stream::compile_error_token_stream(
                    constants_str::catalog::COMPILE_ERROR_CE_066,
                )
                .into_inner(),
            ),
        );
    };
    let name = quote::format_ident!("{name_text}");
    let error_name = quote::format_ident!("{error_name_text}");
    let error_variant = quote::format_ident!("{error_variant_text}");
    let error_field = quote::format_ident!("{error_field_text}");
    let Some(inner) = workspace_macro_helpers::part_at::part_at(&parts, 2) else {
        return proc_macro_try_from_parse_token_stream::ProcMacroTryFromParseTokenStream::from(
            proc_macro::TokenStream::from(
                workspace_macro_helpers::compile_error_token_stream::compile_error_token_stream(
                    constants_str::catalog::COMPILE_ERROR_CE_069,
                )
                .into_inner(),
            ),
        );
    };
    let (error_ty, derives) = fixed_error_ty_opt.map_or_else(
        || {
            let Some(error_ty) = workspace_macro_helpers::part_at::part_at(&parts, 5) else {
                return (proc_macro2::TokenStream::new(), Vec::new());
            };
            let derives = parts.get(6..).unwrap_or(&[]).to_vec();
            (error_ty.into(), derives)
        },
        |fixed_error_ty_value| {
            (
                fixed_error_ty_value,
                vec![quote::quote! {Clone}, quote::quote! {Copy}],
            )
        },
    );
    proc_macro_try_from_parse_token_stream::ProcMacroTryFromParseTokenStream::from(
        proc_macro::TokenStream::from(quote::quote! {
            #[derive(Debug, #(#derives,)* generate_accessor_traits_for_struct_fields::GenerateAccessorTrait, optimal_memory_layout::OptimalMemoryLayout)]
            pub struct #name(pub #inner);
            #[derive(Debug, thiserror::Error, optimal_memory_layout::OptimalMemoryLayout)]
            pub enum #error_name {
                #[error("{:?}", .#error_field)]
                #error_variant { #error_field: #error_ty },
            }
            impl crate::try_from_std_env_var_ok::TryFromStdEnvVarOk for #name {
                type Error = #error_name;
                fn try_from_std_env_var_ok(v: crate::std_env_var_ok::StdEnvVarOk) -> Result<Self, Self::Error> {
                    crate::parse_from_str_with_error::parse_from_str_with_error(
                        crate::std_env_var_ok_ref::StdEnvVarOkRef::from(v.as_str()),
                        |#error_field| Self::Error::#error_variant { #error_field },
                    )
                    .map(Self)
                }
            }
        }),
    )
}
#[proc_macro]
pub fn assert_parse_ok_matches(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let parts = workspace_macro_helpers::split_top_level_commas::split_top_level_commas(
        workspace_macro_helpers::proc_macro2_macro_tokens::ProcMacro2MacroTokens::from_into(input),
    );
    if parts.len() != 3 {
        return workspace_macro_helpers::compile_error_token_stream::compile_error_token_stream(
            constants_str::catalog::COMPILE_ERROR_CE_040,
        )
        .into_inner()
        .into();
    }
    let Some(ty) = workspace_macro_helpers::part_at::part_at(&parts, 0) else {
        return workspace_macro_helpers::compile_error_token_stream::compile_error_token_stream(
            constants_str::catalog::COMPILE_ERROR_CE_039,
        )
        .into_inner()
        .into();
    };
    let Some(value) = workspace_macro_helpers::part_at::part_at(&parts, 1) else {
        return workspace_macro_helpers::compile_error_token_stream::compile_error_token_stream(
            constants_str::catalog::COMPILE_ERROR_CE_041,
        )
        .into_inner()
        .into();
    };
    let Some(pattern) = workspace_macro_helpers::part_at::part_at(&parts, 2) else {
        return workspace_macro_helpers::compile_error_token_stream::compile_error_token_stream(
            constants_str::catalog::COMPILE_ERROR_CE_038,
        )
        .into_inner()
        .into();
    };
    quote::quote! {
        assert!(matches!(parse_env::<#ty>(#value), Ok(#pattern)));
    }
    .into()
}
#[proc_macro]
pub fn assert_parse_err_matches(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let parts = workspace_macro_helpers::split_top_level_commas::split_top_level_commas(
        workspace_macro_helpers::proc_macro2_macro_tokens::ProcMacro2MacroTokens::from_into(input),
    );
    if parts.len() != 3 {
        return workspace_macro_helpers::compile_error_token_stream::compile_error_token_stream(
            constants_str::catalog::COMPILE_ERROR_CE_036,
        )
        .into_inner()
        .into();
    }
    let Some(ty) = workspace_macro_helpers::part_at::part_at(&parts, 0) else {
        return workspace_macro_helpers::compile_error_token_stream::compile_error_token_stream(
            constants_str::catalog::COMPILE_ERROR_CE_035,
        )
        .into_inner()
        .into();
    };
    let Some(value) = workspace_macro_helpers::part_at::part_at(&parts, 1) else {
        return workspace_macro_helpers::compile_error_token_stream::compile_error_token_stream(
            constants_str::catalog::COMPILE_ERROR_CE_037,
        )
        .into_inner()
        .into();
    };
    let Some(pattern) = workspace_macro_helpers::part_at::part_at(&parts, 2) else {
        return workspace_macro_helpers::compile_error_token_stream::compile_error_token_stream(
            constants_str::catalog::COMPILE_ERROR_CE_034,
        )
        .into_inner()
        .into();
    };
    quote::quote! {
        assert!(matches!(parse_env::<#ty>(#value), Err(#pattern)));
    }
    .into()
}
#[proc_macro]
pub fn assert_empty_parse_err_matches(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let parts = workspace_macro_helpers::split_top_level_commas::split_top_level_commas(
        workspace_macro_helpers::proc_macro2_macro_tokens::ProcMacro2MacroTokens::from_into(input),
    );
    if parts.len() != 2 {
        return workspace_macro_helpers::compile_error_token_stream::compile_error_token_stream(
            constants_str::catalog::COMPILE_ERROR_CE_033,
        )
        .into_inner()
        .into();
    }
    let Some(ty) = workspace_macro_helpers::part_at::part_at(&parts, 0) else {
        return workspace_macro_helpers::compile_error_token_stream::compile_error_token_stream(
            constants_str::catalog::COMPILE_ERROR_CE_032,
        )
        .into_inner()
        .into();
    };
    let Some(pattern) = workspace_macro_helpers::part_at::part_at(&parts, 1) else {
        return workspace_macro_helpers::compile_error_token_stream::compile_error_token_stream(
            constants_str::catalog::COMPILE_ERROR_CE_031,
        )
        .into_inner()
        .into();
    };
    quote::quote! {
        assert!(matches!(parse_env::<#ty>(""), Err(#pattern)));
    }
    .into()
}
