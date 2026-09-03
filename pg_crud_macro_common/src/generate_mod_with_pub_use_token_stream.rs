#[must_use]
pub fn generate_mod_with_pub_use_token_stream(
    mod_name: &dyn quote::ToTokens,
    proc_macro2_generated_rust_token_stream_vec: &crate::proc_macro2_generated_rust_token_stream_vec::ProcMacro2GeneratedRustTokenStreamVec,
) -> macro_helpers::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream {
    quote::quote! {

        #[allow(unused_qualifications, reason = "lint suppression is required here")]

        #[allow(unused_variables, reason = "lint suppression is required here")]

        #[allow(clippy::absolute_paths, reason = "lint suppression is required here")]

        #[allow(clippy::arbitrary_source_item_ordering, reason = "lint suppression is required here")]
        pub mod #mod_name {
            #proc_macro2_generated_rust_token_stream_vec
        }
    }
    .into()
}
