// The owner module retains lint-sensitive semantics from the original implementation.
#![allow(clippy::arbitrary_source_item_ordering)] // split domain modules remain grouped with their facade imports
pub(crate) const DIRECT_GENERATION_REPEAT_COUNT: usize = 5;
pub(crate) const MEASURE_REPEAT_COUNT: usize = 1000;
const RUNNER_MODE_MAX_LEN: usize = 1_024usize;
pub(crate) const SQL_BUILDER_MEASURE_SERIES_COUNT: usize = 5;
const CLEAN_ANSI_TEXT_MAX_LEN: usize = 16_777_216;
#[path = "measurement_name.rs"]
mod measurement_name;
pub(crate) use measurement_name::*;
#[path = "cargo_args.rs"]
mod cargo_args;
pub(crate) use cargo_args::*;
#[path = "stderr_text_ref.rs"]
mod stderr_text_ref;
pub(crate) use stderr_text_ref::*;
#[path = "ansi_text_ref.rs"]
mod ansi_text_ref;
pub(crate) use ansi_text_ref::*;
#[path = "clean_ansi_text.rs"]
mod clean_ansi_text;
pub(crate) use clean_ansi_text::*;
#[path = "memusage_key.rs"]
mod memusage_key;
pub(crate) use memusage_key::*;
#[path = "memusage_row_name.rs"]
mod memusage_row_name;
pub(crate) use memusage_row_name::*;
#[path = "memusage_column_idx.rs"]
mod memusage_column_idx;
pub(crate) use memusage_column_idx::*;
#[path = "memusage_value_ref.rs"]
mod memusage_value_ref;
pub(crate) use memusage_value_ref::*;
#[path = "program_path_ref.rs"]
mod program_path_ref;
pub(crate) use program_path_ref::*;
#[path = "program_args_ref.rs"]
mod program_args_ref;
pub(crate) use program_args_ref::*;
#[path = "memusage_prog_name_ref.rs"]
mod memusage_prog_name_ref;
pub(crate) use memusage_prog_name_ref::*;
#[path = "quote_token_stream_generate_pg_table_measure_input_token_stream.rs"]
mod quote_token_stream_generate_pg_table_measure_input_token_stream;
pub(crate) use quote_token_stream_generate_pg_table_measure_input_token_stream::*;
#[path = "tool_name.rs"]
mod tool_name;
pub(crate) use tool_name::*;
#[path = "tool_path.rs"]
mod tool_path;
pub(crate) use tool_path::*;
#[path = "tool_available.rs"]
mod tool_available;
pub(crate) use tool_available::*;
#[path = "runner_mode.rs"]
mod runner_mode;
pub(crate) use runner_mode::*;
#[path = "allocation_tool.rs"]
mod allocation_tool;
pub(crate) use allocation_tool::*;
#[path = "macro_generation_measurements.rs"]
mod macro_generation_measurements;
pub(crate) use macro_generation_measurements::*;
#[path = "allocation_tools.rs"]
mod allocation_tools;
pub(crate) use allocation_tools::*;
#[path = "strip_ansi_codes.rs"]
mod strip_ansi_codes;
pub(crate) use strip_ansi_codes::*;
#[path = "memusage_heap_value.rs"]
mod memusage_heap_value;
pub(crate) use memusage_heap_value::*;
#[path = "memusage_table_value.rs"]
mod memusage_table_value;
pub(crate) use memusage_table_value::*;

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
