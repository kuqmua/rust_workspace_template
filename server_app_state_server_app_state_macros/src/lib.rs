#[proc_macro]
pub fn impl_cfg_accessor(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let parts = workspace_macro_helpers::domain_types::split_top_level_commas(
        workspace_macro_helpers::domain_types::ProcMacro2MacroTokens::from_into(input),
    );
    if parts.len() != 3 {
        return workspace_macro_helpers::domain_types::compile_error_token_stream(
            constants_str::COMPILE_ERROR_CE_059,
        )
        .into_inner()
        .into();
    }
    let Some(trait_token_stream) = workspace_macro_helpers::domain_types::part_at(&parts, 0) else {
        return workspace_macro_helpers::domain_types::compile_error_token_stream(
            constants_str::COMPILE_ERROR_CE_058,
        )
        .into_inner()
        .into();
    };
    let Some(fn_name) = workspace_macro_helpers::domain_types::first_identifier_at(&parts, 1)
    else {
        return workspace_macro_helpers::domain_types::compile_error_token_stream(
            constants_str::COMPILE_ERROR_CE_056,
        )
        .into_inner()
        .into();
    };
    let fn_identifier = quote::format_ident!("{fn_name}");
    let Some(ret_ty) = workspace_macro_helpers::domain_types::part_at(&parts, 2) else {
        return workspace_macro_helpers::domain_types::compile_error_token_stream(
            constants_str::COMPILE_ERROR_CE_057,
        )
        .into_inner()
        .into();
    };
    quote::quote! {
        impl #trait_token_stream for ServerAppState<'_> {
            fn #fn_identifier(&self) -> &#ret_ty {
                self.cfg_ref().#fn_identifier()
            }
        }
    }
    .into()
}
