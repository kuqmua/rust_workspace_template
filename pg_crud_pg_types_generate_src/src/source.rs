// The owner module retains lint-sensitive semantics from the original implementation.
#![allow(
    clippy::arbitrary_source_item_ordering,
    clippy::unreachable,
    clippy::wildcard_enum_match_arm,
    clippy::wildcard_imports
)] // split implementation modules share a private generation vocabulary; declaration/re-export pairs retain dependency order
#[path = "generate_pg_types_max_len.rs"]
mod generate_pg_types_max_len;
use generate_pg_types_max_len::GENERATE_PG_TYPES_MAX_LEN;
#[path = "rust_type_name.rs"]
mod rust_type_name;
use rust_type_name::*;
#[path = "pg_type_name.rs"]
mod pg_type_name;
use pg_type_name::*;
#[path = "pg_type.rs"]
mod pg_type;
use pg_type::*;
#[path = "wire_kind.rs"]
mod wire_kind;
use wire_kind::*;
#[path = "filter_kind.rs"]
mod filter_kind;
use filter_kind::*;
#[path = "can_be_primary_key.rs"]
mod can_be_primary_key;
use can_be_primary_key::*;
#[path = "pg_sql_name.rs"]
mod pg_sql_name;
use pg_sql_name::*;
#[path = "can_be_nullable.rs"]
mod can_be_nullable;
use can_be_nullable::*;
#[path = "range.rs"]
mod range;
use range::*;
#[path = "pg_type_pattern.rs"]
mod pg_type_pattern;
use pg_type_pattern::*;
#[path = "pg_type_record.rs"]
mod pg_type_record;
use pg_type_record::*;
#[path = "pg_type_record_raw.rs"]
mod pg_type_record_raw;
use pg_type_record_raw::*;
#[path = "generate_pg_type_records.rs"]
mod generate_pg_type_records;
use generate_pg_type_records::*;
#[path = "generate_pg_types.rs"]
mod generate_pg_types;
use generate_pg_types::*;
#[path = "generate_pg_types_config_variant.rs"]
mod generate_pg_types_config_variant;
#[path = "generate_pg_types_length_error.rs"]
mod generate_pg_types_length_error;
use generate_pg_types_config_variant::*;
#[path = "generate_secret_text.rs"]
mod generate_secret_text;
use generate_secret_text::*;
#[path = "generate_pg_types_config.rs"]
mod generate_pg_types_config;
use generate_pg_types_config::*;
#[path = "pg_type_initialization_try_new.rs"]
mod pg_type_initialization_try_new;
use pg_type_initialization_try_new::*;
#[path = "pg_type_impl_try_new_for_de.rs"]
mod pg_type_impl_try_new_for_de;
use pg_type_impl_try_new_for_de::*;
#[path = "pg_type_impl_new_for_deserialize_or_try_new_for_de.rs"]
mod pg_type_impl_new_for_deserialize_or_try_new_for_de;
use pg_type_impl_new_for_deserialize_or_try_new_for_de::*;
#[path = "pg_type_deserialize.rs"]
mod pg_type_deserialize;
use pg_type_deserialize::*;
#[path = "parsed_generate_pg_types_config.rs"]
mod parsed_generate_pg_types_config;
pub use parsed_generate_pg_types_config::*;
#[path = "built_generate_pg_types_model.rs"]
mod built_generate_pg_types_model;
pub use built_generate_pg_types_model::*;
#[path = "validated_generate_pg_types_config.rs"]
mod validated_generate_pg_types_config;
pub use validated_generate_pg_types_config::*;
#[path = "pg_types_model_entry_count.rs"]
mod pg_types_model_entry_count;
pub use pg_types_model_entry_count::*;
#[path = "serde_json_generate_pg_types_error.rs"]
mod serde_json_generate_pg_types_error;
pub use serde_json_generate_pg_types_error::*;
#[path = "generate_pg_types_pipeline_error.rs"]
mod generate_pg_types_pipeline_error;
pub use generate_pg_types_pipeline_error::*;
#[path = "parse_generate_pg_types.rs"]
mod parse_generate_pg_types;
pub use parse_generate_pg_types::*;
#[path = "validate_generate_pg_types.rs"]
mod validate_generate_pg_types;
pub use validate_generate_pg_types::*;
#[path = "build_generate_pg_types.rs"]
mod build_generate_pg_types;
pub use build_generate_pg_types::*;
#[path = "functions.rs"]
mod functions;
pub use functions::*;
#[path = "emit_generate_pg_types.rs"]
mod emit_generate_pg_types;
pub use emit_generate_pg_types::*;

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
