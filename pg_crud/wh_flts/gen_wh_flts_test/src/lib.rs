#[cfg(test)]
mod tests {
    #[test]
    fn clippy() {
        macro_clippy_check_cmn::clippy_check(
            "gen_wh_flts_test_cnt",
            "../pg_crud/wh_flts/",
            "[dependencies]
sqlx = { workspace = true }
serde = { workspace = true }
schemars = { workspace = true }
utoipa = { workspace = true }
loc_lib = { workspace = true }
loc_macros = { workspace = true }
location = { workspace = true }
pg_crud_cmn = { workspace = true }
wh_flts = { workspace = true }
to_err_string = { workspace = true }
[features]
test-utils = []",
            &format!(
                "#![allow(dead_code)]\n#![allow(unreachable_pub)]\n#![allow(unused_imports)]\n#[allow(clippy::wildcard_imports)]\nuse wh_flts::*;\n{}",
                gen_wh_flts_src::gen_wh_flts(gen_wh_flts_src::ProcMacro2GenWhFltsInput::from(
                    &quote::quote! {
                        {
                            "pg_types_write_into_file": "False",
                            "whole_write_into_file": "False"
                        }
                    }
                ))
            ),
        );
    }
}
