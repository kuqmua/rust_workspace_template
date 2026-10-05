#[cfg(test)]
mod test_pipeline_tests {
    #[test]
    fn test_config_builds_and_validates_without_emitting_source() {
        let input = where_filter_generation_input();
        let parsed = crate::parse_generate_where_filters::parse_generate_where_filters(
            crate::proc_macro2_generate_where_filters_input::ProcMacro2GenerateWhereFiltersInput::from(input.as_ref()),
        )
        .expect(constants_str::DIAGNOSTIC_4FB319D6);
        let built = crate::build_generate_where_filters::build_generate_where_filters(parsed)
            .expect(constants_str::DIAGNOSTIC_98C270EA);
        let _validated =
            crate::validate_generate_where_filters::validate_generate_where_filters(built)
                .expect(constants_str::DIAGNOSTIC_E61243AF);
    }

    fn where_filter_generation_input()
    -> macro_helpers::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream
    {
        macro_helpers::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream::from(quote::quote! {{
            "pg_types_write_into_file": "False",
            "whole_write_into_file": "False"
        }})
    }

    #[test]
    fn test_where_filter_generation_rejects_invalid_input_with_parse_diagnostic() {
        [quote::quote! {}, quote::quote! {{}}, quote::quote! {[]}, quote::quote! {null}, quote::quote! {true}]
            .into_iter().fold((), |(), input| {
                assert!(crate::parse_generate_where_filters::parse_generate_where_filters(
                    crate::proc_macro2_generate_where_filters_input::ProcMacro2GenerateWhereFiltersInput::from(&input),
                ).is_err_and(|error| {
                    let is_parse_error = matches!(&error, crate::generate_where_filters_pipeline_error::GenerateWhereFiltersPipelineError::Parse(_));
                    let message = error.to_string();
                    let generated = crate::generate_where_filters_source::generate_where_filters_source(
                        crate::proc_macro2_generate_where_filters_input::ProcMacro2GenerateWhereFiltersInput::from(&input),
                    );
                    is_parse_error && generated.to_string() == quote::quote! {compile_error!(#message);}.to_string()
                }));
            });
    }

    #[test]
    fn test_where_filter_generation_emits_filter_and_text_search_type_catalog() {
        let input = where_filter_generation_input();
        let generated = crate::generate_where_filters_source::generate_where_filters_source(
            crate::proc_macro2_generate_where_filters_input::ProcMacro2GenerateWhereFiltersInput::from(input.as_ref()),
        );
        let struct_keyword = quote::quote! {struct}.to_string();
        let mut next_is_name = false;
        let names = generated
            .as_ref()
            .clone()
            .into_iter()
            .filter_map(|token| {
                if let proc_macro2::TokenTree::Ident(identifier) = token {
                    let is_name = next_is_name;
                    next_is_name = identifier == struct_keyword;
                    is_name.then(|| identifier.to_string())
                } else {
                    next_is_name = false;
                    None
                }
            })
            .collect::<Vec<_>>();
        let expected = quote::quote! {TextSearchMaximumInputBytes TextSearchPolicy TextSearchPattern PgTypeWhereTextSearch PgTypeWhereEq PgTypeWhereGreaterThan PgTypeWhereBetween PgTypeWhereIn PgTypeWhereRegex};
        assert!(expected.into_iter().all(|token| {
            if let proc_macro2::TokenTree::Ident(identifier) = token {
                names
                    .iter()
                    .filter(|name| identifier == name.as_str())
                    .count()
                    == constants_usize::ONE
            } else {
                false
            }
        }));
    }

    #[test]
    fn test_where_filter_validation_rejects_explicit_invalid_contract() {
        let input = where_filter_generation_input();
        assert!(crate::parse_generate_where_filters::parse_generate_where_filters(
            crate::proc_macro2_generate_where_filters_input::ProcMacro2GenerateWhereFiltersInput::from(input.as_ref()),
        ).is_ok_and(|parsed| {
            matches!(crate::validate_generate_where_filters::validate_generate_where_filters(
                crate::built_generate_where_filters_model::BuiltGenerateWhereFiltersModel::from((
                    parsed,
                    crate::filter_spec_valid::FilterSpecValid::from(false),
                )),
            ), Err(crate::generate_where_filters_pipeline_error::GenerateWhereFiltersPipelineError::InvalidContract))
        }));
    }

    #[test]
    fn test_where_filter_pipeline_preserves_each_output_flag_without_emitting() {
        [
            (macro_helpers::should_write_token_stream_into_file::ShouldWriteTokenStreamIntoFile::False, macro_helpers::should_write_token_stream_into_file::ShouldWriteTokenStreamIntoFile::False),
            (macro_helpers::should_write_token_stream_into_file::ShouldWriteTokenStreamIntoFile::False, macro_helpers::should_write_token_stream_into_file::ShouldWriteTokenStreamIntoFile::True),
            (macro_helpers::should_write_token_stream_into_file::ShouldWriteTokenStreamIntoFile::True, macro_helpers::should_write_token_stream_into_file::ShouldWriteTokenStreamIntoFile::False),
            (macro_helpers::should_write_token_stream_into_file::ShouldWriteTokenStreamIntoFile::True, macro_helpers::should_write_token_stream_into_file::ShouldWriteTokenStreamIntoFile::True),
        ].into_iter().fold((), |(), (pg_types_flag, whole_flag)| {
            let expected_pg_types = format!("{pg_types_flag:?}");
            let expected_whole = format!("{whole_flag:?}");
            let value = serde_json::json!({
                (quote::quote! {pg_types_write_into_file}.to_string()): expected_pg_types,
                (quote::quote! {whole_write_into_file}.to_string()): expected_whole,
            });
            let input = value.to_string().parse::<proc_macro2::TokenStream>();
            assert!(input.is_ok_and(|tokens| {
                crate::parse_generate_where_filters::parse_generate_where_filters(
                    crate::proc_macro2_generate_where_filters_input::ProcMacro2GenerateWhereFiltersInput::from(&tokens),
                ).and_then(crate::build_generate_where_filters::build_generate_where_filters)
                    .and_then(crate::validate_generate_where_filters::validate_generate_where_filters)
                    .is_ok_and(|validated| {
                        let (pg_types, whole) = crate::parsed_generate_where_filters_config::ParsedGenerateWhereFiltersConfig::from(validated).into_parts();
                        format!("{pg_types:?}") == expected_pg_types && format!("{whole:?}") == expected_whole
                    })
            }));
        });
    }
}
