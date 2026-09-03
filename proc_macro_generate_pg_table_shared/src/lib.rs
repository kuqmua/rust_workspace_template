#![allow(
    clippy::must_use_candidate,
    clippy::useless_conversion,
    reason = "shared proc-macro implementations preserve original entrypoint conversion points while returning proc_macro2 streams to one-entrypoint facade crates; every result is consumed immediately by its facade"
)]

pub fn generate_pg_table_config(
    _attr: proc_macro2::TokenStream,
    item: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    item
}
pub fn cm_error_variants(
    _attr: proc_macro2::TokenStream,
    item: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    item
}
pub fn co_error_variants(
    _attr: proc_macro2::TokenStream,
    item: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    item
}
pub fn ro_error_variants(
    _attr: proc_macro2::TokenStream,
    item: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    item
}
pub fn rm_error_variants(
    _attr: proc_macro2::TokenStream,
    item: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    item
}
pub fn uo_error_variants(
    _attr: proc_macro2::TokenStream,
    item: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    item
}
pub fn um_error_variants(
    _attr: proc_macro2::TokenStream,
    item: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    item
}
pub fn dlo_error_variants(
    _attr: proc_macro2::TokenStream,
    item: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    item
}
pub fn dm_error_variants(
    _attr: proc_macro2::TokenStream,
    item: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    item
}
pub fn common_error_variants(
    _attr: proc_macro2::TokenStream,
    item: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    item
}
pub fn cm_logic(
    _attr: proc_macro2::TokenStream,
    item: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    item
}
pub fn co_logic(
    _attr: proc_macro2::TokenStream,
    item: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    item
}
pub fn rm_logic(
    _attr: proc_macro2::TokenStream,
    item: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    item
}
pub fn ro_logic(
    _attr: proc_macro2::TokenStream,
    item: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    item
}
pub fn um_logic(
    _attr: proc_macro2::TokenStream,
    item: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    item
}
pub fn uo_logic(
    _attr: proc_macro2::TokenStream,
    item: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    item
}
pub fn dm_logic(
    _attr: proc_macro2::TokenStream,
    item: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    item
}
pub fn dlo_logic(
    _attr: proc_macro2::TokenStream,
    item: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    item
}
pub fn common_logic(
    _attr: proc_macro2::TokenStream,
    item: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    item
}
pub fn derive_generate_pg_table(
    token_stream: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    let input_token_stream = token_stream.into();
    generate_pg_table_src::generate_pg_table::generate_pg_table(
        macro_helpers::proc_macro2_token_stream_ref::ProcMacro2TokenStreamRef::from(
            &input_token_stream,
        ),
    )
    .to_string()
    .parse::<proc_macro2::TokenStream>()
    .expect(constants_str::DIAGNOSTIC_6BFF799B)
}
