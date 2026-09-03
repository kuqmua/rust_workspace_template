#[must_use]
pub fn generate_mod_with_pub_use_token_stream(
    mod_name: &dyn quote::ToTokens,
    proc_macro2_generated_rust_token_stream_vec: &crate::proc_macro2_generated_rust_token_stream_vec::ProcMacro2GeneratedRustTokenStreamVec,
) -> macro_helpers::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream {
    quote::quote! {

        #[allow(unused_qualifications, reason = "generate mod with pub use token stream keeps explicit generated paths stable across expansion contexts")]

        #[allow(unused_variables, reason = "generate mod with pub use token stream emits configuration-dependent bindings that are unused in some generated variants")]

        #[allow(clippy::absolute_paths, reason = "generate mod with pub use token stream uses explicit paths to comply with the workspace import policy")]

        #[allow(clippy::arbitrary_source_item_ordering, reason = "generate mod with pub use token stream keeps declaration order aligned with generated layout or processing flow")]
        pub mod #mod_name {
            #proc_macro2_generated_rust_token_stream_vec
        }
    }
    .into()
}
