#[must_use]
pub fn generate_field_location_new_token_stream(
    field_location_file: crate::field_location_file::FieldLocationFile,
    field_location_line: crate::field_location_line::FieldLocationLine,
    field_location_column: crate::field_location_column::FieldLocationColumn,
) -> crate::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream {
    let location_snake_case = naming::domain_types::LocationSnakeCase;
    let location_new_token_stream = {
        let file_token_stream =
            generate_quotes::dq_token_stream::dq_token_stream(field_location_file.as_str());
        let line_token_stream = {
            let literal = proc_macro2::Literal::u32_unsuffixed(field_location_line.value());
            quote::quote! {#literal}
        };
        let column_token_stream = {
            let literal = proc_macro2::Literal::u32_unsuffixed(field_location_column.value());
            quote::quote! {#literal}
        };
        quote::quote! {
            location_lib::location::Location::new(
                file!(),
                location_lib::location_line::LocationLine::from(
                    std::num::NonZeroU32::new(line!()).unwrap_or(std::num::NonZeroU32::MIN),
                ),
                location_lib::location_column::LocationColumn::from(
                    std::num::NonZeroU32::new(column!()).unwrap_or(std::num::NonZeroU32::MIN),
                ),
                Some(location_lib::occurrence::Occurrence::new(
                    location_lib::location_file::LocationFile::try_from(String::from(#file_token_stream)).unwrap_or_else(location_lib::location_file::LocationFile::from),
                    location_lib::location_line::LocationLine::try_from(#line_token_stream)
                        .unwrap_or_else(|_error| location_lib::location_line::LocationLine::first()),
                    location_lib::location_column::LocationColumn::try_from(#column_token_stream)
                        .unwrap_or_else(|_error| location_lib::location_column::LocationColumn::first()),
                ))
            )
        }
    };
    crate::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream::from(
        quote::quote! {#location_snake_case: #location_new_token_stream},
    )
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_coordinates_reject_zero_and_accept_positive_values() {
        let _line_error =
            crate::field_location_line::FieldLocationLine::try_from(constants_u32::ZERO)
                .expect_err(constants_str::VALUE_A7ABF9D9);
        let _column_error =
            crate::field_location_column::FieldLocationColumn::try_from(constants_u32::ZERO)
                .expect_err(constants_str::VALUE_B430FE14);
        let _line = crate::field_location_line::FieldLocationLine::try_from(7u32)
            .expect(constants_str::DIAGNOSTIC_070DBEE8);
        let _column = crate::field_location_column::FieldLocationColumn::try_from(11u32)
            .expect(constants_str::DIAGNOSTIC_E067C790);
    }
    #[test]
    fn test_first_coordinates_generate_complete_location_field() {
        let generated = crate::generate_field_location_new_token_stream::generate_field_location_new_token_stream(
            crate::field_location_file::FieldLocationFile::from(constants_str::VALUE_E1CEB1AF),
            crate::field_location_line::FieldLocationLine::first(),
            crate::field_location_column::FieldLocationColumn::first(),
        )
        .as_ref()
        .to_string();
        assert!(generated.starts_with(constants_str::VALUE_D5D0CC41));
        assert!(generated.contains(constants_str::VALUE_B0452175));
        assert!(generated.contains(constants_str::VALUE_FD67A0AA));
        assert!(generated.contains(constants_str::VALUE_22354C74));
        assert!(generated.contains(constants_str::VALUE_A37D1D7D));
        assert!(generated.contains(constants_str::VALUE_BD7DA2C7));
    }
}
