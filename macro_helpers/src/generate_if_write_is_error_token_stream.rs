#![allow(
    clippy::arbitrary_source_item_ordering,
    reason = "the flat source facade keeps its owner adjacent to implementation while declaring sibling modules"
)]
pub fn generate_if_write_is_error_token_stream(
    parameters_token_stream: &dyn quote::ToTokens,
    ts: &dyn quote::ToTokens,
) -> ProcMacro2IfWriteIsErrTokenStream {
    quote::quote! {
        if {
            use std::fmt::Write as _;
            write!(#parameters_token_stream)
        }.is_err() {
            #ts
        }
    }
    .into()
}
pub use super::proc_macro2_if_write_is_err_token_stream::ProcMacro2IfWriteIsErrTokenStream;
