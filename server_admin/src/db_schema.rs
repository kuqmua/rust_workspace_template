pub fn admin_catalog_snapshot(
    schema: super::StdAdminStrRef<'_>,
) -> Result<pg_crud_common::DbCatalogSnapshot, pg_crud_common::DbSchemaTextTryFromStringError> {
    fn snapshot(
        name: &str,
        kind: pg_crud_common::DbObjectKind,
        definition: String,
    ) -> Result<pg_crud_common::DbObjectSnapshot, pg_crud_common::DbSchemaTextTryFromStringError>
    {
        Ok(pg_crud_common::DbObjectSnapshot::new(
            pg_crud_common::DbSchemaText::try_from(name.to_owned())?,
            kind,
            pg_crud_common::DbSchemaText::try_from(definition)?,
        ))
    }
    let updated_at_function = format!(
        "CREATE OR REPLACE FUNCTION {}.set_updated_at()\n RETURNS trigger\n LANGUAGE plpgsql\nAS $function$\nBEGIN\n    NEW.updated_at = NOW();\n    RETURN NEW;\nEND;\n$function$\n",
        schema.0
    );
    let audit_function = format!(
        "CREATE OR REPLACE FUNCTION {}.audit_log_append_only()\n RETURNS trigger\n LANGUAGE plpgsql\nAS $function$\nBEGIN\n    IF TG_OP = 'DELETE' AND current_setting('app.admin_audit_cleanup', TRUE) = 'on' THEN\n        RETURN OLD;\n    END IF;\n    RAISE EXCEPTION 'audit_log is append-only';\nEND;\n$function$\n",
        schema.0
    );
    Ok(pg_crud_common::DbCatalogSnapshot::new(
        vec![
            snapshot(
                str_constants::SERVER_ADMIN_DB_SCHEMA_VALUE_001,
                pg_crud_common::DbObjectKind::Function,
                audit_function,
            )?,
            snapshot(
                str_constants::SERVER_ADMIN_DB_SCHEMA_VALUE_002,
                pg_crud_common::DbObjectKind::Function,
                updated_at_function,
            )?,
            snapshot(
                str_constants::SERVER_ADMIN_DB_SCHEMA_VALUE_003,
                pg_crud_common::DbObjectKind::Trigger,
                str_constants::SERVER_ADMIN_DB_SCHEMA_VALUE_004.to_owned(),
            )?,
            snapshot(
                str_constants::SERVER_ADMIN_DB_SCHEMA_VALUE_003,
                pg_crud_common::DbObjectKind::Trigger,
                str_constants::SERVER_ADMIN_DB_SCHEMA_VALUE_005.to_owned(),
            )?,
            snapshot(
                str_constants::SERVER_ADMIN_DB_SCHEMA_VALUE_006,
                pg_crud_common::DbObjectKind::Trigger,
                str_constants::SERVER_ADMIN_DB_SCHEMA_VALUE_007.to_owned(),
            )?,
            snapshot(
                str_constants::SERVER_ADMIN_DB_SCHEMA_VALUE_008,
                pg_crud_common::DbObjectKind::Trigger,
                str_constants::SERVER_ADMIN_DB_SCHEMA_VALUE_009.to_owned(),
            )?,
            snapshot(
                str_constants::SERVER_ADMIN_DB_SCHEMA_VALUE_010,
                pg_crud_common::DbObjectKind::Trigger,
                str_constants::SERVER_ADMIN_DB_SCHEMA_VALUE_011.to_owned(),
            )?,
        ]
        .into(),
    ))
}

