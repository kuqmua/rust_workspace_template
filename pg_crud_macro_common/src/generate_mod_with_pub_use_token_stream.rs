#[must_use]
pub fn generate_mod_with_pub_use_token_stream(
    mod_name: &dyn quote::ToTokens,
    proc_macro2_generated_rust_token_stream_vec: &crate::proc_macro2_generated_rust_token_stream_vec::ProcMacro2GeneratedRustTokenStreamVec,
) -> macro_helpers::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream {
    quote::quote! {
        // The owner module retains lint-sensitive semantics from the original implementation.
        #[allow(unused_qualifications)]
        // The owner module retains lint-sensitive semantics from the original implementation.
        #[allow(unused_variables)]
        // The owner module retains lint-sensitive semantics from the original implementation.
        #[allow(clippy::absolute_paths)]
        // The owner module retains lint-sensitive semantics from the original implementation.
        #[allow(clippy::arbitrary_source_item_ordering)]
        pub mod #mod_name {
            #proc_macro2_generated_rust_token_stream_vec
        }
    }
    .into()
}
