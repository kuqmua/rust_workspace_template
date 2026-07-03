#[proc_macro]
pub fn gen_pg_json(input_token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    gen_pg_json_src::gen_pg_json(&input_token_stream.into()).into()
}
