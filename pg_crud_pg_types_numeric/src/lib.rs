proc_macro_generate_pg_types::generate_pg_types!({
    "pg_table_cols_write_into_file": "False",
    "whole_write_into_file": "False",
    "variant": {
        "Subset": [
            "I16AsInt2",
            "I32AsInt4",
            "I64AsInt8",
            "F32AsFloat4",
            "F64AsFloat8",
            "I16AsSmallSerialInitializationByPg",
            "I32AsSerialInitializationByPg",
            "I64AsBigSerialInitializationByPg",
            "SqlxPgTypesPgMoneyAsMoney",
            "BoolAsBool",
            "SqlxPgTypesPgRangeI32AsInt4Range",
            "SqlxPgTypesPgRangeI64AsInt8Range"
        ]
    }
});

#[cfg(test)]
mod test_generated_read_query_part_overflow;

#[cfg(feature = "test-utils")]
#[cfg(test)]
mod test_generated_comparison_cases;

#[cfg(test)]
mod test_generated_equality_filter_sql_and_nullable_counters;

#[cfg(test)]
mod test_between_json_contract;
