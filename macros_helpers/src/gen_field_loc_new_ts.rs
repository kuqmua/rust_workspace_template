#[must_use]
#[derive(Debug, Clone, Copy)]
pub struct FieldLocFile(pub &'static str);
#[derive(Debug, Clone, Copy)]
pub struct FieldLocLine(pub u32);
#[derive(Debug, Clone, Copy)]
pub struct FieldLocCol(pub u32);
#[must_use]
pub fn gen_field_loc_new_ts(
    file: FieldLocFile,
    line: FieldLocLine,
    col: FieldLocCol,
) -> crate::GeneratedRustTs {
    let loc_sc = naming::LocSc;
    let loc_new_ts = {
        let file_ts = gen_quotes::dq_ts(&file.0);
        let line_ts = {
            let literal = proc_macro2::Literal::u32_unsuffixed(line.0);
            quote::quote! {#literal}
        };
        let col_ts = {
            let literal = proc_macro2::Literal::u32_unsuffixed(col.0);
            quote::quote! {#literal}
        };
        quote::quote! {
            loc_lib::loc::Loc::new(
                file!(),
                line!(),
                column!(),
                Some(loc_lib::loc::Occr {
                    file: loc_lib::loc::LocFile(String::from(#file_ts)),
                    line: loc_lib::loc::LocLine(#line_ts),
                    col: loc_lib::loc::LocCol(#col_ts),
                })
            )
        }
    };
    crate::GeneratedRustTs(quote::quote! {#loc_sc: #loc_new_ts})
}
