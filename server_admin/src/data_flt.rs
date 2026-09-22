#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone)]
pub(crate) enum DataFlt {
    AccessSessions(crate::data_access_sessions_flt::DataAccessSessionsFlt),
    AuditLog(crate::data_audit_log_flt::DataAuditLogFlt),
    CleanupStatus(crate::data_cleanup_status_flt::DataCleanupStatusFlt),
    LoginAttempts(crate::data_login_attempts_flt::DataLoginAttemptsFlt),
    Rules(crate::data_rules_flt::DataRulesFlt),
    RateLimits(crate::data_rate_limits_flt::DataRateLimitsFlt),
    RefreshTokens(crate::data_refresh_tokens_flt::DataRefreshTokensFlt),
    RoleRules(crate::data_role_rules_flt::DataRoleRulesFlt),
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
            Self::AccessSessions(value) => {
                pg_crud_common::pg_type_where_filter::PgTypeWhereFilter::query_bind(
                    value.into_inner(),
                    sqlx_postgres_query,
                )
            }
            Self::AuditLog(value) => {
                pg_crud_common::pg_type_where_filter::PgTypeWhereFilter::query_bind(
                    value.into_inner(),
                    sqlx_postgres_query,
                )
            }
            Self::CleanupStatus(value) => {
                pg_crud_common::pg_type_where_filter::PgTypeWhereFilter::query_bind(
                    value.into_inner(),
                    sqlx_postgres_query,
                )
            }
            Self::LoginAttempts(value) => {
                pg_crud_common::pg_type_where_filter::PgTypeWhereFilter::query_bind(
                    value.into_inner(),
                    sqlx_postgres_query,
                )
            }
            Self::Rules(value) => {
                pg_crud_common::pg_type_where_filter::PgTypeWhereFilter::query_bind(
                    value.into_inner(),
                    sqlx_postgres_query,
                )
            }
            Self::RateLimits(value) => {
                pg_crud_common::pg_type_where_filter::PgTypeWhereFilter::query_bind(
                    value.into_inner(),
                    sqlx_postgres_query,
                )
            }
            Self::RefreshTokens(value) => {
                pg_crud_common::pg_type_where_filter::PgTypeWhereFilter::query_bind(
                    value.into_inner(),
                    sqlx_postgres_query,
                )
            }
            Self::RoleRules(value) => {
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
            Self::AccessSessions(value) => {
                pg_crud_common::pg_type_where_filter::PgTypeWhereFilter::query_part(
                    value.get_inner(),
                    query_part_increment,
                    pg_crud_common::sql_column_ref::SqlColumnRef::from(&column),
                    pg_crud_common::add_operator::AddOperator::from(false),
                )
            }
            Self::AuditLog(value) => {
                pg_crud_common::pg_type_where_filter::PgTypeWhereFilter::query_part(
                    value.get_inner(),
                    query_part_increment,
                    pg_crud_common::sql_column_ref::SqlColumnRef::from(&column),
                    pg_crud_common::add_operator::AddOperator::from(false),
                )
            }
            Self::CleanupStatus(value) => {
                pg_crud_common::pg_type_where_filter::PgTypeWhereFilter::query_part(
                    value.get_inner(),
                    query_part_increment,
                    pg_crud_common::sql_column_ref::SqlColumnRef::from(&column),
                    pg_crud_common::add_operator::AddOperator::from(false),
                )
            }
            Self::LoginAttempts(value) => {
                pg_crud_common::pg_type_where_filter::PgTypeWhereFilter::query_part(
                    value.get_inner(),
                    query_part_increment,
                    pg_crud_common::sql_column_ref::SqlColumnRef::from(&column),
                    pg_crud_common::add_operator::AddOperator::from(false),
                )
            }
            Self::Rules(value) => {
                pg_crud_common::pg_type_where_filter::PgTypeWhereFilter::query_part(
                    value.get_inner(),
                    query_part_increment,
                    pg_crud_common::sql_column_ref::SqlColumnRef::from(&column),
                    pg_crud_common::add_operator::AddOperator::from(false),
                )
            }
            Self::RateLimits(value) => {
                pg_crud_common::pg_type_where_filter::PgTypeWhereFilter::query_part(
                    value.get_inner(),
                    query_part_increment,
                    pg_crud_common::sql_column_ref::SqlColumnRef::from(&column),
                    pg_crud_common::add_operator::AddOperator::from(false),
                )
            }
            Self::RefreshTokens(value) => {
                pg_crud_common::pg_type_where_filter::PgTypeWhereFilter::query_part(
                    value.get_inner(),
                    query_part_increment,
                    pg_crud_common::sql_column_ref::SqlColumnRef::from(&column),
                    pg_crud_common::add_operator::AddOperator::from(false),
                )
            }
            Self::RoleRules(value) => {
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
