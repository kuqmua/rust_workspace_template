#[proc_macro]
pub fn gen_wh_flts(input_token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    gen_wh_flts_src::gen_wh_flts(&input_token_stream.into()).into()
}
