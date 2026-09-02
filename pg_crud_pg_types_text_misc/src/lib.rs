proc_macro_generate_pg_types::generate_pg_types!({
    "pg_table_cols_write_into_file": "False",
    "whole_write_into_file": "False",
    "generate_secret_text": true,
    "variant": {
        "Subset": [
            "StringAsText",
            "StdVecVecU8AsBytea",
            "SqlxTypesUuidUuidAsUuidV4InitializationByPg",
            "SqlxTypesUuidUuidAsUuidInitializationByClient",
            "SqlxTypesTimeTimeAsTime",
            "SqlxPgTypesPgIntervalAsInterval"
        ]
    }
});
