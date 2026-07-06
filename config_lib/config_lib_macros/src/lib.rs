#[proc_macro]
pub fn impl_try_from_non_empty_string(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let parts = workspace_macro_helpers::split_top_level_commas(input.into());
    if parts.len() != 2 {
        return workspace_macro_helpers::compile_error_ts(
            "impl_try_from_non_empty_string expects name, error name",
        )
        .into();
    }
    let Some(name_text) = workspace_macro_helpers::first_ident_at(&parts, 0) else {
        return workspace_macro_helpers::compile_error_ts(
            "impl_try_from_non_empty_string expects name",
        )
        .into();
    };
    let Some(er_name_text) = workspace_macro_helpers::first_ident_at(&parts, 1) else {
        return workspace_macro_helpers::compile_error_ts(
            "impl_try_from_non_empty_string expects error name",
        )
        .into();
    };
    let name = quote::format_ident!("{name_text}");
    let er_name = quote::format_ident!("{er_name_text}");
    quote::quote! {
        #[derive(Debug, Clone, gen_getter_traits_for_struct_fields::GenGetterTrait, Optml)]
        pub struct #name(pub String);
        #[derive(Debug, Clone, Copy, Error, Optml)]
        pub enum #er_name {
            #[error("{is_empty:?}")]
            IsEmpty { is_empty: &'static str },
        }
        impl TryFromStdEnvVarOk for #name {
            type Error = #er_name;
            fn try_from_std_env_var_ok(v: String) -> Result<Self, Self::Error> {
                try_map_non_empty_env_value(v, |is_empty| Self::Error::IsEmpty { is_empty }, Self)
            }
        }
    }
    .into()
}
#[proc_macro]
pub fn impl_try_from_secret_url(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let parts = workspace_macro_helpers::split_top_level_commas(input.into());
    if parts.len() != 2 {
        return workspace_macro_helpers::compile_error_ts(
            "impl_try_from_secret_url expects name, error name",
        )
        .into();
    }
    let Some(name_text) = workspace_macro_helpers::first_ident_at(&parts, 0) else {
        return workspace_macro_helpers::compile_error_ts("impl_try_from_secret_url expects name")
            .into();
    };
    let Some(er_name_text) = workspace_macro_helpers::first_ident_at(&parts, 1) else {
        return workspace_macro_helpers::compile_error_ts(
            "impl_try_from_secret_url expects error name",
        )
        .into();
    };
    let name = quote::format_ident!("{name_text}");
    let er_name = quote::format_ident!("{er_name_text}");
    quote::quote! {
        #[derive(Debug, gen_getter_traits_for_struct_fields::GenGetterTrait, Optml)]
        pub struct #name(pub SecretBox<String>);
        #[derive(Debug, Clone, Copy, Error, Optml)]
        pub enum #er_name {
            #[error("{is_empty:?}")]
            IsEmpty { is_empty: &'static str },
        }
        impl TryFromStdEnvVarOk for #name {
            type Error = #er_name;
            fn try_from_std_env_var_ok(v: String) -> Result<Self, Self::Error> {
                try_map_non_empty_env_value(
                    v,
                    |is_empty| Self::Error::IsEmpty { is_empty },
                    |v| Self(SecretBox::new(Box::new(v))),
                )
            }
        }
    }
    .into()
}
#[proc_macro]
pub fn impl_try_from_parse(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    impl_try_from_parse_with_er_ty(input.into(), None)
}
#[proc_macro]
pub fn impl_try_from_parse_string_er(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    impl_try_from_parse_with_er_ty(input.into(), Some(quote::quote! {String}))
}
fn impl_try_from_parse_with_er_ty(
    input: proc_macro2::TokenStream,
    fixed_er_ty: Option<proc_macro2::TokenStream>,
) -> proc_macro::TokenStream {
    let parts = workspace_macro_helpers::split_top_level_commas(input);
    let min_len = if fixed_er_ty.is_some() { 5 } else { 6 };
    if parts.len() < min_len {
        return workspace_macro_helpers::compile_error_ts(
            "impl_try_from_parse expects name, error name, inner type and error variant",
        )
        .into();
    }
    let Some(name_text) = workspace_macro_helpers::first_ident_at(&parts, 0) else {
        return workspace_macro_helpers::compile_error_ts("impl_try_from_parse expects name")
            .into();
    };
    let Some(er_name_text) = workspace_macro_helpers::first_ident_at(&parts, 1) else {
        return workspace_macro_helpers::compile_error_ts("impl_try_from_parse expects error name")
            .into();
    };
    let Some(er_vrt_text) = workspace_macro_helpers::first_ident_at(&parts, 3) else {
        return workspace_macro_helpers::compile_error_ts(
            "impl_try_from_parse expects error variant",
        )
        .into();
    };
    let Some(er_field_text) = workspace_macro_helpers::first_ident_at(&parts, 4) else {
        return workspace_macro_helpers::compile_error_ts(
            "impl_try_from_parse expects error field",
        )
        .into();
    };
    let name = quote::format_ident!("{name_text}");
    let er_name = quote::format_ident!("{er_name_text}");
    let er_vrt = quote::format_ident!("{er_vrt_text}");
    let er_field = quote::format_ident!("{er_field_text}");
    let Some(inner) = workspace_macro_helpers::part_at(&parts, 2) else {
        return workspace_macro_helpers::compile_error_ts("impl_try_from_parse expects inner type")
            .into();
    };
    let (er_ty, derives) = fixed_er_ty.map_or_else(
        || {
            let Some(er_ty) = workspace_macro_helpers::part_at(&parts, 5) else {
                return (proc_macro2::TokenStream::new(), Vec::new());
            };
            let derives = parts.get(6..).unwrap_or(&[]).to_vec();
            (er_ty, derives)
        },
        |fixed_er_ty_value| {
            (
                fixed_er_ty_value,
                vec![quote::quote! {Clone}, quote::quote! {Copy}],
            )
        },
    );
    quote::quote! {
        #[derive(Debug, #(#derives,)* gen_getter_traits_for_struct_fields::GenGetterTrait, Optml)]
        pub struct #name(pub #inner);
        #[derive(Debug, Error, Optml)]
        pub enum #er_name {
            #[error("{:?}", .#er_field)]
            #er_vrt { #er_field: #er_ty },
        }
        impl TryFromStdEnvVarOk for #name {
            type Error = #er_name;
            fn try_from_std_env_var_ok(v: String) -> Result<Self, Self::Error> {
                parse_from_str_with_er(&v, |#er_field| Self::Error::#er_vrt { #er_field }).map(Self)
            }
        }
    }
    .into()
}
#[proc_macro]
pub fn assert_parse_ok_matches(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let parts = workspace_macro_helpers::split_top_level_commas(input.into());
    if parts.len() != 3 {
        return workspace_macro_helpers::compile_error_ts(
            "assert_parse_ok_matches expects type, value, pattern",
        )
        .into();
    }
    let Some(ty) = workspace_macro_helpers::part_at(&parts, 0) else {
        return workspace_macro_helpers::compile_error_ts("assert_parse_ok_matches expects type")
            .into();
    };
    let Some(value) = workspace_macro_helpers::part_at(&parts, 1) else {
        return workspace_macro_helpers::compile_error_ts("assert_parse_ok_matches expects value")
            .into();
    };
    let Some(pattern) = workspace_macro_helpers::part_at(&parts, 2) else {
        return workspace_macro_helpers::compile_error_ts(
            "assert_parse_ok_matches expects pattern",
        )
        .into();
    };
    quote::quote! {
        assert!(matches!(parse_env::<#ty>(#value), Ok(#pattern)));
    }
    .into()
}
#[proc_macro]
pub fn assert_parse_err_matches(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let parts = workspace_macro_helpers::split_top_level_commas(input.into());
    if parts.len() != 3 {
        return workspace_macro_helpers::compile_error_ts(
            "assert_parse_err_matches expects type, value, pattern",
        )
        .into();
    }
    let Some(ty) = workspace_macro_helpers::part_at(&parts, 0) else {
        return workspace_macro_helpers::compile_error_ts("assert_parse_err_matches expects type")
            .into();
    };
    let Some(value) = workspace_macro_helpers::part_at(&parts, 1) else {
        return workspace_macro_helpers::compile_error_ts("assert_parse_err_matches expects value")
            .into();
    };
    let Some(pattern) = workspace_macro_helpers::part_at(&parts, 2) else {
        return workspace_macro_helpers::compile_error_ts(
            "assert_parse_err_matches expects pattern",
        )
        .into();
    };
    quote::quote! {
        assert!(matches!(parse_env::<#ty>(#value), Err(#pattern)));
    }
    .into()
}
#[proc_macro]
pub fn assert_empty_parse_err_matches(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let parts = workspace_macro_helpers::split_top_level_commas(input.into());
    if parts.len() != 2 {
        return workspace_macro_helpers::compile_error_ts(
            "assert_empty_parse_err_matches expects type, pattern",
        )
        .into();
    }
    let Some(ty) = workspace_macro_helpers::part_at(&parts, 0) else {
        return workspace_macro_helpers::compile_error_ts(
            "assert_empty_parse_err_matches expects type",
        )
        .into();
    };
    let Some(pattern) = workspace_macro_helpers::part_at(&parts, 1) else {
        return workspace_macro_helpers::compile_error_ts(
            "assert_empty_parse_err_matches expects pattern",
        )
        .into();
    };
    quote::quote! {
        assert!(matches!(parse_env::<#ty>(""), Err(#pattern)));
    }
    .into()
}
