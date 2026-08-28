use super::{
    AdminGeneratedTable, AdminGeneratedTablesValidationError, AdminPermissions,
    AdminRolePermissions, AdminRoles, AdminSystemSettings, AdminUserRoles, AdminUsers,
};

pub async fn validate_catalog_schema(
    pool: pg_crud_common::domain_types::SqlxPgPoolRef<'_>,
    schema: pg_crud_common::domain_types::DbSchemaNameRef<'_>,
) -> Result<(), AdminGeneratedTablesValidationError> {
    futures::future::try_join_all(AdminGeneratedTable::ALL.into_iter().map(|table| {
        let table_pool = pool;
        let table_schema = schema;
        async move {
            async fn validate<Table>(
                pool: pg_crud_common::domain_types::SqlxPgPoolRef<'_>,
                schema: pg_crud_common::domain_types::DbSchemaNameRef<'_>,
            ) -> Result<(), AdminGeneratedTablesValidationError>
            where
                Table: pg_crud_common::domain_types::DbTableSchema,
            {
                pg_crud_common::domain_types::validate_generated_postgres_table::<Table>(
                    pool, schema,
                )
                .await
                .map_err(AdminGeneratedTablesValidationError::from)
            }
            match table {
                AdminGeneratedTable::Roles => {
                    validate::<AdminRoles>(table_pool, table_schema).await
                }
                AdminGeneratedTable::RolePermissions => {
                    validate::<AdminRolePermissions>(table_pool, table_schema).await
                }
                AdminGeneratedTable::Users => {
                    validate::<AdminUsers>(table_pool, table_schema).await
                }
                AdminGeneratedTable::Permissions => {
                    validate::<AdminPermissions>(table_pool, table_schema).await
                }
                AdminGeneratedTable::SystemSettings => {
                    validate::<AdminSystemSettings>(table_pool, table_schema).await
                }
                AdminGeneratedTable::UserRoles => {
                    validate::<AdminUserRoles>(table_pool, table_schema).await
                }
            }
        }
    }))
    .await
    .map(|_validated| ())
}
