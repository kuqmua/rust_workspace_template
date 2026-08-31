pg_crud_common_macros::trait_alias!(DebugClonePartialEqAlias = std::fmt::Debug + Clone + PartialEq);
pg_crud_common_macros::trait_alias!(
    DebugClonePartialEqSerializeAlias =
        crate::domain_types::DebugClonePartialEqAlias + serde::Serialize
);
pg_crud_common_macros::trait_alias!(DebugClonePartialEqSerdeAlias = crate::domain_types::DebugClonePartialEqSerializeAlias + for<'__> serde::Deserialize<'__>);
pg_crud_common_macros::trait_alias!(
    DebugClonePartialEqSerdeDefaultSomeOneAlias = crate::domain_types::DebugClonePartialEqSerdeAlias
        + crate::default_some_one_element::DefaultSomeOneElement
);
pg_crud_common_macros::trait_alias!(SqlxEncodePgSqlxTypePgAlias = for<'__> sqlx::Encode<'__, sqlx::Postgres> + sqlx::Type<sqlx::Postgres>);
pg_crud_common_macros::trait_alias!(
    UtoipaToSchemaAndSchemarsJsonSchemaAlias = utoipa::ToSchema + schemars::JsonSchema
);
pg_crud_common_macros::trait_alias!(
    TableTypeAlias = crate::domain_types::DebugClonePartialEqSerdeDefaultSomeOneAlias
);
pg_crud_common_macros::trait_alias!(
    CreateAlias = crate::domain_types::DebugClonePartialEqSerdeDefaultSomeOneAlias
);
pg_crud_common_macros::trait_alias!(
    CreateForQueryAlias = crate::domain_types::DebugClonePartialEqSerializeAlias
        + crate::domain_types::SqlxEncodePgSqlxTypePgAlias
);
pg_crud_common_macros::trait_alias!(
    SelectAlias = crate::domain_types::DebugClonePartialEqSerdeDefaultSomeOneAlias
);
pg_crud_common_macros::trait_alias!(WhereAlias = crate::domain_types::DebugClonePartialEqSerdeAlias + for<'__> crate::pg_type_where_filter::PgTypeWhereFilter<'__>);
pg_crud_common_macros::trait_alias!(ReadAlias = crate::domain_types::DebugClonePartialEqSerdeAlias);
pg_crud_common_macros::trait_alias!(
    ReadIdsAlias = crate::domain_types::DebugClonePartialEqSerdeAlias
);
pg_crud_common_macros::trait_alias!(ReadInnerAlias = crate::domain_types::DebugClonePartialEqAlias);
pg_crud_common_macros::trait_alias!(
    UpdateAlias = crate::domain_types::DebugClonePartialEqSerdeDefaultSomeOneAlias
);
pg_crud_common_macros::trait_alias!(
    UpdateForQueryAlias = crate::domain_types::DebugClonePartialEqSerializeAlias
);
