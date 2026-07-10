#![allow(clippy::needless_for_each)] // utoipa 4 OpenApi derive expands iterator callbacks at crate scope
#[allow(clippy::arbitrary_source_item_ordering, clippy::needless_for_each)] // utoipa 4 derives component registration with iterator callbacks
#[derive(Debug, Clone, Copy, gen_pg_tbl::GenPgTbl, optml::Optml)]
#[gen_pg_tbl::gen_pg_tbl_config{{
    "tests_write_into_file": "False",
    "cmn_write_into_file": "False",
    "whole_write_into_file": "False"
}}]
#[gen_pg_tbl::cm_er_vrts{enum CmErVrts{}}]
#[gen_pg_tbl::co_er_vrts{enum CoErVrts{}}]
#[gen_pg_tbl::rm_er_vrts{enum RmErVrts{}}]
#[gen_pg_tbl::ro_er_vrts{enum RoErVrts{}}]
#[gen_pg_tbl::um_er_vrts{enum UmErVrts{}}]
#[gen_pg_tbl::uo_er_vrts{enum UoErVrts{}}]
#[gen_pg_tbl::dm_er_vrts{enum DmErVrts{}}]
#[gen_pg_tbl::dlo_er_vrts{enum DloErVrts{}}]
#[gen_pg_tbl::cmn_er_vrts{enum CmnErVrts{}}]
#[gen_pg_tbl::cm_logic{}]
#[gen_pg_tbl::co_logic{}]
#[gen_pg_tbl::rm_logic{}]
#[gen_pg_tbl::ro_logic{}]
#[gen_pg_tbl::um_logic{}]
#[gen_pg_tbl::uo_logic{}]
#[gen_pg_tbl::dm_logic{}]
#[gen_pg_tbl::dlo_logic{}]
#[gen_pg_tbl::cmn_logic{}]
pub struct TblExample {
    #[gen_pg_tbl_pk]
    pub pk_col: pg_types_text_misc::SqlxTypesUuidUuidAsNnUuidV4InitByPg,
    pub col_0: pg_types_numeric::I16AsNnInt2,
    pub col_1: pg_types_numeric::OptI16AsNlInt2,
    pub col_2: pg_types_numeric::I32AsNnInt4,
}
#[cfg(test)]
#[allow(
    clippy::indexing_slicing,
    clippy::needless_for_each,
    clippy::shadow_reuse,
    clippy::shadow_unrelated
)] // compact recursive JSON assertions keep the generated document structure visible
mod tests {
    fn collect_component_refs(
        value: &serde_json::Value,
        refs: &mut std::collections::BTreeSet<String>,
    ) {
        match value {
            serde_json::Value::Array(values) => values
                .iter()
                .for_each(|value| collect_component_refs(value, refs)),
            serde_json::Value::Object(values) => values.iter().for_each(|(key, value)| {
                if key == "$ref"
                    && let Some(name) = value
                        .as_str()
                        .and_then(|value| value.strip_prefix("#/components/schemas/"))
                {
                    let _inserted = refs.insert(name.to_owned());
                }
                collect_component_refs(value, refs);
            }),
            serde_json::Value::Null
            | serde_json::Value::Bool(_)
            | serde_json::Value::Number(_)
            | serde_json::Value::String(_) => {}
        }
    }
    #[test]
    fn generated_open_api_contains_all_crud_paths() {
        let doc = serde_json::to_value(super::TblExampleOpenApi::open_api()).expect("3176b0d5");
        [
            ("cm", "post"),
            ("co", "post"),
            ("rm", "post"),
            ("ro", "post"),
            ("um", "patch"),
            ("uo", "patch"),
            ("dm", "delete"),
            ("dlo", "delete"),
        ]
        .into_iter()
        .for_each(|(operation, method)| {
            assert!(
                doc.pointer(&format!("/paths/~1tbl_example~1{operation}/{method}"))
                    .is_some()
            );
        });
        let schemas = doc["components"]["schemas"].as_object().expect("95ec6823");
        assert!(!schemas.is_empty());
        let mut refs = std::collections::BTreeSet::new();
        collect_component_refs(&doc, &mut refs);
        let missing_refs = refs
            .iter()
            .filter(|name| !schemas.contains_key(*name))
            .collect::<Vec<_>>();
        assert!(
            missing_refs.is_empty(),
            "missing component schemas: {missing_refs:?}"
        );
    }
}
