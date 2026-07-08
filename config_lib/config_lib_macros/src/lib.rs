struct TryFromParseInput(proc_macro2::TokenStream);
struct TryFromParseFixedErTy(Option<proc_macro2::TokenStream>);
struct TryFromParseTs(proc_macro::TokenStream);
#[proc_macro]
pub fn impl_try_from_non_empty_string(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let parts = workspace_macro_helpers::split_top_level_commas(
        workspace_macro_helpers::MacroTokens::from_into(input),
    );
    if parts.len() != 2 {
        return workspace_macro_helpers::compile_error_ts(
            "impl_try_from_non_empty_string expects name, error name",
        )
        .into_inner()
        .into();
    }
    let Some(name_text) = workspace_macro_helpers::first_ident_at(&parts, 0) else {
        return workspace_macro_helpers::compile_error_ts(
            "impl_try_from_non_empty_string expects name",
        )
        .into_inner()
        .into();
    };
    let Some(er_name_text) = workspace_macro_helpers::first_ident_at(&parts, 1) else {
        return workspace_macro_helpers::compile_error_ts(
            "impl_try_from_non_empty_string expects error name",
        )
        .into_inner()
        .into();
    };
    let name = quote::format_ident!("{name_text}");
    let er_name = quote::format_ident!("{er_name_text}");
    quote::quote! {
        #[derive(Debug, Clone, gen_getter_traits_for_struct_fields::GenGetterTrait, optml::Optml)]
        pub struct #name(pub String);
        #[derive(Debug, Clone, Copy, thiserror::Error, optml::Optml)]
        pub enum #er_name {
            #[error("{is_empty:?}")]
            IsEmpty { is_empty: &'static str },
        }
        impl TryFromStdEnvVarOk for #name {
            type Error = #er_name;
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
        workspace_macro_helpers::MacroTokens::from_into(input),
    );
    if parts.len() != 2 {
        return workspace_macro_helpers::compile_error_ts(
            "impl_try_from_secret_url expects name, error name",
        )
        .into_inner()
        .into();
    }
    let Some(name_text) = workspace_macro_helpers::first_ident_at(&parts, 0) else {
        return workspace_macro_helpers::compile_error_ts("impl_try_from_secret_url expects name")
            .into_inner()
            .into();
    };
    let Some(er_name_text) = workspace_macro_helpers::first_ident_at(&parts, 1) else {
        return workspace_macro_helpers::compile_error_ts(
            "impl_try_from_secret_url expects error name",
        )
        .into_inner()
        .into();
    };
    let name = quote::format_ident!("{name_text}");
    let er_name = quote::format_ident!("{er_name_text}");
    quote::quote! {
        #[derive(Debug, gen_getter_traits_for_struct_fields::GenGetterTrait, optml::Optml)]
        pub struct #name(pub secrecy::SecretBox<String>);
        #[derive(Debug, Clone, Copy, thiserror::Error, optml::Optml)]
        pub enum #er_name {
            #[error("{is_empty:?}")]
            IsEmpty { is_empty: &'static str },
        }
        impl TryFromStdEnvVarOk for #name {
            type Error = #er_name;
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
    impl_try_from_parse_with_er_ty(TryFromParseInput(input.into()), TryFromParseFixedErTy(None)).0
}
#[proc_macro]
pub fn impl_try_from_parse_string_er(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    impl_try_from_parse_with_er_ty(
        TryFromParseInput(input.into()),
        TryFromParseFixedErTy(Some(quote::quote! {String})),
    )
    .0
}
fn impl_try_from_parse_with_er_ty(
    input: TryFromParseInput,
    fixed_er_ty: TryFromParseFixedErTy,
) -> TryFromParseTs {
    let parts = workspace_macro_helpers::split_top_level_commas(input.0);
    let min_len = if fixed_er_ty.0.is_some() { 5 } else { 6 };
    if parts.len() < min_len {
        return TryFromParseTs(
            workspace_macro_helpers::compile_error_ts(
                "impl_try_from_parse expects name, error name, inner type and error variant",
            )
            .into_inner()
            .into(),
        );
    }
    let Some(name_text) = workspace_macro_helpers::first_ident_at(&parts, 0) else {
        return TryFromParseTs(
            workspace_macro_helpers::compile_error_ts("impl_try_from_parse expects name")
                .into_inner()
                .into(),
        );
    };
    let Some(er_name_text) = workspace_macro_helpers::first_ident_at(&parts, 1) else {
        return TryFromParseTs(
            workspace_macro_helpers::compile_error_ts("impl_try_from_parse expects error name")
                .into_inner()
                .into(),
        );
    };
    let Some(er_vrt_text) = workspace_macro_helpers::first_ident_at(&parts, 3) else {
        return TryFromParseTs(
            workspace_macro_helpers::compile_error_ts("impl_try_from_parse expects error variant")
                .into_inner()
                .into(),
        );
    };
    let Some(er_field_text) = workspace_macro_helpers::first_ident_at(&parts, 4) else {
        return TryFromParseTs(
            workspace_macro_helpers::compile_error_ts("impl_try_from_parse expects error field")
                .into_inner()
                .into(),
        );
    };
    let name = quote::format_ident!("{name_text}");
    let er_name = quote::format_ident!("{er_name_text}");
    let er_vrt = quote::format_ident!("{er_vrt_text}");
    let er_field = quote::format_ident!("{er_field_text}");
    let Some(inner) = workspace_macro_helpers::part_at(&parts, 2) else {
        return TryFromParseTs(
            workspace_macro_helpers::compile_error_ts("impl_try_from_parse expects inner type")
                .into_inner()
                .into(),
        );
    };
    let (er_ty, derives) = fixed_er_ty.0.map_or_else(
        || {
            let Some(er_ty) = workspace_macro_helpers::part_at(&parts, 5) else {
                return (proc_macro2::TokenStream::new(), Vec::new());
            };
            let derives = parts.get(6..).unwrap_or(&[]).to_vec();
            (er_ty.into(), derives)
        },
        |fixed_er_ty_value| {
            (
                fixed_er_ty_value,
                vec![quote::quote! {Clone}, quote::quote! {Copy}],
            )
        },
    );
    TryFromParseTs(quote::quote! {
        #[derive(Debug, #(#derives,)* gen_getter_traits_for_struct_fields::GenGetterTrait, optml::Optml)]
        pub struct #name(pub #inner);
        #[derive(Debug, thiserror::Error, optml::Optml)]
        pub enum #er_name {
            #[error("{:?}", .#er_field)]
            #er_vrt { #er_field: #er_ty },
        }
        impl TryFromStdEnvVarOk for #name {
            type Error = #er_name;
            fn try_from_std_env_var_ok(v: StdEnvVarOk) -> Result<Self, Self::Error> {
                parse_from_str_with_er(StdEnvVarOkRef(v.0.as_str()), |#er_field| Self::Error::#er_vrt { #er_field }).map(Self)
            }
        }
    }
    .into())
}
#[proc_macro]
pub fn assert_parse_ok_matches(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let parts = workspace_macro_helpers::split_top_level_commas(
        workspace_macro_helpers::MacroTokens::from_into(input),
    );
    if parts.len() != 3 {
        return workspace_macro_helpers::compile_error_ts(
            "assert_parse_ok_matches expects type, value, pattern",
        )
        .into_inner()
        .into();
    }
    let Some(ty) = workspace_macro_helpers::part_at(&parts, 0) else {
        return workspace_macro_helpers::compile_error_ts("assert_parse_ok_matches expects type")
            .into_inner()
            .into();
    };
    let Some(value) = workspace_macro_helpers::part_at(&parts, 1) else {
        return workspace_macro_helpers::compile_error_ts("assert_parse_ok_matches expects value")
            .into_inner()
            .into();
    };
    let Some(pattern) = workspace_macro_helpers::part_at(&parts, 2) else {
        return workspace_macro_helpers::compile_error_ts(
            "assert_parse_ok_matches expects pattern",
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
        workspace_macro_helpers::MacroTokens::from_into(input),
    );
    if parts.len() != 3 {
        return workspace_macro_helpers::compile_error_ts(
            "assert_parse_err_matches expects type, value, pattern",
        )
        .into_inner()
        .into();
    }
    let Some(ty) = workspace_macro_helpers::part_at(&parts, 0) else {
        return workspace_macro_helpers::compile_error_ts("assert_parse_err_matches expects type")
            .into_inner()
            .into();
    };
    let Some(value) = workspace_macro_helpers::part_at(&parts, 1) else {
        return workspace_macro_helpers::compile_error_ts("assert_parse_err_matches expects value")
            .into_inner()
            .into();
    };
    let Some(pattern) = workspace_macro_helpers::part_at(&parts, 2) else {
        return workspace_macro_helpers::compile_error_ts(
            "assert_parse_err_matches expects pattern",
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
        workspace_macro_helpers::MacroTokens::from_into(input),
    );
    if parts.len() != 2 {
        return workspace_macro_helpers::compile_error_ts(
            "assert_empty_parse_err_matches expects type, pattern",
        )
        .into_inner()
        .into();
    }
    let Some(ty) = workspace_macro_helpers::part_at(&parts, 0) else {
        return workspace_macro_helpers::compile_error_ts(
            "assert_empty_parse_err_matches expects type",
        )
        .into_inner()
        .into();
    };
    let Some(pattern) = workspace_macro_helpers::part_at(&parts, 1) else {
        return workspace_macro_helpers::compile_error_ts(
            "assert_empty_parse_err_matches expects pattern",
        )
        .into_inner()
        .into();
    };
    quote::quote! {
        assert!(matches!(parse_env::<#ty>(""), Err(#pattern)));
    }
    .into()
}
