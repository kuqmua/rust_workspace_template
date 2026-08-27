#![allow(
    clippy::module_inception,
    reason = "same-named type and function owners require nested modules under the facade"
)]
#[path = "generate_if_write_is_error_token_stream/generate_if_write_is_error_token_stream.rs"]
mod generate_if_write_is_error_token_stream;
#[path = "generate_if_write_is_error_token_stream/proc_macro2_if_write_is_err_token_stream.rs"]
mod proc_macro2_if_write_is_err_token_stream;

pub use generate_if_write_is_error_token_stream::generate_if_write_is_error_token_stream;
pub use proc_macro2_if_write_is_err_token_stream::ProcMacro2IfWriteIsErrTokenStream;
