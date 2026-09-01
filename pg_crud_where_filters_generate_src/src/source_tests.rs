#[cfg(test)]
mod test_pipeline_tests {
    #[test]
    fn test_config_builds_and_validates_without_emitting_source() {
        let input = quote::quote! {{
            "pg_types_write_into_file": "False",
            "whole_write_into_file": "False"
        }};
        let parsed = crate::parse_generate_where_filters::parse_generate_where_filters(
            crate::proc_macro2_generate_where_filters_input::ProcMacro2GenerateWhereFiltersInput::from(&input),
        )
        .expect(constants_str::DIAGNOSTIC_4FB319D6);
        let built = crate::build_generate_where_filters::build_generate_where_filters(parsed)
            .expect(constants_str::DIAGNOSTIC_98C270EA);
        let _validated =
            crate::validate_generate_where_filters::validate_generate_where_filters(built)
                .expect(constants_str::DIAGNOSTIC_E61243AF);
    }
}
