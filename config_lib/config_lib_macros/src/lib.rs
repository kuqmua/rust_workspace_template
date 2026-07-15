struct ProcMacro2TryFromParseInput(proc_macro2::TokenStream);
struct ProcMacro2TryFromParseFixedErrorTy(Option<proc_macro2::TokenStream>);
struct ProcMacroTryFromParseTokenStream(proc_macro::TokenStream);
#[proc_macro]
pub fn impl_try_from_non_empty_string(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let parts = workspace_macro_helpers::split_top_level_commas(
        workspace_macro_helpers::ProcMacro2MacroTokens::from_into(input),
    );
    if parts.len() != 2 {
        return workspace_macro_helpers::compile_error_token_stream(
            str_constants::compile_error::CE_065,
        )
        .into_inner()
        .into();
    }
    let Some(name_text) = workspace_macro_helpers::first_identifier_at(&parts, 0) else {
        return workspace_macro_helpers::compile_error_token_stream(
            str_constants::compile_error::CE_064,
        )
        .into_inner()
        .into();
    };
    let Some(error_name_text) = workspace_macro_helpers::first_identifier_at(&parts, 1) else {
        return workspace_macro_helpers::compile_error_token_stream(
            str_constants::compile_error::CE_063,
        )
        .into_inner()
        .into();
    };
    let name = quote::format_ident!("{name_text}");
    let error_name = quote::format_ident!("{error_name_text}");
    quote::quote! {
        #[derive(Debug, Clone, generate_getter_traits_for_struct_fields::GenerateGetterTrait, optml::Optml)]
        pub struct #name(pub String);
        #[derive(Debug, Clone, Copy, thiserror::Error, optml::Optml)]
        pub enum #error_name {
            #[error("{is_empty:?}")]
            IsEmpty { is_empty: &'static str },
        }
        impl TryFromStdEnvVarOk for #name {
            type Error = #error_name;
            fn try_from_std_env_var_ok(v: StdEnvVarOk) -> Result<Self, Self::Error> {
                try_map_non_empty_env_value(v, |is_empty| Self::Error::IsEmpty { is_empty }, Self)
            }
        }
    }
    .into()
}
#[proc_macro]
pub fn impl_try_from_secret_url(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let parts = workspace_macro_helpers::split_top_level_commas(
        workspace_macro_helpers::ProcMacro2MacroTokens::from_into(input),
    );
    if parts.len() != 2 {
        return workspace_macro_helpers::compile_error_token_stream(
            str_constants::compile_error::CE_074,
        )
        .into_inner()
        .into();
    }
    let Some(name_text) = workspace_macro_helpers::first_identifier_at(&parts, 0) else {
        return workspace_macro_helpers::compile_error_token_stream(
            str_constants::compile_error::CE_073,
        )
        .into_inner()
        .into();
    };
    let Some(error_name_text) = workspace_macro_helpers::first_identifier_at(&parts, 1) else {
        return workspace_macro_helpers::compile_error_token_stream(
            str_constants::compile_error::CE_072,
        )
        .into_inner()
        .into();
    };
    let name = quote::format_ident!("{name_text}");
    let error_name = quote::format_ident!("{error_name_text}");
    quote::quote! {
        #[derive(Debug, generate_getter_traits_for_struct_fields::GenerateGetterTrait, optml::Optml)]
        pub struct #name(pub secrecy::SecretBox<String>);
        #[derive(Debug, Clone, Copy, thiserror::Error, optml::Optml)]
        pub enum #error_name {
            #[error("{is_empty:?}")]
            IsEmpty { is_empty: &'static str },
        }
        impl TryFromStdEnvVarOk for #name {
            type Error = #error_name;
            fn try_from_std_env_var_ok(v: StdEnvVarOk) -> Result<Self, Self::Error> {
                try_map_non_empty_env_value(
                    v,
                    |is_empty| Self::Error::IsEmpty { is_empty },
                    |v| Self(secrecy::SecretBox::new(Box::new(v))),
                )
            }
        }
    }
    .into()
}
#[proc_macro]
pub fn impl_try_from_parse(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    impl_try_from_parse_with_error_ty(
        ProcMacro2TryFromParseInput(input.into()),
        ProcMacro2TryFromParseFixedErrorTy(None),
    )
    .0
}
#[proc_macro]
pub fn impl_try_from_parse_string_error(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    impl_try_from_parse_with_error_ty(
        ProcMacro2TryFromParseInput(input.into()),
        ProcMacro2TryFromParseFixedErrorTy(Some(quote::quote! {String})),
    )
    .0
}
fn impl_try_from_parse_with_error_ty(
    input: ProcMacro2TryFromParseInput,
    fixed_error_ty: ProcMacro2TryFromParseFixedErrorTy,
) -> ProcMacroTryFromParseTokenStream {
    let parts = workspace_macro_helpers::split_top_level_commas(input.0);
    let min_len = if fixed_error_ty.0.is_some() { 5 } else { 6 };
    if parts.len() < min_len {
        return ProcMacroTryFromParseTokenStream(
            workspace_macro_helpers::compile_error_token_stream(
                str_constants::compile_error::CE_071,
            )
            .into_inner()
            .into(),
        );
    }
    let Some(name_text) = workspace_macro_helpers::first_identifier_at(&parts, 0) else {
        return ProcMacroTryFromParseTokenStream(
            workspace_macro_helpers::compile_error_token_stream(
                str_constants::compile_error::CE_070,
            )
            .into_inner()
            .into(),
        );
    };
    let Some(error_name_text) = workspace_macro_helpers::first_identifier_at(&parts, 1) else {
        return ProcMacroTryFromParseTokenStream(
            workspace_macro_helpers::compile_error_token_stream(
                str_constants::compile_error::CE_067,
            )
            .into_inner()
            .into(),
        );
    };
    let Some(error_variant_text) = workspace_macro_helpers::first_identifier_at(&parts, 3) else {
        return ProcMacroTryFromParseTokenStream(
            workspace_macro_helpers::compile_error_token_stream(
                str_constants::compile_error::CE_068,
            )
            .into_inner()
            .into(),
        );
    };
    let Some(error_field_text) = workspace_macro_helpers::first_identifier_at(&parts, 4) else {
        return ProcMacroTryFromParseTokenStream(
            workspace_macro_helpers::compile_error_token_stream(
                str_constants::compile_error::CE_066,
            )
            .into_inner()
            .into(),
        );
    };
    let name = quote::format_ident!("{name_text}");
    let error_name = quote::format_ident!("{error_name_text}");
    let error_variant = quote::format_ident!("{error_variant_text}");
    let error_field = quote::format_ident!("{error_field_text}");
    let Some(inner) = workspace_macro_helpers::part_at(&parts, 2) else {
        return ProcMacroTryFromParseTokenStream(
            workspace_macro_helpers::compile_error_token_stream(
                str_constants::compile_error::CE_069,
            )
            .into_inner()
            .into(),
        );
    };
    let (error_ty, derives) = fixed_error_ty.0.map_or_else(
        || {
            let Some(error_ty) = workspace_macro_helpers::part_at(&parts, 5) else {
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
    ProcMacroTryFromParseTokenStream(quote::quote! {
        #[derive(Debug, #(#derives,)* generate_getter_traits_for_struct_fields::GenerateGetterTrait, optml::Optml)]
        pub struct #name(pub #inner);
        #[derive(Debug, thiserror::Error, optml::Optml)]
        pub enum #error_name {
            #[error("{:?}", .#error_field)]
            #error_variant { #error_field: #error_ty },
        }
        impl TryFromStdEnvVarOk for #name {
            type Error = #error_name;
            fn try_from_std_env_var_ok(v: StdEnvVarOk) -> Result<Self, Self::Error> {
                parse_from_str_with_error(StdEnvVarOkRef(v.0.as_str()), |#error_field| Self::Error::#error_variant { #error_field }).map(Self)
            }
        }
    }
    .into())
}
#[proc_macro]
pub fn assert_parse_ok_matches(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let parts = workspace_macro_helpers::split_top_level_commas(
        workspace_macro_helpers::ProcMacro2MacroTokens::from_into(input),
    );
    if parts.len() != 3 {
        return workspace_macro_helpers::compile_error_token_stream(
            str_constants::compile_error::CE_040,
        )
        .into_inner()
        .into();
    }
    let Some(ty) = workspace_macro_helpers::part_at(&parts, 0) else {
        return workspace_macro_helpers::compile_error_token_stream(
            str_constants::compile_error::CE_039,
        )
        .into_inner()
        .into();
    };
    let Some(value) = workspace_macro_helpers::part_at(&parts, 1) else {
        return workspace_macro_helpers::compile_error_token_stream(
            str_constants::compile_error::CE_041,
        )
        .into_inner()
        .into();
    };
    let Some(pattern) = workspace_macro_helpers::part_at(&parts, 2) else {
        return workspace_macro_helpers::compile_error_token_stream(
            str_constants::compile_error::CE_038,
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
    let parts = workspace_macro_helpers::split_top_level_commas(
        workspace_macro_helpers::ProcMacro2MacroTokens::from_into(input),
    );
    if parts.len() != 3 {
        return workspace_macro_helpers::compile_error_token_stream(
            str_constants::compile_error::CE_036,
        )
        .into_inner()
        .into();
    }
    let Some(ty) = workspace_macro_helpers::part_at(&parts, 0) else {
        return workspace_macro_helpers::compile_error_token_stream(
            str_constants::compile_error::CE_035,
        )
        .into_inner()
        .into();
    };
    let Some(value) = workspace_macro_helpers::part_at(&parts, 1) else {
        return workspace_macro_helpers::compile_error_token_stream(
            str_constants::compile_error::CE_037,
        )
        .into_inner()
        .into();
    };
    let Some(pattern) = workspace_macro_helpers::part_at(&parts, 2) else {
        return workspace_macro_helpers::compile_error_token_stream(
            str_constants::compile_error::CE_034,
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
    let parts = workspace_macro_helpers::split_top_level_commas(
        workspace_macro_helpers::ProcMacro2MacroTokens::from_into(input),
    );
    if parts.len() != 2 {
        return workspace_macro_helpers::compile_error_token_stream(
            str_constants::compile_error::CE_033,
        )
        .into_inner()
        .into();
    }
    let Some(ty) = workspace_macro_helpers::part_at(&parts, 0) else {
        return workspace_macro_helpers::compile_error_token_stream(
            str_constants::compile_error::CE_032,
        )
        .into_inner()
        .into();
    };
    let Some(pattern) = workspace_macro_helpers::part_at(&parts, 1) else {
        return workspace_macro_helpers::compile_error_token_stream(
            str_constants::compile_error::CE_031,
        )
        .into_inner()
        .into();
    };
    quote::quote! {
        assert!(matches!(parse_env::<#ty>(""), Err(#pattern)));
    }
    .into()
}
