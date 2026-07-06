use gen_quotes::dq_ts;
use naming::LocSc;
use proc_macro2::Literal;
use proc_macro2::TokenStream as Ts2;
use quote::quote;
#[must_use]
pub fn gen_field_loc_new_ts(file: &'static str, line: u32, col: u32) -> Ts2 {
    let loc_new_ts = {
        let file_ts = dq_ts(&file);
        let line_ts = {
            let literal = Literal::u32_unsuffixed(line);
            quote! {#literal}
        };
        let col_ts = {
            let literal = Literal::u32_unsuffixed(col);
            quote! {#literal}
        };
        quote! {
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
    quote! {#LocSc: #loc_new_ts}
}
