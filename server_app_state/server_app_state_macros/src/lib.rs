#[proc_macro]
pub fn impl_cfg_getter(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let parts = workspace_macro_helpers::split_top_level_commas(
        workspace_macro_helpers::MacroTokens::from_into(input),
    );
    if parts.len() != 3 {
        return workspace_macro_helpers::compile_error_ts(
            "impl_cfg_getter expects trait, fn, ret_ty",
        )
        .into_inner()
        .into();
    }
    let Some(trait_ts) = workspace_macro_helpers::part_at(&parts, 0) else {
        return workspace_macro_helpers::compile_error_ts("impl_cfg_getter expects trait name")
            .into_inner()
            .into();
    };
    let Some(fn_name) = workspace_macro_helpers::first_ident_at(&parts, 1) else {
        return workspace_macro_helpers::compile_error_ts("impl_cfg_getter expects fn name")
            .into_inner()
            .into();
    };
    let fn_ident = quote::format_ident!("{fn_name}");
    let Some(ret_ty) = workspace_macro_helpers::part_at(&parts, 2) else {
        return workspace_macro_helpers::compile_error_ts("impl_cfg_getter expects return type")
            .into_inner()
            .into();
    };
    quote::quote! {
        impl #trait_ts for ServerAppState<'_> {
            fn #fn_ident(&self) -> &#ret_ty {
                self.cfg_ref().#fn_ident()
            }
        }
    }
    .into()
}
