#[path = "compile_error_message.rs"]
mod compile_error_message;
#[path = "compile_error_token_stream.rs"]
mod compile_error_token_stream;
#[path = "emit_generate_pg_table.rs"]
mod emit_generate_pg_table;
#[path = "generate_pg_table.rs"]
mod generate_pg_table;
#[path = "table_test_names.rs"]
mod table_test_names;

pub use emit_generate_pg_table::emit_generate_pg_table;
pub use generate_pg_table::generate_pg_table;

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
