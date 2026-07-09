#[proc_macro]
pub fn gen_pg_types(input_ts: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input_tokens = input_ts.into();
    gen_pg_types_src::gen_pg_types(macros_helpers::ts_writer::ProcMacro2TsRef::from(
        &input_tokens,
    ))
    .to_string()
    .parse::<proc_macro::TokenStream>()
    .expect("122809ba")
}
