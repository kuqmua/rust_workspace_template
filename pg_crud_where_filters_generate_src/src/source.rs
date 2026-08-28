pub use crate::build_generate_where_filters::build_generate_where_filters;
pub use crate::built_generate_where_filters_model::BuiltGenerateWhereFiltersModel;
pub use crate::emit_generate_where_filters::emit_generate_where_filters;
pub use crate::generate_where_filters_pipeline_error::GenerateWhereFiltersPipelineError;
pub use crate::generate_where_filters_source::generate_where_filters_source;
pub use crate::parse_generate_where_filters::parse_generate_where_filters;
pub use crate::parsed_generate_where_filters_config::ParsedGenerateWhereFiltersConfig;
pub use crate::proc_macro2_generate_where_filters_input::ProcMacro2GenerateWhereFiltersInput;
pub use crate::proc_macro2_generate_where_filters_token_stream::ProcMacro2GenerateWhereFiltersTokenStream;
pub use crate::serde_json_generate_where_filters_error::SerdeJsonGenerateWhereFiltersError;
pub use crate::validate_generate_where_filters::validate_generate_where_filters;
pub use crate::validated_generate_where_filters_config::ValidatedGenerateWhereFiltersConfig;

#[cfg(test)]
mod pipeline_tests {
    #[test]
    fn config_builds_and_validates_without_emitting_source() {
        let input = quote::quote! {{
            "pg_types_write_into_file": "False",
            "whole_write_into_file": "False"
        }};
        let parsed = super::parse_generate_where_filters(
            super::ProcMacro2GenerateWhereFiltersInput::from(&input),
        )
        .expect("4fb319d6 config_builds_and_validates_without_emitting_source invariant must hold");
        let built = super::build_generate_where_filters(parsed).expect(
            "98c270ea config_builds_and_validates_without_emitting_source invariant must hold",
        );
        let _validated = super::validate_generate_where_filters(built).expect(
            "e61243af config_builds_and_validates_without_emitting_source invariant must hold",
        );
    }
}
