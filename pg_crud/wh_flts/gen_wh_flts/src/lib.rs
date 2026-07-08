#[proc_macro]
pub fn gen_wh_flts(input_ts: proc_macro::TokenStream) -> proc_macro::TokenStream {
    gen_wh_flts_src::gen_wh_flts(gen_wh_flts_src::ProcMacro2GenWhFltsInput::from(
        &input_ts.into(),
    ))
    .to_string()
    .parse::<proc_macro::TokenStream>()
    .expect("6716175c")
}
