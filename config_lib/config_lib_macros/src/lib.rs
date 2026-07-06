use proc_macro::TokenStream;
use proc_macro2::TokenStream as Ts2;
use quote::{format_ident, quote};
use workspace_macro_helpers::{compile_error_ts, first_ident_at, part_at, split_top_level_commas};
#[proc_macro]
pub fn impl_try_from_non_empty_string(input: TokenStream) -> TokenStream {
    let parts = split_top_level_commas(input.into());
    if parts.len() != 2 {
        return compile_error_ts("impl_try_from_non_empty_string expects name, error name").into();
    }
    let Some(name_text) = first_ident_at(&parts, 0) else {
        return compile_error_ts("impl_try_from_non_empty_string expects name").into();
    };
    let Some(er_name_text) = first_ident_at(&parts, 1) else {
        return compile_error_ts("impl_try_from_non_empty_string expects error name").into();
    };
    let name = format_ident!("{name_text}");
    let er_name = format_ident!("{er_name_text}");
    quote! {
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
pub fn impl_try_from_secret_url(input: TokenStream) -> TokenStream {
    let parts = split_top_level_commas(input.into());
    if parts.len() != 2 {
        return compile_error_ts("impl_try_from_secret_url expects name, error name").into();
    }
    let Some(name_text) = first_ident_at(&parts, 0) else {
        return compile_error_ts("impl_try_from_secret_url expects name").into();
    };
    let Some(er_name_text) = first_ident_at(&parts, 1) else {
        return compile_error_ts("impl_try_from_secret_url expects error name").into();
    };
    let name = format_ident!("{name_text}");
    let er_name = format_ident!("{er_name_text}");
    quote! {
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
pub fn impl_try_from_parse(input: TokenStream) -> TokenStream {
    impl_try_from_parse_with_er_ty(input.into(), None)
}
#[proc_macro]
pub fn impl_try_from_parse_string_er(input: TokenStream) -> TokenStream {
    impl_try_from_parse_with_er_ty(input.into(), Some(quote! {String}))
}
fn impl_try_from_parse_with_er_ty(input: Ts2, fixed_er_ty: Option<Ts2>) -> TokenStream {
    let parts = split_top_level_commas(input);
    let min_len = if fixed_er_ty.is_some() { 5 } else { 6 };
    if parts.len() < min_len {
        return compile_error_ts(
            "impl_try_from_parse expects name, error name, inner type and error variant",
        )
        .into();
    }
    let Some(name_text) = first_ident_at(&parts, 0) else {
        return compile_error_ts("impl_try_from_parse expects name").into();
    };
    let Some(er_name_text) = first_ident_at(&parts, 1) else {
        return compile_error_ts("impl_try_from_parse expects error name").into();
    };
    let Some(er_vrt_text) = first_ident_at(&parts, 3) else {
        return compile_error_ts("impl_try_from_parse expects error variant").into();
    };
    let Some(er_field_text) = first_ident_at(&parts, 4) else {
        return compile_error_ts("impl_try_from_parse expects error field").into();
    };
    let name = format_ident!("{name_text}");
    let er_name = format_ident!("{er_name_text}");
    let er_vrt = format_ident!("{er_vrt_text}");
    let er_field = format_ident!("{er_field_text}");
    let Some(inner) = part_at(&parts, 2) else {
        return compile_error_ts("impl_try_from_parse expects inner type").into();
    };
    let (er_ty, derives) = fixed_er_ty.map_or_else(
        || {
            let Some(er_ty) = part_at(&parts, 5) else {
                return (Ts2::new(), Vec::new());
            };
            let derives = parts.get(6..).unwrap_or(&[]).to_vec();
            (er_ty, derives)
        },
        |fixed_er_ty_value| (fixed_er_ty_value, vec![quote! {Clone}, quote! {Copy}]),
    );
    quote! {
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
pub fn assert_parse_ok_matches(input: TokenStream) -> TokenStream {
    let parts = split_top_level_commas(input.into());
    if parts.len() != 3 {
        return compile_error_ts("assert_parse_ok_matches expects type, value, pattern").into();
    }
    let Some(ty) = part_at(&parts, 0) else {
        return compile_error_ts("assert_parse_ok_matches expects type").into();
    };
    let Some(value) = part_at(&parts, 1) else {
        return compile_error_ts("assert_parse_ok_matches expects value").into();
    };
    let Some(pattern) = part_at(&parts, 2) else {
        return compile_error_ts("assert_parse_ok_matches expects pattern").into();
    };
    quote! {
        assert!(matches!(parse_env::<#ty>(#value), Ok(#pattern)));
    }
    .into()
}
#[proc_macro]
pub fn assert_parse_err_matches(input: TokenStream) -> TokenStream {
    let parts = split_top_level_commas(input.into());
    if parts.len() != 3 {
        return compile_error_ts("assert_parse_err_matches expects type, value, pattern").into();
    }
    let Some(ty) = part_at(&parts, 0) else {
        return compile_error_ts("assert_parse_err_matches expects type").into();
    };
    let Some(value) = part_at(&parts, 1) else {
        return compile_error_ts("assert_parse_err_matches expects value").into();
    };
    let Some(pattern) = part_at(&parts, 2) else {
        return compile_error_ts("assert_parse_err_matches expects pattern").into();
    };
    quote! {
        assert!(matches!(parse_env::<#ty>(#value), Err(#pattern)));
    }
    .into()
}
#[proc_macro]
pub fn assert_empty_parse_err_matches(input: TokenStream) -> TokenStream {
    let parts = split_top_level_commas(input.into());
    if parts.len() != 2 {
        return compile_error_ts("assert_empty_parse_err_matches expects type, pattern").into();
    }
    let Some(ty) = part_at(&parts, 0) else {
        return compile_error_ts("assert_empty_parse_err_matches expects type").into();
    };
    let Some(pattern) = part_at(&parts, 1) else {
        return compile_error_ts("assert_empty_parse_err_matches expects pattern").into();
    };
    quote! {
        assert!(matches!(parse_env::<#ty>(""), Err(#pattern)));
    }
    .into()
}
