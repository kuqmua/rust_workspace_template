#[must_use]
pub fn generate_field_location_new_token_stream(
    file: crate::field_location_file::FieldLocationFile,
    line: crate::field_location_line::FieldLocationLine,
    column: crate::field_location_column::FieldLocationColumn,
) -> crate::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream {
    let location_snake_case = naming::domain_types::LocationSnakeCase;
    let location_new_token_stream = {
        let file_token_stream = generate_quotes::dq_token_stream::dq_token_stream(file.as_str());
        let line_token_stream = {
            let literal = proc_macro2::Literal::u32_unsuffixed(line.value());
            quote::quote! {#literal}
        };
        let column_token_stream = {
            let literal = proc_macro2::Literal::u32_unsuffixed(column.value());
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
                Some(location_lib::occr::Occr {
                    file: location_lib::location_file::LocationFile::try_from(String::from(#file_token_stream)).unwrap_or_else(location_lib::location_file::LocationFile::from),
                    line: location_lib::location_line::LocationLine::try_from(#line_token_stream)
                        .unwrap_or_else(|_error| location_lib::location_line::LocationLine::first()),
                    column: location_lib::location_column::LocationColumn::try_from(#column_token_stream)
                        .unwrap_or_else(|_error| location_lib::location_column::LocationColumn::first()),
                })
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
    fn coordinates_reject_zero_and_accept_positive_values() {
        let _line_error =
            crate::field_location_line::FieldLocationLine::try_from(constants_u32::ZERO)
                .expect_err(constants_str::test_fixtures::VALUE_A7ABF9D9);
        let _column_error =
            crate::field_location_column::FieldLocationColumn::try_from(constants_u32::ZERO)
                .expect_err(constants_str::test_fixtures::VALUE_B430FE14);
        let _line = crate::field_location_line::FieldLocationLine::try_from(7u32).expect(
            "070dbee8 coordinates_reject_zero_and_accept_positive_values invariant must hold",
        );
        let _column = crate::field_location_column::FieldLocationColumn::try_from(11u32).expect(
            "e067c790 coordinates_reject_zero_and_accept_positive_values invariant must hold",
        );
    }
    #[test]
    fn first_coordinates_generate_complete_location_field() {
        let generated = crate::generate_field_location_new_token_stream::generate_field_location_new_token_stream(
            crate::field_location_file::FieldLocationFile::from(constants_str::test_fixtures::VALUE_E1CEB1AF),
            crate::field_location_line::FieldLocationLine::first(),
            crate::field_location_column::FieldLocationColumn::first(),
        )
        .as_ref()
        .to_string();
        assert!(generated.starts_with("location : location_lib :: location :: Location :: new"));
        assert!(generated.contains("\"src/example.rs\""));
        assert!(generated.contains("try_from (1)"));
        assert!(generated.contains("file ! ()"));
        assert!(generated.contains("line ! ()"));
        assert!(generated.contains("column ! ()"));
    }
}
