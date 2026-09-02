proc_macro_trait_alias::trait_alias!(
    DebugClonePartialEqAlias = std::fmt::Debug + Clone + PartialEq
);
proc_macro_trait_alias::trait_alias!(
    DebugClonePartialEqSerializeAlias =
        crate::domain_types::DebugClonePartialEqAlias + serde::Serialize
);
proc_macro_trait_alias::trait_alias!(DebugClonePartialEqSerdeAlias = crate::domain_types::DebugClonePartialEqSerializeAlias + for<'__> serde::Deserialize<'__>);
proc_macro_trait_alias::trait_alias!(
    DebugClonePartialEqSerdeDefaultSomeOneAlias = crate::domain_types::DebugClonePartialEqSerdeAlias
        + crate::default_some_one_element::DefaultSomeOneElement
);
proc_macro_trait_alias::trait_alias!(SqlxEncodePgSqlxTypePgAlias = for<'__> sqlx::Encode<'__, sqlx::Postgres> + sqlx::Type<sqlx::Postgres>);
proc_macro_trait_alias::trait_alias!(
    UtoipaToSchemaAndSchemarsJsonSchemaAlias = utoipa::ToSchema + schemars::JsonSchema
);
proc_macro_trait_alias::trait_alias!(
    TableTypeAlias = crate::domain_types::DebugClonePartialEqSerdeDefaultSomeOneAlias
);
proc_macro_trait_alias::trait_alias!(
    CreateAlias = crate::domain_types::DebugClonePartialEqSerdeDefaultSomeOneAlias
);
proc_macro_trait_alias::trait_alias!(
    CreateForQueryAlias = crate::domain_types::DebugClonePartialEqSerializeAlias
        + crate::domain_types::SqlxEncodePgSqlxTypePgAlias
);
proc_macro_trait_alias::trait_alias!(
    SelectAlias = crate::domain_types::DebugClonePartialEqSerdeDefaultSomeOneAlias
);
proc_macro_trait_alias::trait_alias!(WhereAlias = crate::domain_types::DebugClonePartialEqSerdeAlias + for<'__> crate::pg_type_where_filter::PgTypeWhereFilter<'__>);
proc_macro_trait_alias::trait_alias!(
    ReadAlias = crate::domain_types::DebugClonePartialEqSerdeAlias
);
proc_macro_trait_alias::trait_alias!(
    ReadIdsAlias = crate::domain_types::DebugClonePartialEqSerdeAlias
);
proc_macro_trait_alias::trait_alias!(
    ReadInnerAlias = crate::domain_types::DebugClonePartialEqAlias
);
proc_macro_trait_alias::trait_alias!(
    UpdateAlias = crate::domain_types::DebugClonePartialEqSerdeDefaultSomeOneAlias
);
proc_macro_trait_alias::trait_alias!(
    UpdateForQueryAlias = crate::domain_types::DebugClonePartialEqSerializeAlias
);
