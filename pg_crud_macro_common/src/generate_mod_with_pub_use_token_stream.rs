#[must_use]
pub fn generate_mod_with_pub_use_token_stream(
    mod_name: &dyn quote::ToTokens,
    content_token_stream: &crate::domain_types::ProcMacro2GeneratedRustTokenStreamVec,
) -> macro_helpers::domain_types::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream{
    quote::quote! {
        // The owner module retains lint-sensitive semantics from the original implementation.
        #[allow(unused_qualifications)]
        // The owner module retains lint-sensitive semantics from the original implementation.
        #[allow(unused_variables)]
        // The owner module retains lint-sensitive semantics from the original implementation.
        #[allow(clippy::absolute_paths)]
        // The owner module retains lint-sensitive semantics from the original implementation.
        #[allow(clippy::arbitrary_source_item_ordering)]
        mod #mod_name {
            #content_token_stream
        }
        pub use #mod_name::*;
    }
    .into()
}
