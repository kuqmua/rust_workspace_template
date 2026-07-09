pub fn pgn_start_end_init_ts(v: &dyn quote::ToTokens) -> crate::generated_rust_ts::GeneratedRustTs {
    quote::quote! {
        let start = #v.pgn.start();
        let end = #v.pgn.end();
    }
    .into()
}
