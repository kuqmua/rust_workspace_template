#[path = "source/build_generate_where_filters.rs"]
mod build_generate_where_filters;
#[path = "source/built_generate_where_filters_model.rs"]
mod built_generate_where_filters_model;
#[path = "source/emit_generate_where_filters.rs"]
mod emit_generate_where_filters;
#[path = "source/generate_where_filters.rs"]
mod generate_where_filters;
#[path = "source/generate_where_filters_pipeline_error.rs"]
mod generate_where_filters_pipeline_error;
#[path = "source/parse_generate_where_filters.rs"]
mod parse_generate_where_filters;
#[path = "source/parsed_generate_where_filters_config.rs"]
mod parsed_generate_where_filters_config;
#[path = "source/proc_macro2_generate_where_filters_input.rs"]
mod proc_macro2_generate_where_filters_input;
#[path = "source/proc_macro2_generate_where_filters_token_stream.rs"]
mod proc_macro2_generate_where_filters_token_stream;
#[path = "source/serde_json_generate_where_filters_error.rs"]
mod serde_json_generate_where_filters_error;
#[path = "source/validate_generate_where_filters.rs"]
mod validate_generate_where_filters;
#[path = "source/validated_generate_where_filters_config.rs"]
mod validated_generate_where_filters_config;

pub use build_generate_where_filters::build_generate_where_filters;
pub use built_generate_where_filters_model::BuiltGenerateWhereFiltersModel;
pub use emit_generate_where_filters::emit_generate_where_filters;
pub use generate_where_filters::generate_where_filters;
pub use generate_where_filters_pipeline_error::GenerateWhereFiltersPipelineError;
pub use parse_generate_where_filters::parse_generate_where_filters;
pub use parsed_generate_where_filters_config::ParsedGenerateWhereFiltersConfig;
pub use proc_macro2_generate_where_filters_input::ProcMacro2GenerateWhereFiltersInput;
pub use proc_macro2_generate_where_filters_token_stream::ProcMacro2GenerateWhereFiltersTokenStream;
pub use serde_json_generate_where_filters_error::SerdeJsonGenerateWhereFiltersError;
pub use validate_generate_where_filters::validate_generate_where_filters;
pub use validated_generate_where_filters_config::ValidatedGenerateWhereFiltersConfig;

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
