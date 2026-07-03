#[must_use]
pub fn gen_field_loc_new_ts<FileTokenStream, LineTokenStream, ColumnTokenStream>(
    file: &FileTokenStream,
    line: &LineTokenStream,
    column: &ColumnTokenStream,
) -> proc_macro2::TokenStream
where
    FileTokenStream: quote::ToTokens + ?Sized,
    LineTokenStream: quote::ToTokens + ?Sized,
    ColumnTokenStream: quote::ToTokens + ?Sized,
{
    quote::quote! {
        loc: loc_lib::loc::Loc::new(
            file!().to_owned(),
            line!(),
            column!(),
            Some(loc_lib::loc::Occr {
                file: String::from(#file),
                line: #line,
                col: #column,
            })
        )
    }
}
