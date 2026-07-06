#[proc_macro]
pub fn impl_to_err_string_with(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let Some((types_raw, closure)) = workspace_macro_helpers::split_fat_arrow(input.into()) else {
        return workspace_macro_helpers::compile_error_ts(
            "impl_to_err_string_with expects types => |value| body",
        )
        .into();
    };
    let Some((value, body)) = workspace_macro_helpers::closure_ident_and_body(closure) else {
        return workspace_macro_helpers::compile_error_ts(
            "impl_to_err_string_with expects closure",
        )
        .into();
    };
    let value_ident = quote::format_ident!("{value}");
    let types = workspace_macro_helpers::split_top_level_commas(types_raw)
        .into_iter()
        .collect::<Vec<proc_macro2::TokenStream>>();
    quote::quote! {
        #(impl ToErrString for #types {
            fn to_err_string(&self) -> String {
                let #value_ident = self;
                #body
            }
        })*
    }
    .into()
}
#[proc_macro]
pub fn impl_to_err_string_const(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut types = Vec::new();
    let mut msgs = Vec::new();
    for part in workspace_macro_helpers::split_top_level_commas(input.into()) {
        if part.is_empty() {
            continue;
        }
        let Some((ty, msg)) = workspace_macro_helpers::split_fat_arrow(part) else {
            return workspace_macro_helpers::compile_error_ts(
                "impl_to_err_string_const expects type => msg",
            )
            .into();
        };
        types.push(ty);
        msgs.push(msg);
    }
    quote::quote! {
        #(impl ToErrString for #types {
            fn to_err_string(&self) -> String {
                static_str_to_owned(#msgs)
            }
        })*
    }
    .into()
}
#[proc_macro]
pub fn impl_to_err_string_as_ref_str(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let types = workspace_macro_helpers::split_top_level_commas(input.into())
        .into_iter()
        .filter(|part| !part.is_empty())
        .collect::<Vec<proc_macro2::TokenStream>>();
    quote::quote! {
        #(impl ToErrString for #types {
            fn to_err_string(&self) -> String {
                as_ref_str_to_owned(self)
            }
        })*
    }
    .into()
}
