#[cfg(test)]
#[allow(clippy::default_numeric_fallback, clippy::indexing_slicing)] // literal JSON assertions mirror the exact serialized OpenAPI wire values
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
loc_macros = { workspace = true }
location = { workspace = true }
pg_crud_cmn = { workspace = true }
pg_types_cmn = { workspace = true }
wh_flts = { workspace = true }
optml = { workspace = true }
schemars = { workspace = true }
to_err_string = { workspace = true }
utoipa = { workspace = true }
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
    #[test]
    fn generated_integer_open_api_schema_has_format_bounds_and_example() {
        let (_, schema) = <pg_types_numeric::I16AsNnInt2Orgn as utoipa::ToSchema>::schema();
        let schema_json = serde_json::to_value(schema).expect("8af67e13");
        assert_eq!(schema_json["type"], "integer");
        assert_eq!(schema_json["format"], "int32");
        assert_eq!(schema_json["minimum"], -32768);
        assert_eq!(schema_json["maximum"], 32767);
        assert_eq!(schema_json["example"], 42);
    }
    #[test]
    fn generated_nullable_open_api_schema_is_nullable() {
        let (_, schema) = <pg_types_numeric::OptI16AsNlInt2Orgn as utoipa::ToSchema>::schema();
        let schema_json = serde_json::to_value(schema).expect("f3b5a711");
        assert_eq!(schema_json["type"], "integer");
        assert_eq!(schema_json["nullable"], true);
    }
    #[test]
    fn generated_uuid_open_api_schema_matches_wire_string() {
        let (_, schema) = <pg_types_text_misc::SqlxTypesUuidUuidAsNnUuidInitByClientOrgn as utoipa::ToSchema>::schema();
        let schema_json = serde_json::to_value(schema).expect("80cb3ea4");
        assert_eq!(schema_json["type"], "string");
        assert_eq!(schema_json["format"], "uuid");
    }
    #[test]
    fn std_bound_wire_shape_is_stable_for_range_schemas() {
        assert_eq!(
            serde_json::to_value(std::ops::Bound::Included(1i32)).expect("90cdfba3"),
            serde_json::json!({"Included": 1})
        );
        assert_eq!(
            serde_json::to_value(std::ops::Bound::<i32>::Unbounded).expect("2e7bd0da"),
            serde_json::json!("Unbounded")
        );
    }
    #[test]
    fn generated_time_open_api_properties_match_wire_object() {
        let time = pg_types_chrono_net::SqlxTypesChronoNaiveTimeAsNnTimeOrgn::try_new(
            chrono::NaiveTime::from_hms_micro_opt(12, 34, 56, 789).expect("c19f58a4"),
        )
        .expect("68c0e12b");
        let wire = serde_json::to_value(time).expect("de790942");
        let (_, schema) =
            <pg_types_chrono_net::SqlxTypesChronoNaiveTimeAsNnTimeOrgn as utoipa::ToSchema>::schema(
            );
        let schema_json = serde_json::to_value(schema).expect("dc191318");
        let wire_obj = wire.as_object().expect("e7150f4c");
        let schema_props = schema_json["properties"].as_object().expect("85098dc5");
        assert!(wire_obj.keys().all(|key| schema_props.contains_key(key)));
        assert_eq!(schema_json["required"].as_array().map(Vec::len), Some(4));
    }
    #[test]
    fn generated_range_open_api_properties_match_wire_object() {
        let range = pg_types_numeric::SqlxPgTypesPgRangeI32AsNnInt4RangeOrgn::try_new(
            sqlx::postgres::types::PgRange {
                start: std::ops::Bound::Included(1),
                end: std::ops::Bound::Excluded(3),
            },
        )
        .expect("760545b6");
        let wire = serde_json::to_value(range).expect("290b56bb");
        let (_, schema) =
            <pg_types_numeric::SqlxPgTypesPgRangeI32AsNnInt4RangeOrgn as utoipa::ToSchema>::schema(
            );
        let schema_json = serde_json::to_value(schema).expect("72860bf4");
        let wire_obj = wire.as_object().expect("06a340b9");
        let schema_props = schema_json["properties"].as_object().expect("3dc31cc6");
        assert!(wire_obj.keys().all(|key| schema_props.contains_key(key)));
        assert_eq!(
            schema_json["properties"]["start"]["oneOf"]
                .as_array()
                .map(Vec::len),
            Some(3)
        );
    }
    #[test]
    fn generated_filter_has_open_api_one_of_schema() {
        let (_, schema) = <pg_types_numeric::I16AsNnInt2Wh as utoipa::ToSchema>::schema();
        let schema_json = serde_json::to_value(schema).expect("4bbd5367");
        assert!(
            schema_json["oneOf"]
                .as_array()
                .is_some_and(|vrts| !vrts.is_empty())
        );
    }
}
