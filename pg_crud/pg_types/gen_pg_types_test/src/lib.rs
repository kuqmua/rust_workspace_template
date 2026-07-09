#[cfg(test)]
mod tests {
    #[test]
    fn clippy() {
        macro_clippy_check_cmn::clippy_check(
            "gen_pg_types_test_cnt",
            "../pg_crud/pg_types/",
            "[dependencies]
chrono = { workspace = true }
uuid = { workspace = true }
sqlx = { workspace = true }
serde = { workspace = true }
thiserror = { workspace = true }
loc_lib = { workspace = true }
pg_crud_cmn = { workspace = true }
pg_types_cmn = { workspace = true }
wh_flts = { workspace = true }
optml = { workspace = true }
[features]
test-utils = []",
            &gen_pg_types_src::gen_pg_types(macros_helpers::ts_writer::ProcMacro2TsRef::from(
                &quote::quote! {
                    {
                        "pg_tbl_cols_write_into_file": "False",
                        "whole_write_into_file": "False",
                        "vrt": "All"
                    }
                },
            ))
            .to_string(),
        );
    }
}
