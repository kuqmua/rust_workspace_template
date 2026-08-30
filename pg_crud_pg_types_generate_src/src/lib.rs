#![allow(
    clippy::arbitrary_source_item_ordering,
    clippy::items_after_statements,
    clippy::unreachable,
    clippy::wildcard_enum_match_arm,
    clippy::wildcard_imports
)] // split implementation modules preserve the generator-stage lint contracts and share a private generation vocabulary

pub mod build_generate_pg_types;
pub mod built_generate_pg_types_model;
pub mod can_be_nullable;
pub mod can_be_primary_key;
pub mod contract_tests;
pub mod domain_types;
pub mod emit_generate_pg_types;
pub mod filter_kind;
pub mod generate_pg_type_records;
pub mod generate_pg_types;
pub mod generate_pg_types_config;
pub mod generate_pg_types_config_variant;
pub mod generate_pg_types_length_error;
pub mod generate_pg_types_max_len;
pub mod generate_pg_types_pipeline_error;
pub mod generate_pg_types_tokens;
pub mod generate_secret_text;
pub mod parse_generate_pg_types;
pub mod parsed_generate_pg_types_config;
pub mod pg_name;
pub mod pg_sql_name;
pub mod pg_type_can_be_nullable;
pub mod pg_type_catalog_kind;
pub mod pg_type_deserialize;
pub mod pg_type_impl_new_for_deserialize_or_try_new_for_de;
pub mod pg_type_impl_try_new_for_de;
pub mod pg_type_initialization_try_new;
pub mod pg_type_name;
pub mod pg_type_pattern;
pub mod pg_type_record;
pub mod pg_type_record_raw;
pub mod pg_type_spec;
pub mod pg_types_model_entry_count;
pub mod range;
pub mod rust_type_name;
pub mod rust_type_wire_kind;
pub mod schema_wire_kind;
pub mod serde_json_generate_pg_types_error;
pub mod sqlx;
#[cfg(test)]
pub mod tests;
pub mod validate_generate_pg_types;
pub mod validated_generate_pg_types_config;
pub mod wire_kind;
