#[cfg(test)]
mod tests {
    #[derive(Eq, PartialEq, serde::Deserialize, serde::Serialize)]
    struct JsonContractValue {
        value: Vec<i32>,
    }
    #[test]
    fn shared_json_contract_helper_round_trips_filter_fixture() {
        macros_helpers::json_contract::ensure_json_contract_round_trip::<JsonContractValue>(
            macros_helpers::json_contract::JsonFixtureRef::from(r#"{"value":[1,2]}"#),
        )
        .expect("46f3bec1");
    }
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
