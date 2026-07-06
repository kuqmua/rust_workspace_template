#[must_use]
pub fn gen_field_loc_new_ts(file: &'static str, line: u32, col: u32) -> proc_macro2::TokenStream {
    let loc_sc = naming::LocSc;
    let loc_new_ts = {
        let file_ts = gen_quotes::dq_ts(&file);
        let line_ts = {
            let literal = proc_macro2::Literal::u32_unsuffixed(line);
            quote::quote! {#literal}
        };
        let col_ts = {
            let literal = proc_macro2::Literal::u32_unsuffixed(col);
            quote::quote! {#literal}
        };
        quote::quote! {
            loc_lib::loc::Loc::new(
                file!().to_owned(),
                line!(),
                column!(),
                Some(loc_lib::loc::Occr {
                    file: String::from(#file_ts),
                    line: #line_ts,
                    col: #col_ts,
                })
            )
        }
    };
    quote::quote! {#loc_sc: #loc_new_ts}
}
