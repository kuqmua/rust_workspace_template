#![allow(
    clippy::arbitrary_source_item_ordering,
    clippy::items_after_statements,
    clippy::single_call_fn,
    clippy::unreachable,
    clippy::wildcard_enum_match_arm,
    clippy::wildcard_imports
)] // split implementation modules preserve the generator-stage lint contracts and share a private generation vocabulary

mod build_generate_pg_types;
mod built_generate_pg_types_model;
mod can_be_nullable;
mod can_be_primary_key;
mod contract_tests;
pub mod domain_types;
mod emit_generate_pg_types;
mod filter_kind;
mod generate_pg_type_records;
mod generate_pg_types;
mod generate_pg_types_config;
mod generate_pg_types_config_variant;
mod generate_pg_types_length_error;
mod generate_pg_types_max_len;
mod generate_pg_types_pipeline_error;
mod generate_pg_types_tokens;
mod generate_secret_text;
mod parse_generate_pg_types;
mod parsed_generate_pg_types_config;
mod pg_name;
mod pg_sql_name;
mod pg_type;
mod pg_type_can_be_nullable;
mod pg_type_can_be_primary_key;
mod pg_type_deserialize;
mod pg_type_filter_kind;
mod pg_type_impl_new_for_deserialize_or_try_new_for_de;
mod pg_type_impl_try_new_for_de;
mod pg_type_initialization_try_new;
mod pg_type_name;
mod pg_type_pattern;
mod pg_type_record;
mod pg_type_record_raw;
mod pg_type_spec;
mod pg_types_model_entry_count;
mod range;
mod rust_type_name;
mod rust_type_wire_kind;
mod schema_wire_kind;
mod serde_json_generate_pg_types_error;
mod serde_wire_kind;
mod source;
mod sqlx;
#[cfg(test)]
mod tests;
mod validate_generate_pg_types;
mod validated_generate_pg_types_config;
mod wire_kind;

pub(crate) use source::*;
