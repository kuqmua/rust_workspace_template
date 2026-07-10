#[must_use]
#[derive(Debug, Clone, Copy, newtype::Newtype)]
#[newtype(from_inner)]
pub struct FieldLocFile(&'static str);
#[derive(Debug, Clone, Copy, newtype::Newtype)]
#[newtype(from_inner)]
pub struct FieldLocLine(u32);
#[derive(Debug, Clone, Copy, newtype::Newtype)]
#[newtype(from_inner)]
pub struct FieldLocCol(u32);
#[must_use]
pub fn gen_field_loc_new_ts(
    file: FieldLocFile,
    line: FieldLocLine,
    col: FieldLocCol,
) -> crate::generated_rust_ts::GeneratedRustTs {
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
                    file: loc_lib::loc::LocFile::try_from(String::from(#file_ts)).unwrap_or_else(loc_lib::loc::LocFile::from),
                    line: loc_lib::loc::LocLine::from(#line_ts),
                    col: loc_lib::loc::LocCol::from(#col_ts),
                })
            )
        }
    };
    crate::generated_rust_ts::GeneratedRustTs::from(quote::quote! {#loc_sc: #loc_new_ts})
}
