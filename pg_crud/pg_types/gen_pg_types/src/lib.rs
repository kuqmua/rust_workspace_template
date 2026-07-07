#[proc_macro]
pub fn gen_pg_types(input_ts: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input_tokens = input_ts.into();
    gen_pg_types_src::gen_pg_types(gen_pg_types_src::TsRef(&input_tokens))
        .0
        .into()
}
