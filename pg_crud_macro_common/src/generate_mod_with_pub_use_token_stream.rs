#[must_use]
pub fn generate_mod_with_pub_use_token_stream(
    mod_name: &dyn quote::ToTokens,
    content_token_stream: &crate::domain_types::ProcMacro2GeneratedRustTokenStreamVec,
) -> macro_helpers::domain_types::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream{
    quote::quote! {
        #[allow(unused_qualifications)]
        #[allow(unused_variables)]
        #[allow(clippy::absolute_paths)]
        #[allow(clippy::arbitrary_source_item_ordering)]
        mod #mod_name {
            #content_token_stream
        }
        pub use #mod_name::*;
    }
    .into()
}
