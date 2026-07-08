#[proc_macro]
pub fn impl_to_err_string_with(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let Some((types_raw, closure)) = workspace_macro_helpers::split_fat_arrow(
        workspace_macro_helpers::MacroTokens::from_into(input),
    ) else {
        return workspace_macro_helpers::compile_error_ts(
            "impl_to_err_string_with expects types => |value| body",
        )
        .into_inner()
        .into();
    };
    let Some((value, body)) = workspace_macro_helpers::closure_ident_and_body(closure) else {
        return workspace_macro_helpers::compile_error_ts(
            "impl_to_err_string_with expects closure",
        )
        .into_inner()
        .into();
    };
    let value_ident = quote::format_ident!("{value}");
    let types = workspace_macro_helpers::split_top_level_commas(types_raw)
        .into_iter()
        .collect::<Vec<proc_macro2::TokenStream>>();
    quote::quote! {
        #(impl ToErrString for #types {
            fn to_err_string(&self) -> ToErrStringValue {
                let #value_ident = self;
                ToErrStringValue::try_from(#body).unwrap_or_else(ToErrStringValue::from)
            }
        })*
    }
    .into()
}
#[proc_macro]
pub fn impl_to_err_string_const(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let parsed_pairs_res = workspace_macro_helpers::split_top_level_commas(
        workspace_macro_helpers::MacroTokens::from_into(input),
    )
    .into_iter()
    .filter(|part| !part.is_empty())
    .map(|part| {
        workspace_macro_helpers::split_fat_arrow(part)
            .map(|(ty, msg)| (ty.into_inner(), msg.into_inner()))
            .ok_or_else(|| {
                workspace_macro_helpers::compile_error_ts(
                    "impl_to_err_string_const expects type => msg",
                )
            })
    })
    .collect::<Result<Vec<(proc_macro2::TokenStream, proc_macro2::TokenStream)>, _>>();
    let pairs = match parsed_pairs_res {
        Ok(v) => v,
        Err(er) => return er.into_inner().into(),
    };
    let (types, msgs): (Vec<_>, Vec<_>) = pairs.into_iter().unzip();
    quote::quote! {
        #(impl ToErrString for #types {
            fn to_err_string(&self) -> ToErrStringValue {
                static_str_to_owned(StaticStrToOwnedInput(#msgs))
            }
        })*
    }
    .into()
}
#[proc_macro]
pub fn impl_to_err_string_as_ref_str(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let types = workspace_macro_helpers::split_top_level_commas(
        workspace_macro_helpers::MacroTokens::from_into(input),
    )
    .into_iter()
    .filter(|part| !part.is_empty())
    .collect::<Vec<proc_macro2::TokenStream>>();
    quote::quote! {
        #(impl ToErrString for #types {
            fn to_err_string(&self) -> ToErrStringValue {
                as_ref_str_to_owned(self)
            }
        })*
    }
    .into()
}
