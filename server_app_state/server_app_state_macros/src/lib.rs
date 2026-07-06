use proc_macro::TokenStream;
use quote::{format_ident, quote};
use workspace_macro_helpers::{compile_error_ts, first_ident_at, part_at, split_top_level_commas};
#[proc_macro]
pub fn impl_cfg_getter(input: TokenStream) -> TokenStream {
    let parts = split_top_level_commas(input.into());
    if parts.len() != 3 {
        return compile_error_ts("impl_cfg_getter expects trait, fn, ret_ty").into();
    }
    let Some(trait_name) = first_ident_at(&parts, 0) else {
        return compile_error_ts("impl_cfg_getter expects trait name").into();
    };
    let Some(fn_name) = first_ident_at(&parts, 1) else {
        return compile_error_ts("impl_cfg_getter expects fn name").into();
    };
    let trait_ident = format_ident!("{trait_name}");
    let fn_ident = format_ident!("{fn_name}");
    let Some(ret_ty) = part_at(&parts, 2) else {
        return compile_error_ts("impl_cfg_getter expects return type").into();
    };
    quote! {
        impl #trait_ident for ServerAppState<'_> {
            fn #fn_ident(&self) -> &#ret_ty {
                self.cfg_ref().#fn_ident()
            }
        }
    }
    .into()
}
