#[must_use]
#[derive(Debug, Clone, Copy, newtype::FromInner)]
pub struct FieldLocationFile(&'static str);
#[derive(Debug, Clone, Copy, newtype::FromInner)]
pub struct FieldLocationLine(u32);
#[derive(Debug, Clone, Copy, newtype::FromInner)]
pub struct FieldLocationColumn(u32);
#[must_use]
pub fn generate_field_location_new_token_stream(
    file: FieldLocationFile,
    line: FieldLocationLine,
    column: FieldLocationColumn,
) -> crate::generated_rust_token_stream::GeneratedRustTokenStream {
    let location_snake_case = naming::LocationSnakeCase;
    let location_new_token_stream = {
        let file_token_stream = generate_quotes::dq_token_stream(&file.0);
        let line_token_stream = {
            let literal = proc_macro2::Literal::u32_unsuffixed(line.0);
            quote::quote! {#literal}
        };
        let column_token_stream = {
            let literal = proc_macro2::Literal::u32_unsuffixed(column.0);
            quote::quote! {#literal}
        };
        quote::quote! {
            location_lib::location::Location::new(
                file!(),
                line!(),
                column!(),
                Some(location_lib::location::Occr {
                    file: location_lib::location::LocationFile::try_from(String::from(#file_token_stream)).unwrap_or_else(location_lib::location::LocationFile::from),
                    line: location_lib::location::LocationLine::from(#line_token_stream),
                    column: location_lib::location::LocationColumn::from(#column_token_stream),
                })
            )
        }
    };
    crate::generated_rust_token_stream::GeneratedRustTokenStream::from(
        quote::quote! {#location_snake_case: #location_new_token_stream},
    )
}
