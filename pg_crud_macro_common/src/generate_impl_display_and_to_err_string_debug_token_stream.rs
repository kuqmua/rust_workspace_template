#![allow(
    clippy::wildcard_imports,
    reason = "split owner modules import the private facade vocabulary used by the moved generator"
)]
use crate::domain_types::*;

pub fn generate_impl_display_and_to_err_string_debug_token_stream(
    identifier: &dyn quote::ToTokens,
) -> macro_helpers::domain_types::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream{
    let impl_display_token_stream =
        macro_helpers::domain_types::generate_impl_display_token_stream::generate_impl_display_token_stream(
            &proc_macro2::TokenStream::new(),
            identifier,
            &proc_macro2::TokenStream::new(),
            &quote::quote! {write!(f, "{self:?}")},
        );
    let impl_to_err_string_token_stream = generate_impl_to_err_string_no_generics_token_stream(
        identifier,
        &quote::quote! {format!("{self:#?}")},
    );
    quote::quote! {
        #impl_display_token_stream
        #impl_to_err_string_token_stream
    }
    .into()
}
