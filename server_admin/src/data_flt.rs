#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone)]
pub(crate) enum DataFlt {
    Permissions(crate::data_permissions_flt::DataPermissionsFlt),
    RolePermissions(crate::data_role_permissions_flt::DataRolePermissionsFlt),
    Roles(crate::data_roles_flt::DataRolesFlt),
    SystemSettings(crate::data_system_settings_flt::DataSystemSettingsFlt),
    UserRoles(crate::data_user_roles_flt::DataUserRolesFlt),
    Users(crate::data_users_flt::DataUsersFlt),
}

impl DataFlt {
    pub(crate) fn query_bind(
        self,
        sqlx_postgres_query: pg_crud_common::sqlx_postgres_query::SqlxPostgresQuery<'_>,
    ) -> Result<
        pg_crud_common::sqlx_postgres_query::SqlxPostgresQuery<'_>,
        pg_crud_common::sqlx_postgres_query_bind_error::SqlxPostgresQueryBindError,
    > {
        match self {
            Self::Permissions(value) => {
                pg_crud_common::pg_type_where_filter::PgTypeWhereFilter::query_bind(
                    value.into_inner(),
                    sqlx_postgres_query,
                )
            }
            Self::RolePermissions(value) => {
                pg_crud_common::pg_type_where_filter::PgTypeWhereFilter::query_bind(
                    value.into_inner(),
                    sqlx_postgres_query,
                )
            }
            Self::Roles(value) => {
                pg_crud_common::pg_type_where_filter::PgTypeWhereFilter::query_bind(
                    value.into_inner(),
                    sqlx_postgres_query,
                )
            }
            Self::SystemSettings(value) => {
                pg_crud_common::pg_type_where_filter::PgTypeWhereFilter::query_bind(
                    value.into_inner(),
                    sqlx_postgres_query,
                )
            }
            Self::UserRoles(value) => {
                pg_crud_common::pg_type_where_filter::PgTypeWhereFilter::query_bind(
                    value.into_inner(),
                    sqlx_postgres_query,
                )
            }
            Self::Users(value) => {
                pg_crud_common::pg_type_where_filter::PgTypeWhereFilter::query_bind(
                    value.into_inner(),
                    sqlx_postgres_query,
                )
            }
        }
    }
    pub(crate) fn query_part(
        &self,
        query_part_increment: &mut pg_crud_common::query_part_increment::QueryPartIncrement,
    ) -> Result<
        pg_crud_common::query_part_fragment::QueryPartFragment,
        pg_crud_common::query_part_error::QueryPartError,
    > {
        let column = constants_str::PG_CRUD_EMPTY_SQL_SUFFIX;
        match self {
            Self::Permissions(value) => {
                pg_crud_common::pg_type_where_filter::PgTypeWhereFilter::query_part(
                    value.get_inner(),
                    query_part_increment,
                    pg_crud_common::sql_column_ref::SqlColumnRef::from(&column),
                    pg_crud_common::add_operator::AddOperator::from(false),
                )
            }
            Self::RolePermissions(value) => {
                pg_crud_common::pg_type_where_filter::PgTypeWhereFilter::query_part(
                    value.get_inner(),
                    query_part_increment,
                    pg_crud_common::sql_column_ref::SqlColumnRef::from(&column),
                    pg_crud_common::add_operator::AddOperator::from(false),
                )
            }
            Self::Roles(value) => {
                pg_crud_common::pg_type_where_filter::PgTypeWhereFilter::query_part(
                    value.get_inner(),
                    query_part_increment,
                    pg_crud_common::sql_column_ref::SqlColumnRef::from(&column),
                    pg_crud_common::add_operator::AddOperator::from(false),
                )
            }
            Self::SystemSettings(value) => {
                pg_crud_common::pg_type_where_filter::PgTypeWhereFilter::query_part(
                    value.get_inner(),
                    query_part_increment,
                    pg_crud_common::sql_column_ref::SqlColumnRef::from(&column),
                    pg_crud_common::add_operator::AddOperator::from(false),
                )
            }
            Self::UserRoles(value) => {
                pg_crud_common::pg_type_where_filter::PgTypeWhereFilter::query_part(
                    value.get_inner(),
                    query_part_increment,
                    pg_crud_common::sql_column_ref::SqlColumnRef::from(&column),
                    pg_crud_common::add_operator::AddOperator::from(false),
                )
            }
            Self::Users(value) => {
                pg_crud_common::pg_type_where_filter::PgTypeWhereFilter::query_part(
                    value.get_inner(),
                    query_part_increment,
                    pg_crud_common::sql_column_ref::SqlColumnRef::from(&column),
                    pg_crud_common::add_operator::AddOperator::from(false),
                )
            }
        }
    }
}
