#[cfg(test)]
mod pipeline_tests {
    #[test]
    fn config_builds_and_validates_without_emitting_source() {
        let input = quote::quote! {{
            "pg_types_write_into_file": "False",
            "whole_write_into_file": "False"
        }};
        let parsed = crate::parse_generate_where_filters::parse_generate_where_filters(
            crate::proc_macro2_generate_where_filters_input::ProcMacro2GenerateWhereFiltersInput::from(&input),
        )
        .expect("4fb319d6 config_builds_and_validates_without_emitting_source invariant must hold");
        let built = crate::build_generate_where_filters::build_generate_where_filters(parsed)
            .expect(
                "98c270ea config_builds_and_validates_without_emitting_source invariant must hold",
            );
        let _validated = crate::validate_generate_where_filters::validate_generate_where_filters(
            built,
        )
        .expect("e61243af config_builds_and_validates_without_emitting_source invariant must hold");
    }
}
