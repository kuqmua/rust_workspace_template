use super::{
    AdminGeneratedTable, AdminPermissions, AdminRolePermissions, AdminRoles, AdminSystemSettings,
    AdminUserRoles, AdminUsers,
};

pub async fn validate_catalog_schema(
    pool: pg_crud_common::domain_types::SqlxPgPoolRef<'_>,
    schema: pg_crud_common::domain_types::DbSchemaNameRef<'_>,
) -> Result<(), pg_crud_common::domain_types::DbSchemaConformanceError> {
    futures::future::try_join_all(AdminGeneratedTable::ALL.into_iter().map(|table| {
        let table_pool = pool;
        let table_schema = schema;
        async move {
            async fn validate_generated_table<Table>(
                pool: pg_crud_common::domain_types::SqlxPgPoolRef<'_>,
                schema: pg_crud_common::domain_types::DbSchemaNameRef<'_>,
            ) -> Result<(), pg_crud_common::domain_types::DbSchemaConformanceError>
            where
                Table: pg_crud_common::domain_types::DbTableSchema,
            {
                pg_crud_common::domain_types::validate_generated_postgres_table::<Table>(
                    pool, schema,
                )
                .await
            }
            match table {
                AdminGeneratedTable::Roles => {
                    validate_generated_table::<AdminRoles>(table_pool, table_schema).await
                }
                AdminGeneratedTable::RolePermissions => {
                    validate_generated_table::<AdminRolePermissions>(table_pool, table_schema).await
                }
                AdminGeneratedTable::Users => {
                    validate_generated_table::<AdminUsers>(table_pool, table_schema).await
                }
                AdminGeneratedTable::Permissions => {
                    validate_generated_table::<AdminPermissions>(table_pool, table_schema).await
                }
                AdminGeneratedTable::SystemSettings => {
                    validate_generated_table::<AdminSystemSettings>(table_pool, table_schema).await
                }
                AdminGeneratedTable::UserRoles => {
                    validate_generated_table::<AdminUserRoles>(table_pool, table_schema).await
                }
            }
        }
    }))
    .await
    .map(|_validated| ())
}
