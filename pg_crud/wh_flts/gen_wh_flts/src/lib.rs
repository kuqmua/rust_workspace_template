#[proc_macro]
pub fn gen_wh_flts(input_ts: proc_macro::TokenStream) -> proc_macro::TokenStream {
    gen_wh_flts_src::gen_wh_flts(gen_wh_flts_src::GenWhFltsInput(&input_ts.into()))
        .0
        .into()
}