fn default<Column, Expression>(
    column: Column,
    expression: Expression,
) -> pg_crud_common::DbDefaultSpec
where
    Column: Into<pg_crud_common::DbStaticSchemaText>,
    Expression: Into<pg_crud_common::DbStaticSchemaText>,
{
    pg_crud_common::DbDefaultSpec::new(column.into(), expression.into())
}
fn object<Name, Definition>(
    name: Name,
    kind: pg_crud_common::DbObjectKind,
    definition: Definition,
) -> pg_crud_common::DbObjectSpec
where
    Name: Into<pg_crud_common::DbStaticSchemaText>,
    Definition: Into<pg_crud_common::DbStaticSchemaText>,
{
    pg_crud_common::DbObjectSpec::new(name.into(), kind, definition.into())
}

impl pg_crud_common::DbExtendedTableSchema for super::generated_tables::AdminUsers {
    fn exact_defaults() -> pg_crud_common::DbDefaultSpecs {
        vec![
            default(str_constants::IS_BANNED, str_constants::FALSE),
            default(
                str_constants::CREATED_AT,
                str_constants::SERVER_ADMIN_NOW_SQL,
            ),
            default(
                str_constants::UPDATED_AT,
                str_constants::SERVER_ADMIN_NOW_SQL,
            ),
        ]
        .into()
    }
    fn checks_and_indexes() -> pg_crud_common::DbObjectSpecs {
        vec![
            object(
                str_constants::SERVER_ADMIN_DB_SCHEMA_VALUE_015,
                pg_crud_common::DbObjectKind::Check,
                str_constants::SERVER_ADMIN_DB_SCHEMA_VALUE_016,
            ),
            object(
                str_constants::SERVER_ADMIN_DB_SCHEMA_VALUE_017,
                pg_crud_common::DbObjectKind::Check,
                str_constants::SERVER_ADMIN_DB_SCHEMA_VALUE_018,
            ),
            object(
                str_constants::SERVER_ADMIN_DB_SCHEMA_VALUE_019,
                pg_crud_common::DbObjectKind::Check,
                str_constants::SERVER_ADMIN_DB_SCHEMA_VALUE_020,
            ),
            object(
                str_constants::SERVER_ADMIN_DB_SCHEMA_VALUE_021,
                pg_crud_common::DbObjectKind::Check,
                str_constants::SERVER_ADMIN_DB_SCHEMA_VALUE_022,
            ),
            object(
                str_constants::SERVER_ADMIN_DB_SCHEMA_VALUE_023,
                pg_crud_common::DbObjectKind::Check,
                str_constants::SERVER_ADMIN_DB_SCHEMA_VALUE_024,
            ),
            object(
                str_constants::SERVER_ADMIN_DB_SCHEMA_VALUE_025,
                pg_crud_common::DbObjectKind::Index,
                str_constants::SERVER_ADMIN_DB_SCHEMA_VALUE_026,
            ),
        ]
        .into()
    }
}
impl pg_crud_common::DbExtendedTableSchema for super::generated_tables::AdminRoles {
    fn exact_defaults() -> pg_crud_common::DbDefaultSpecs {
        vec![
            default(
                str_constants::SERVER_ADMIN_DB_SCHEMA_VALUE_027,
                str_constants::FALSE,
            ),
            default(
                str_constants::CREATED_AT,
                str_constants::SERVER_ADMIN_NOW_SQL,
            ),
            default(
                str_constants::UPDATED_AT,
                str_constants::SERVER_ADMIN_NOW_SQL,
            ),
        ]
        .into()
    }
    fn checks_and_indexes() -> pg_crud_common::DbObjectSpecs {
        vec![
            object(
                str_constants::SERVER_ADMIN_DB_SCHEMA_VALUE_028,
                pg_crud_common::DbObjectKind::Check,
                str_constants::SERVER_ADMIN_DB_SCHEMA_VALUE_029,
            ),
            object(
                str_constants::SERVER_ADMIN_DB_SCHEMA_VALUE_030,
                pg_crud_common::DbObjectKind::Check,
                str_constants::SERVER_ADMIN_DB_SCHEMA_VALUE_031,
            ),
        ]
        .into()
    }
}
impl pg_crud_common::DbExtendedTableSchema for super::generated_tables::AdminPermissions {
    fn exact_defaults() -> pg_crud_common::DbDefaultSpecs {
        vec![default(
            str_constants::CREATED_AT,
            str_constants::SERVER_ADMIN_NOW_SQL,
        )]
        .into()
    }
    fn checks_and_indexes() -> pg_crud_common::DbObjectSpecs {
        vec![
            object(
                str_constants::SERVER_ADMIN_DB_SCHEMA_VALUE_032,
                pg_crud_common::DbObjectKind::Check,
                str_constants::SERVER_ADMIN_DB_SCHEMA_VALUE_033,
            ),
            object(
                str_constants::SERVER_ADMIN_DB_SCHEMA_VALUE_034,
                pg_crud_common::DbObjectKind::Check,
                str_constants::SERVER_ADMIN_DB_SCHEMA_VALUE_035,
            ),
        ]
        .into()
    }
}
impl pg_crud_common::DbExtendedTableSchema for super::generated_tables::AdminUserRoles {
    fn exact_defaults() -> pg_crud_common::DbDefaultSpecs {
        vec![default(
            str_constants::CREATED_AT,
            str_constants::SERVER_ADMIN_NOW_SQL,
        )]
        .into()
    }
    fn checks_and_indexes() -> pg_crud_common::DbObjectSpecs {
        vec![object(
            str_constants::SERVER_ADMIN_DB_SCHEMA_VALUE_036,
            pg_crud_common::DbObjectKind::Index,
            str_constants::SERVER_ADMIN_DB_SCHEMA_VALUE_037,
        )]
        .into()
    }
}
impl pg_crud_common::DbExtendedTableSchema for super::generated_tables::AdminRolePermissions {
    fn exact_defaults() -> pg_crud_common::DbDefaultSpecs {
        vec![default(
            str_constants::CREATED_AT,
            str_constants::SERVER_ADMIN_NOW_SQL,
        )]
        .into()
    }
    fn checks_and_indexes() -> pg_crud_common::DbObjectSpecs {
        vec![object(
            str_constants::SERVER_ADMIN_DB_SCHEMA_VALUE_038,
            pg_crud_common::DbObjectKind::Index,
            str_constants::SERVER_ADMIN_DB_SCHEMA_VALUE_039,
        )]
        .into()
    }
}
impl pg_crud_common::DbExtendedTableSchema for super::generated_tables::AdminSystemSettings {
    fn exact_defaults() -> pg_crud_common::DbDefaultSpecs {
        vec![
            default(str_constants::SQL_NAMES_ID, str_constants::VALUE_1),
            default(
                str_constants::SERVER_ADMIN_DB_SCHEMA_VALUE_042,
                str_constants::SERVER_ADMIN_DB_SCHEMA_VALUE_043,
            ),
            default(
                str_constants::SERVER_ADMIN_DB_SCHEMA_VALUE_044,
                str_constants::SERVER_ADMIN_DB_SCHEMA_VALUE_045,
            ),
            default(
                str_constants::UPDATED_AT,
                str_constants::SERVER_ADMIN_NOW_SQL,
            ),
        ]
        .into()
    }
    fn checks_and_indexes() -> pg_crud_common::DbObjectSpecs {
        vec![
            object(
                str_constants::SERVER_ADMIN_DB_SCHEMA_VALUE_046,
                pg_crud_common::DbObjectKind::Check,
                str_constants::SERVER_ADMIN_DB_SCHEMA_VALUE_047,
            ),
            object(
                str_constants::SERVER_ADMIN_DB_SCHEMA_VALUE_048,
                pg_crud_common::DbObjectKind::Check,
                str_constants::SERVER_ADMIN_DB_SCHEMA_VALUE_049,
            ),
            object(
                str_constants::SERVER_ADMIN_DB_SCHEMA_VALUE_050,
                pg_crud_common::DbObjectKind::Check,
                str_constants::SERVER_ADMIN_DB_SCHEMA_VALUE_051,
            ),
        ]
        .into()
    }
}
