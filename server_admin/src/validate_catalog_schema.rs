pub async fn validate_catalog_schema(
    sqlx_pg_catalog_pool_ref: pg_crud_common::sqlx_pg_catalog_pool_ref::SqlxPgCatalogPoolRef<'_>,
    db_schema_name_ref: pg_crud_common::db_schema_name_ref::DbSchemaNameRef<'_>,
) -> Result<(), pg_crud_common::db_schema_conformance_error::DbSchemaConformanceError> {
    futures::future::try_join_all(
        crate::admin_generated_table::AdminGeneratedTable::ALL
            .into_iter()
            .map(async |table| {
                    async fn validate_generated_table<Table>(
                        sqlx_pg_catalog_pool_ref: pg_crud_common::sqlx_pg_catalog_pool_ref::SqlxPgCatalogPoolRef<'_>,
                        db_schema_name_ref: pg_crud_common::db_schema_name_ref::DbSchemaNameRef<'_>,
                    ) -> Result<(), pg_crud_common::db_schema_conformance_error::DbSchemaConformanceError>
                    where
                        Table: pg_crud_common::db_table_schema::DbTableSchema,
                    {
                        pg_crud_common::validate_generated_postgres_table::validate_generated_postgres_table::<Table>(
                            sqlx_pg_catalog_pool_ref, db_schema_name_ref,
                        )
                        .await
                    }
                    match table {
                        crate::admin_generated_table::AdminGeneratedTable::AccessSessions => {
                            validate_generated_table::<
                                crate::admin_access_sessions::AdminAccessSessions,
                            >(sqlx_pg_catalog_pool_ref, db_schema_name_ref)
                            .await
                        }
                        crate::admin_generated_table::AdminGeneratedTable::AuditLog => {
                            validate_generated_table::<crate::admin_audit_log::AdminAuditLog>(
                                sqlx_pg_catalog_pool_ref,
                                db_schema_name_ref,
                            )
                            .await
                        }
                        crate::admin_generated_table::AdminGeneratedTable::PermissionActions
                        | crate::admin_generated_table::AdminGeneratedTable::PermissionResourceActions
                        | crate::admin_generated_table::AdminGeneratedTable::PermissionResources
                        | crate::admin_generated_table::AdminGeneratedTable::Rules => {
                            Ok(())
                        }
                        crate::admin_generated_table::AdminGeneratedTable::Roles => {
                            validate_generated_table::<crate::admin_roles::AdminRoles>(
                                sqlx_pg_catalog_pool_ref,
                                db_schema_name_ref,
                            )
                            .await
                        }
                        crate::admin_generated_table::AdminGeneratedTable::RoleRules => {
                            validate_generated_table::<
                                crate::admin_role_rules::AdminRoleRules,
                            >(sqlx_pg_catalog_pool_ref, db_schema_name_ref)
                            .await
                        }
                        crate::admin_generated_table::AdminGeneratedTable::UsersDatabaseRead => {
                            validate_generated_table::<
                                crate::admin_users_database_read::AdminUsersDatabaseRead,
                            >(sqlx_pg_catalog_pool_ref, db_schema_name_ref)
                            .await
                        }
                        crate::admin_generated_table::AdminGeneratedTable::SystemSettings => {
                            validate_generated_table::<
                                crate::admin_system_settings::AdminSystemSettings,
                            >(sqlx_pg_catalog_pool_ref, db_schema_name_ref)
                            .await
                        }
                        crate::admin_generated_table::AdminGeneratedTable::UserRoles => {
                            validate_generated_table::<crate::admin_user_roles::AdminUserRoles>(
                                sqlx_pg_catalog_pool_ref,
                                db_schema_name_ref,
                            )
                            .await
                        }
                    }
            }),
    )
    .await
    .map(|_validated| ())
}
