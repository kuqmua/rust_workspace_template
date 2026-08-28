#![allow(
    clippy::arbitrary_source_item_ordering,
    clippy::items_after_statements,
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
mod pg_type_deserialize;
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
pub use crate::build_generate_pg_types::*;
pub use crate::built_generate_pg_types_model::*;
pub(crate) use crate::can_be_nullable::*;
pub(crate) use crate::can_be_primary_key::*;
pub use crate::emit_generate_pg_types::*;
pub(crate) use crate::filter_kind::*;
pub(crate) use crate::generate_pg_type_records::*;
pub(crate) use crate::generate_pg_types::*;
pub(crate) use crate::generate_pg_types_config::*;
pub(crate) use crate::generate_pg_types_config_variant::*;
pub(crate) use crate::generate_pg_types_max_len::GENERATE_PG_TYPES_MAX_LEN;
pub use crate::generate_pg_types_pipeline_error::*;
pub use crate::generate_pg_types_tokens::*;
pub(crate) use crate::generate_secret_text::*;
pub use crate::parse_generate_pg_types::*;
pub use crate::parsed_generate_pg_types_config::*;
pub(crate) use crate::pg_sql_name::*;
pub(crate) use crate::pg_type::*;
pub(crate) use crate::pg_type_deserialize::*;
pub(crate) use crate::pg_type_impl_new_for_deserialize_or_try_new_for_de::*;
pub(crate) use crate::pg_type_impl_try_new_for_de::*;
pub(crate) use crate::pg_type_initialization_try_new::*;
pub(crate) use crate::pg_type_name::*;
pub(crate) use crate::pg_type_pattern::*;
pub(crate) use crate::pg_type_record::*;
pub(crate) use crate::pg_type_record_raw::*;
pub use crate::pg_types_model_entry_count::*;
pub(crate) use crate::range::*;
pub(crate) use crate::rust_type_name::*;
pub use crate::serde_json_generate_pg_types_error::*;
pub use crate::validate_generate_pg_types::*;
pub use crate::validated_generate_pg_types_config::*;
pub(crate) use crate::wire_kind::*;
mod sqlx;
#[cfg(test)]
mod tests;
mod validate_generate_pg_types;
mod validated_generate_pg_types_config;
mod wire_kind;
