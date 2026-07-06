pub fn pgn_start_end_init_ts(v: &dyn quote::ToTokens) -> proc_macro2::TokenStream {
    quote::quote! {
        let start = #v.pgn.start();
        let end = #v.pgn.end();
    }
}
