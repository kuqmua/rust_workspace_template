#[proc_macro_attribute]
pub fn pg_json_obj_config(
    _attribute_token_stream: proc_macro::TokenStream,
    item_token_stream: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    item_token_stream
}

#[proc_macro_derive(GenPgJsonObj)]
pub fn gen_pg_json_obj(input_token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    gen_pg_json_obj_src::gen_pg_json_obj(input_token_stream.into()).into()
}
