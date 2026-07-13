#[cfg(test)]
#[allow(clippy::needless_for_each)] // table-driven assertions avoid repository-forbidden for loops
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
    fn text_search_patterns_escape_reserved_symbols_for_every_mode() {
        let cases = [
            (wh_flts::TextSearchMode::Contains, r"%a\%\_b%"),
            (wh_flts::TextSearchMode::StartsWith, r"a\%\_b%"),
            (wh_flts::TextSearchMode::EndsWith, r"%a\%\_b"),
        ];
        cases.into_iter().for_each(|(mode, expected)| {
            let pattern = wh_flts::build_text_search_pattern("a%_b", mode).expect("bfcd929a");
            assert_eq!(pattern.as_ref(), expected);
        });
    }
    #[test]
    fn text_search_rejects_empty_and_oversized_values() {
        assert_eq!(
            wh_flts::build_text_search_pattern("", wh_flts::TextSearchMode::Contains),
            Err(wh_flts::TextSearchValueEr::Empty)
        );
        let oversized = "a".repeat(wh_flts::TEXT_SEARCH_MAXIMUM_INPUT_BYTES.saturating_add(1usize));
        assert_eq!(
            wh_flts::build_text_search_pattern(
                oversized.as_str(),
                wh_flts::TextSearchMode::Contains
            ),
            Err(wh_flts::TextSearchValueEr::TooLong {
                actual_bytes: oversized.len(),
                maximum_bytes: wh_flts::TEXT_SEARCH_MAXIMUM_INPUT_BYTES,
            })
        );
    }
    #[test]
    fn text_search_query_fragment_uses_ilike_escape_and_ordered_placeholder() {
        let filter = wh_flts::PgTypeWhTextSearch::try_new(
            pg_crud_cmn::Oprtr::And,
            wh_flts::TextSearchMode::Contains,
            "literal%value".to_owned(),
        )
        .expect("20d018ab");
        let mut parameter_index = 4u64;
        let column = "display_name".to_owned();
        let fragment = <wh_flts::PgTypeWhTextSearch as pg_crud_cmn::PgTypeWhFlt>::qp(
            &filter,
            &mut parameter_index,
            pg_crud_cmn::SqlColRef::from(&column),
            pg_crud_cmn::AddOprtr::from(true),
        )
        .expect("509f61f8");
        assert_eq!(fragment.as_ref(), "and display_name ILIKE $5 ESCAPE '\\'");
        assert_eq!(parameter_index, 5u64);
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
