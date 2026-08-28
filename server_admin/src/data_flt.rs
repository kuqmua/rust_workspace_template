use crate::{
    DataPermissionsFlt, DataRolePermissionsFlt, DataRolesFlt, DataSystemSettingsFlt,
    DataUserRolesFlt, DataUsersFlt,
};

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone)]
pub(crate) enum DataFlt {
    Permissions(DataPermissionsFlt),
    RolePermissions(DataRolePermissionsFlt),
    Roles(DataRolesFlt),
    SystemSettings(DataSystemSettingsFlt),
    UserRoles(DataUserRolesFlt),
    Users(DataUsersFlt),
}

impl DataFlt {
    pub(crate) fn query_bind(
        self,
        query: pg_crud_common::domain_types::SqlxPostgresQuery<'_>,
    ) -> Result<
        pg_crud_common::domain_types::SqlxPostgresQuery<'_>,
        pg_crud_common::domain_types::SqlxPostgresQueryBindError,
    > {
        match self {
            Self::Permissions(DataPermissionsFlt(value)) => {
                pg_crud_common::domain_types::PgTypeWhereFilter::query_bind(value, query)
            }
            Self::RolePermissions(DataRolePermissionsFlt(value)) => {
                pg_crud_common::domain_types::PgTypeWhereFilter::query_bind(value, query)
            }
            Self::Roles(DataRolesFlt(value)) => {
                pg_crud_common::domain_types::PgTypeWhereFilter::query_bind(value, query)
            }
            Self::SystemSettings(DataSystemSettingsFlt(value)) => {
                pg_crud_common::domain_types::PgTypeWhereFilter::query_bind(value, query)
            }
            Self::UserRoles(DataUserRolesFlt(value)) => {
                pg_crud_common::domain_types::PgTypeWhereFilter::query_bind(value, query)
            }
            Self::Users(DataUsersFlt(value)) => {
                pg_crud_common::domain_types::PgTypeWhereFilter::query_bind(value, query)
            }
        }
    }
    pub(crate) fn query_part(
        &self,
        increment: &mut pg_crud_common::domain_types::QueryPartIncrement,
    ) -> Result<
        pg_crud_common::domain_types::QueryPartFragment,
        pg_crud_common::domain_types::QueryPartError,
    > {
        let column = constants_str::PG_CRUD_EMPTY_SQL_SUFFIX;
        match self {
            Self::Permissions(value) => {
                pg_crud_common::domain_types::PgTypeWhereFilter::query_part(
                    &value.0,
                    increment,
                    pg_crud_common::domain_types::SqlColumnRef::from(&column),
                    pg_crud_common::domain_types::AddOperator::from(false),
                )
            }
            Self::RolePermissions(value) => {
                pg_crud_common::domain_types::PgTypeWhereFilter::query_part(
                    &value.0,
                    increment,
                    pg_crud_common::domain_types::SqlColumnRef::from(&column),
                    pg_crud_common::domain_types::AddOperator::from(false),
                )
            }
            Self::Roles(value) => pg_crud_common::domain_types::PgTypeWhereFilter::query_part(
                &value.0,
                increment,
                pg_crud_common::domain_types::SqlColumnRef::from(&column),
                pg_crud_common::domain_types::AddOperator::from(false),
            ),
            Self::SystemSettings(value) => {
                pg_crud_common::domain_types::PgTypeWhereFilter::query_part(
                    &value.0,
                    increment,
                    pg_crud_common::domain_types::SqlColumnRef::from(&column),
                    pg_crud_common::domain_types::AddOperator::from(false),
                )
            }
            Self::UserRoles(value) => pg_crud_common::domain_types::PgTypeWhereFilter::query_part(
                &value.0,
                increment,
                pg_crud_common::domain_types::SqlColumnRef::from(&column),
                pg_crud_common::domain_types::AddOperator::from(false),
            ),
            Self::Users(value) => pg_crud_common::domain_types::PgTypeWhereFilter::query_part(
                &value.0,
                increment,
                pg_crud_common::domain_types::SqlColumnRef::from(&column),
                pg_crud_common::domain_types::AddOperator::from(false),
            ),
        }
    }
}
