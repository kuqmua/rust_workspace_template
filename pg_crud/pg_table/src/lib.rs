#![allow(clippy::arbitrary_source_item_ordering)] // SQL helpers stay grouped by generated CRUD concern rather than alphabetically
const PG_TBL_STRING_WRAPPER_MAX_LEN: usize = 1_048_576;
pub trait CombinationOfAppStateLogicTraits:
    config_lib::GetEnableApiGitCommitCheck
    + config_lib::GetMaximumSizeOfHttpBodyInBytes
    + config_lib::GetSrcPlaceType
    + config_lib::GetChronoTimezone
    + app_state::GetSqlxPgPool
    + server_runtime::GetBulkItemResourceBudget
    + server_runtime::GetIdempotencyResponseResourceBudget
    + Send
    + Sync
{
}
const PG_TBL_IDEMPOTENCY_TEXT_MAX_BYTES: usize = 255usize;
const PG_TBL_IDEMPOTENCY_ROUTE_MAX_BYTES: usize = 1024usize;
const PG_TBL_IDEMPOTENCY_RESPONSE_MAX_BYTES: usize = 1_048_576usize;
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PgTableIdempotencyActor(String);
#[derive(Clone, Debug, Eq, PartialEq, newtype::AsRefStr)]
pub struct PgTableIdempotencyKey(String);
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PgTableIdempotencyMethod(String);
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PgTableIdempotencyRoute(String);
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[derive(newtype::FromInner)]
pub struct PgTableIdempotencyRequestHash([u8; 32usize]);
#[derive(Clone, Debug, Eq, PartialEq, newtype::AsRefTarget)]
pub struct PgTableIdempotencyBody(Vec<u8>);
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PgTableIdempotencyBodyError;
impl std::fmt::Display for PgTableIdempotencyBodyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(str_constants::IDEMPOTENCY_RESPONSE_EXCEEDS_THE_STORAGE_LIMIT)
    }
}
impl std::error::Error for PgTableIdempotencyBodyError {}
impl TryFrom<Vec<u8>> for PgTableIdempotencyBody {
    type Error = PgTableIdempotencyBodyError;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        if value.len() > PG_TBL_IDEMPOTENCY_RESPONSE_MAX_BYTES {
            Err(PgTableIdempotencyBodyError)
        } else {
            Ok(Self(value))
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, newtype::AsRefInner, newtype::FromInner)]
pub struct PgTableIdempotencyBodyRef<'body_lt>(&'body_lt [u8]);
#[derive(Clone, Copy, Debug, Eq, PartialEq, newtype::FromInner, newtype::IntoInnerFrom)]
pub struct PgTableIdempotencyResponseStatus(u16);
#[derive(Clone, Copy, Debug, Eq, PartialEq, newtype::Display, newtype::FromInner)]
pub struct PgTableIdempotencyTextBytes(usize);
#[derive(Clone, Copy, Debug, Eq, PartialEq, newtype::FromInner)]
pub struct PgTableIdempotencyCleanupRetentionSeconds(i64);
#[derive(Clone, Copy, Debug, Eq, PartialEq, newtype::FromInner)]
pub struct PgTableIdempotencyCleanupBatchSize(i64);
#[derive(Clone, Copy, Debug, Eq, PartialEq, newtype::FromInner, newtype::IntoInnerFrom)]
pub struct PgTableIdempotencyCleanupRows(u64);
#[derive(Debug, newtype::AsMut)]
pub struct SqlxPgTablePgConnectionRef<'connection_lt>(&'connection_lt mut sqlx::PgConnection);
#[derive(Clone, Copy, Debug, Eq, PartialEq, sqlx::Type, newtype::Display)]
#[sqlx(transparent)]
pub struct PgTableRevision(i64);
#[derive(Debug)]
#[derive(newtype::FromInner)]
pub struct StdPgTableRevisionParseIntError(std::num::ParseIntError);
#[derive(Debug)]
pub enum PgTableRevisionTryFromStringError {
    Invalid(StdPgTableRevisionParseIntError),
    Negative,
}
impl std::fmt::Display for PgTableRevisionTryFromStringError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Invalid(_error) => f.write_str(str_constants::REVISION_MUST_BE_A_DECIMAL_INTEGER),
            Self::Negative => f.write_str(str_constants::REVISION_MUST_NOT_BE_NEGATIVE),
        }
    }
}
impl std::error::Error for PgTableRevisionTryFromStringError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Invalid(error) => Some(&error.0),
            Self::Negative => None,
        }
    }
}
impl TryFrom<String> for PgTableRevision {
    type Error = PgTableRevisionTryFromStringError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let parsed = value.parse::<i64>().map_err(|error| {
            PgTableRevisionTryFromStringError::Invalid(StdPgTableRevisionParseIntError(error))
        })?;
        if parsed < 0i64 {
            Err(PgTableRevisionTryFromStringError::Negative)
        } else {
            Ok(Self(parsed))
        }
    }
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PgTableIdempotencyScope {
    route: PgTableIdempotencyRoute,
    method: PgTableIdempotencyMethod,
    key: PgTableIdempotencyKey,
    actor: PgTableIdempotencyActor,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PgTableIdempotencyRequest {
    scope: PgTableIdempotencyScope,
    request_hash: PgTableIdempotencyRequestHash,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PgTableIdempotencyReplay {
    response_body: PgTableIdempotencyBody,
    response_status: PgTableIdempotencyResponseStatus,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PgTableIdempotencyBegin {
    Acquired,
    Conflict,
    InProgress,
    Replay(PgTableIdempotencyReplay),
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PgTableIdempotencyTextError {
    Empty,
    InvalidMethod,
    InvalidRoute,
    TooLong {
        actual_bytes: PgTableIdempotencyTextBytes,
        maximum_bytes: PgTableIdempotencyTextBytes,
    },
}
impl std::fmt::Display for PgTableIdempotencyTextError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => f.write_str(str_constants::IDEMPOTENCY_TEXT_MUST_NOT_BE_EMPTY),
            Self::InvalidMethod => {
                f.write_str(str_constants::IDEMPOTENCY_METHOD_MUST_BE_POST_PATCH_OR_DELETE)
            }
            Self::InvalidRoute => {
                f.write_str(str_constants::IDEMPOTENCY_ROUTE_MUST_START_WITH_A_SLASH)
            }
            Self::TooLong {
                actual_bytes,
                maximum_bytes,
            } => write!(
                f,
                "idempotency text exceeds {maximum_bytes} bytes: got {actual_bytes}"
            ),
        }
    }
}
impl std::error::Error for PgTableIdempotencyTextError {}
#[derive(Debug, newtype::ErrorTransparent)]
pub struct SqlxPgTableIdempotencyError(sqlx::Error);
impl std::fmt::Display for SqlxPgTableIdempotencyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(str_constants::POSTGRESQL_IDEMPOTENCY_OPERATION_FAILED)
    }
}
impl to_err_string::ToErrString for SqlxPgTableIdempotencyError {
    fn to_err_string(&self) -> to_err_string::ToErrStringValue {
        to_err_string::ToErrStringValue::try_from(self.to_string())
            .unwrap_or_else(to_err_string::ToErrStringValue::from)
    }
}
impl From<sqlx::Error> for SqlxPgTableIdempotencyError {
    fn from(value: sqlx::Error) -> Self {
        Self(value)
    }
}
impl TryFrom<String> for PgTableIdempotencyActor {
    type Error = PgTableIdempotencyTextError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            Err(PgTableIdempotencyTextError::Empty)
        } else if value.len() > PG_TBL_IDEMPOTENCY_TEXT_MAX_BYTES {
            Err(PgTableIdempotencyTextError::TooLong {
                actual_bytes: PgTableIdempotencyTextBytes::from(value.len()),
                maximum_bytes: PgTableIdempotencyTextBytes::from(PG_TBL_IDEMPOTENCY_TEXT_MAX_BYTES),
            })
        } else {
            Ok(Self(value))
        }
    }
}
impl TryFrom<String> for PgTableIdempotencyKey {
    type Error = PgTableIdempotencyTextError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            Err(PgTableIdempotencyTextError::Empty)
        } else if value.len() > PG_TBL_IDEMPOTENCY_TEXT_MAX_BYTES {
            Err(PgTableIdempotencyTextError::TooLong {
                actual_bytes: PgTableIdempotencyTextBytes::from(value.len()),
                maximum_bytes: PgTableIdempotencyTextBytes::from(PG_TBL_IDEMPOTENCY_TEXT_MAX_BYTES),
            })
        } else {
            Ok(Self(value))
        }
    }
}
impl From<uuid::Uuid> for PgTableIdempotencyKey {
    fn from(value: uuid::Uuid) -> Self {
        Self(value.to_string())
    }
}
#[must_use]
pub fn new_pg_table_idempotency_key() -> PgTableIdempotencyKey {
    PgTableIdempotencyKey::from(uuid::Uuid::new_v4())
}
impl TryFrom<String> for PgTableIdempotencyMethod {
    type Error = PgTableIdempotencyTextError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(PgTableIdempotencyTextError::Empty);
        }
        if value.len() > PG_TBL_IDEMPOTENCY_TEXT_MAX_BYTES {
            return Err(PgTableIdempotencyTextError::TooLong {
                actual_bytes: PgTableIdempotencyTextBytes::from(value.len()),
                maximum_bytes: PgTableIdempotencyTextBytes::from(PG_TBL_IDEMPOTENCY_TEXT_MAX_BYTES),
            });
        }
        if matches!(
            value.as_str(),
            str_constants::POST | str_constants::PATCH | str_constants::DELETE
        ) {
            Ok(Self(value))
        } else {
            Err(PgTableIdempotencyTextError::InvalidMethod)
        }
    }
}
impl TryFrom<String> for PgTableIdempotencyRoute {
    type Error = PgTableIdempotencyTextError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(PgTableIdempotencyTextError::Empty);
        }
        if value.len() > PG_TBL_IDEMPOTENCY_ROUTE_MAX_BYTES {
            return Err(PgTableIdempotencyTextError::TooLong {
                actual_bytes: PgTableIdempotencyTextBytes::from(value.len()),
                maximum_bytes: PgTableIdempotencyTextBytes::from(
                    PG_TBL_IDEMPOTENCY_ROUTE_MAX_BYTES,
                ),
            });
        }
        if value.starts_with('/') {
            Ok(Self(value))
        } else {
            Err(PgTableIdempotencyTextError::InvalidRoute)
        }
    }
}
impl PgTableIdempotencyScope {
    #[must_use]
    pub const fn new(
        actor: PgTableIdempotencyActor,
        method: PgTableIdempotencyMethod,
        route: PgTableIdempotencyRoute,
        key: PgTableIdempotencyKey,
    ) -> Self {
        Self {
            route,
            method,
            key,
            actor,
        }
    }
}
impl PgTableIdempotencyRequest {
    #[must_use]
    pub fn new(scope: PgTableIdempotencyScope, body: PgTableIdempotencyBodyRef<'_>) -> Self {
        Self {
            scope,
            request_hash: pg_table_idempotency_request_hash(body),
        }
    }
    #[must_use]
    pub const fn scope(&self) -> &PgTableIdempotencyScope {
        &self.scope
    }
}
impl PgTableIdempotencyReplay {
    #[must_use]
    pub fn into_parts(self) -> (PgTableIdempotencyResponseStatus, PgTableIdempotencyBody) {
        (self.response_status, self.response_body)
    }
}
#[must_use]
pub fn pg_table_idempotency_request_hash(
    body: PgTableIdempotencyBodyRef<'_>,
) -> PgTableIdempotencyRequestHash {
    let digest = <sha2::Sha256 as sha2::Digest>::digest(body.0);
    let mut bytes = [0u8; 32usize];
    bytes.copy_from_slice(&digest);
    PgTableIdempotencyRequestHash::from(bytes)
}
pub async fn ensure_pg_table_idempotency_schema(
    pool: app_state::SqlxPgPoolRef<'_>,
) -> Result<(), SqlxPgTableIdempotencyError> {
    let _query_result = sqlx::query(
        str_constants::CREATE_TABLE_IF_NOT_EXISTS_PG_TABLE_IDEMPOTENCY_ACTOR_TEXT_NOT_NULL,
    )
    .execute(pool.as_ref())
    .await
    .map_err(SqlxPgTableIdempotencyError::from)?;
    let _index_result = sqlx::query(
        str_constants::CREATE_INDEX_IF_NOT_EXISTS_PG_TABLE_IDEMPOTENCY_CREATED_AT_IDX_ON,
    )
    .execute(pool.as_ref())
    .await
    .map_err(SqlxPgTableIdempotencyError::from)?;
    Ok(())
}
pub async fn begin_pg_table_idempotency(
    pool: app_state::SqlxPgPoolRef<'_>,
    request: &PgTableIdempotencyRequest,
) -> Result<PgTableIdempotencyBegin, SqlxPgTableIdempotencyError> {
    let inserted = sqlx::query_scalar::<_, bool>(str_constants::INSERT_INTO_PG_TABLE_IDEMPOTENCY_ACTOR_HTTP_METHOD_ROUTE_PATH_IDEMPOTENCY_KEY)
        .bind(request.scope.actor.0.as_str())
        .bind(request.scope.method.0.as_str())
        .bind(request.scope.route.0.as_str())
        .bind(request.scope.key.0.as_str())
        .bind(request.request_hash.0.as_slice())
        .fetch_optional(pool.as_ref())
        .await
        .map_err(SqlxPgTableIdempotencyError::from)?;
    if inserted == Some(true) {
        return Ok(PgTableIdempotencyBegin::Acquired);
    }
    let existing = sqlx::query_as::<_, (Vec<u8>, String, Option<i16>, Option<Vec<u8>>)>(
        str_constants::SELECT_REQUEST_HASH_STATE_RESPONSE_STATUS_RESPONSE_BODY_FROM_PG_TABLE_IDEMPOTENCY,
    )
    .bind(request.scope.actor.0.as_str())
    .bind(request.scope.method.0.as_str())
    .bind(request.scope.route.0.as_str())
    .bind(request.scope.key.0.as_str())
    .fetch_one(pool.as_ref())
    .await
    .map_err(SqlxPgTableIdempotencyError::from)?;
    if existing.0.as_slice() != request.request_hash.0.as_slice() {
        return Ok(PgTableIdempotencyBegin::Conflict);
    }
    if existing.1 == str_constants::PENDING {
        return Ok(PgTableIdempotencyBegin::InProgress);
    }
    match (existing.2, existing.3) {
        (Some(status), Some(raw_response_body)) => {
            let response_status = match u16::try_from(status) {
                Ok(value) => value,
                Err(_error) => return Ok(PgTableIdempotencyBegin::InProgress),
            };
            let response_body = match PgTableIdempotencyBody::try_from(raw_response_body) {
                Ok(value) => value,
                Err(_error) => return Ok(PgTableIdempotencyBegin::InProgress),
            };
            Ok(PgTableIdempotencyBegin::Replay(PgTableIdempotencyReplay {
                response_body,
                response_status: PgTableIdempotencyResponseStatus::from(response_status),
            }))
        }
        (None | Some(_), None) | (None, Some(_)) => Ok(PgTableIdempotencyBegin::InProgress),
    }
}
pub async fn complete_pg_table_idempotency(
    pool: app_state::SqlxPgPoolRef<'_>,
    request: &PgTableIdempotencyRequest,
    response_status: PgTableIdempotencyResponseStatus,
    response_body: PgTableIdempotencyBodyRef<'_>,
) -> Result<(), SqlxPgTableIdempotencyError> {
    if response_body.0.len() > PG_TBL_IDEMPOTENCY_RESPONSE_MAX_BYTES {
        return release_pg_table_idempotency(pool, request).await;
    }
    let response_status_i16 = match i16::try_from(response_status.0) {
        Ok(value) => value,
        Err(_error) => return release_pg_table_idempotency(pool, request).await,
    };
    let _query_result = sqlx::query(str_constants::PG_CRUD_COMPLETE_IDEMPOTENCY_SQL)
        .bind(request.scope.actor.0.as_str())
        .bind(request.scope.method.0.as_str())
        .bind(request.scope.route.0.as_str())
        .bind(request.scope.key.0.as_str())
        .bind(request.request_hash.0.as_slice())
        .bind(response_status_i16)
        .bind(response_body.0)
        .execute(pool.as_ref())
        .await
        .map_err(SqlxPgTableIdempotencyError::from)?;
    Ok(())
}
pub async fn complete_pg_table_idempotency_in_connection(
    mut connection: SqlxPgTablePgConnectionRef<'_>,
    request: &PgTableIdempotencyRequest,
    response_status: PgTableIdempotencyResponseStatus,
    response_body: PgTableIdempotencyBodyRef<'_>,
) -> Result<(), SqlxPgTableIdempotencyError> {
    if response_body.0.len() > PG_TBL_IDEMPOTENCY_RESPONSE_MAX_BYTES {
        return Err(SqlxPgTableIdempotencyError::from(sqlx::Error::Protocol(
            str_constants::IDEMPOTENCY_RESPONSE_EXCEEDS_THE_STORAGE_LIMIT.to_owned(),
        )));
    }
    let response_status_i16 = i16::try_from(response_status.0).map_err(|_error| {
        SqlxPgTableIdempotencyError::from(sqlx::Error::Protocol(
            str_constants::IDEMPOTENCY_RESPONSE_STATUS_IS_OUTSIDE_SMALLINT.to_owned(),
        ))
    })?;
    let result = sqlx::query(str_constants::PG_CRUD_COMPLETE_IDEMPOTENCY_SQL)
        .bind(request.scope.actor.0.as_str())
        .bind(request.scope.method.0.as_str())
        .bind(request.scope.route.0.as_str())
        .bind(request.scope.key.0.as_str())
        .bind(request.request_hash.0.as_slice())
        .bind(response_status_i16)
        .bind(response_body.0)
        .execute(connection.as_mut())
        .await
        .map_err(SqlxPgTableIdempotencyError::from)?;
    if result.rows_affected() == 1u64 {
        Ok(())
    } else {
        Err(SqlxPgTableIdempotencyError::from(sqlx::Error::Protocol(
            str_constants::IDEMPOTENCY_RESERVATION_IS_UNAVAILABLE_FOR_COMPLETION.to_owned(),
        )))
    }
}
pub async fn release_pg_table_idempotency(
    pool: app_state::SqlxPgPoolRef<'_>,
    request: &PgTableIdempotencyRequest,
) -> Result<(), SqlxPgTableIdempotencyError> {
    let _query_result = sqlx::query(
        str_constants::DELETE_FROM_PG_TABLE_IDEMPOTENCY_WHERE_ACTOR_DOLLAR_1_AND_HTTP_METHOD,
    )
    .bind(request.scope.actor.0.as_str())
    .bind(request.scope.method.0.as_str())
    .bind(request.scope.route.0.as_str())
    .bind(request.scope.key.0.as_str())
    .bind(request.request_hash.0.as_slice())
    .execute(pool.as_ref())
    .await
    .map_err(SqlxPgTableIdempotencyError::from)?;
    Ok(())
}
impl<'connection_lt> From<&'connection_lt mut sqlx::PgConnection>
    for SqlxPgTablePgConnectionRef<'connection_lt>
{
    fn from(value: &'connection_lt mut sqlx::PgConnection) -> Self {
        Self(value)
    }
}
pub async fn cleanup_pg_table_idempotency(
    pool: app_state::SqlxPgPoolRef<'_>,
    completed_retention_seconds: PgTableIdempotencyCleanupRetentionSeconds,
    pending_retention_seconds: PgTableIdempotencyCleanupRetentionSeconds,
    batch_size: PgTableIdempotencyCleanupBatchSize,
) -> Result<PgTableIdempotencyCleanupRows, SqlxPgTableIdempotencyError> {
    let result = sqlx::query(
        str_constants::WITH_EXPIRED_AS_SELECT_ACTOR_HTTP_METHOD_ROUTE_PATH_IDEMPOTENCY_KEY_FROM,
    )
    .bind(completed_retention_seconds.0)
    .bind(pending_retention_seconds.0)
    .bind(batch_size.0)
    .execute(pool.as_ref())
    .await
    .map_err(SqlxPgTableIdempotencyError::from)?;
    Ok(PgTableIdempotencyCleanupRows::from(result.rows_affected()))
}
#[cfg(test)]
mod idempotency_tests {
    #[test]
    fn request_hash_is_stable_and_payload_sensitive() {
        let first = super::pg_table_idempotency_request_hash(
            super::PgTableIdempotencyBodyRef::from(b"same payload".as_slice()),
        );
        let second = super::pg_table_idempotency_request_hash(
            super::PgTableIdempotencyBodyRef::from(b"same payload".as_slice()),
        );
        let changed = super::pg_table_idempotency_request_hash(
            super::PgTableIdempotencyBodyRef::from(b"changed payload".as_slice()),
        );
        assert_eq!(first, second);
        assert_ne!(first, changed);
    }
    #[test]
    fn idempotency_text_types_enforce_boundaries_and_protocol_shape() {
        assert_eq!(
            super::PgTableIdempotencyActor::try_from(String::new()),
            Err(super::PgTableIdempotencyTextError::Empty)
        );
        assert_eq!(
            super::PgTableIdempotencyMethod::try_from("GET".to_owned()),
            Err(super::PgTableIdempotencyTextError::InvalidMethod)
        );
        assert_eq!(
            super::PgTableIdempotencyRoute::try_from("without-slash".to_owned()),
            Err(super::PgTableIdempotencyTextError::InvalidRoute)
        );
        let oversized = str_constants::A_ALT
            .repeat(super::PG_TBL_IDEMPOTENCY_TEXT_MAX_BYTES.saturating_add(1usize));
        assert_eq!(
            super::PgTableIdempotencyKey::try_from(oversized.clone()),
            Err(super::PgTableIdempotencyTextError::TooLong {
                actual_bytes: super::PgTableIdempotencyTextBytes::from(oversized.len()),
                maximum_bytes: super::PgTableIdempotencyTextBytes::from(
                    super::PG_TBL_IDEMPOTENCY_TEXT_MAX_BYTES,
                ),
            })
        );
    }
    #[test]
    fn generated_idempotency_keys_are_valid_and_distinct() {
        let first = super::new_pg_table_idempotency_key();
        let second = super::new_pg_table_idempotency_key();
        assert_ne!(first, second);
        assert!(!first.as_ref().is_empty());
        assert!(first.as_ref().len() <= super::PG_TBL_IDEMPOTENCY_TEXT_MAX_BYTES);
    }
    #[test]
    fn persisted_idempotency_body_rejects_values_above_storage_limit() {
        assert_eq!(
            super::PgTableIdempotencyBody::try_from(vec![
                0u8;
                super::PG_TBL_IDEMPOTENCY_RESPONSE_MAX_BYTES
                    + 1usize
            ])
            .map(drop),
            Err(super::PgTableIdempotencyBodyError)
        );
    }
}
#[derive(Clone, Copy)]
enum InsertValuesFmt {
    Raw,
    Wrapped,
}
#[derive(Clone, Copy)]
enum SelectWhereFmt {
    Plain,
    Where,
}
#[derive(Clone, Copy)]
enum UpdateSelectorFmt {
    Eq,
    InList,
}
#[derive(Debug, Clone, Copy, newtype::AsRefInner, newtype::Display)]
pub struct PgTableNameRef<'lt>(&'lt str);
impl<'lt, T> From<&'lt T> for PgTableNameRef<'lt>
where
    T: AsRef<str> + ?Sized,
{
    fn from(value: &'lt T) -> Self {
        Self(value.as_ref())
    }
}
#[derive(Debug, Clone, Copy, newtype::AsRefInner, newtype::Display)]
pub struct PgTableSqlFragmentRef<'lt>(&'lt str);
impl<'lt, T> From<&'lt T> for PgTableSqlFragmentRef<'lt>
where
    T: AsRef<str> + ?Sized,
{
    fn from(value: &'lt T) -> Self {
        Self(value.as_ref())
    }
}
#[derive(Debug, Clone, newtype::DerefTarget, newtype::Display)]
pub struct PgTableQueryString(String);
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PgTableStringWrapperTryFromStringError {
    TooLong { len: usize, max: usize },
}
impl std::fmt::Display for PgTableStringWrapperTryFromStringError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TooLong { len, max } => {
                write!(
                    f,
                    "pg table string wrapper length {len} exceeds maximum {max}"
                )
            }
        }
    }
}
impl From<PgTableStringWrapperTryFromStringError> for PgTableQueryString {
    fn from(value: PgTableStringWrapperTryFromStringError) -> Self {
        Self(value.to_string())
    }
}
impl TryFrom<String> for PgTableQueryString {
    type Error = PgTableStringWrapperTryFromStringError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > PG_TBL_STRING_WRAPPER_MAX_LEN {
            return Err(Self::Error::TooLong {
                len: value.len(),
                max: PG_TBL_STRING_WRAPPER_MAX_LEN,
            });
        }
        Ok(Self(value))
    }
}
#[derive(Debug, Clone, newtype::DerefTarget, newtype::Display)]
pub struct PgTableQueryPartFragment(String);
impl From<PgTableStringWrapperTryFromStringError> for PgTableQueryPartFragment {
    fn from(value: PgTableStringWrapperTryFromStringError) -> Self {
        Self(value.to_string())
    }
}
impl TryFrom<String> for PgTableQueryPartFragment {
    type Error = PgTableStringWrapperTryFromStringError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > PG_TBL_STRING_WRAPPER_MAX_LEN {
            return Err(Self::Error::TooLong {
                len: value.len(),
                max: PG_TBL_STRING_WRAPPER_MAX_LEN,
            });
        }
        Ok(Self(value))
    }
}
fn generate_insert_query_string(
    table: PgTableNameRef<'_>,
    cols: PgTableSqlFragmentRef<'_>,
    values: PgTableSqlFragmentRef<'_>,
    cols_to_return: PgTableSqlFragmentRef<'_>,
    insert_values_fmt: InsertValuesFmt,
) -> PgTableQueryString {
    let wrapper_len = match insert_values_fmt {
        InsertValuesFmt::Raw => 0usize,
        InsertValuesFmt::Wrapped => 2usize,
    };
    let mut query = String::with_capacity(
        34usize
            .saturating_add(table.as_ref().len())
            .saturating_add(cols.as_ref().len())
            .saturating_add(values.as_ref().len())
            .saturating_add(cols_to_return.as_ref().len())
            .saturating_add(wrapper_len),
    );
    query.push_str(str_constants::INSERT_INTO);
    query.push_str(table.as_ref());
    query.push_str(str_constants::TEXT);
    query.push_str(cols.as_ref());
    query.push_str(str_constants::VALUES);
    if matches!(insert_values_fmt, InsertValuesFmt::Wrapped) {
        query.push('(');
    }
    query.push_str(values.as_ref());
    if matches!(insert_values_fmt, InsertValuesFmt::Wrapped) {
        query.push(')');
    }
    query.push_str(str_constants::RETURNING);
    query.push_str(cols_to_return.as_ref());
    PgTableQueryString::try_from(query).unwrap_or_else(PgTableQueryString::from)
}
fn generate_select_query_string(
    table: PgTableNameRef<'_>,
    select_string: PgTableSqlFragmentRef<'_>,
    where_string: PgTableSqlFragmentRef<'_>,
    select_where_fmt: SelectWhereFmt,
) -> PgTableQueryString {
    let where_len = match select_where_fmt {
        SelectWhereFmt::Plain => 1usize,
        SelectWhereFmt::Where => 7usize,
    };
    let mut query = String::with_capacity(
        13usize
            .saturating_add(select_string.as_ref().len())
            .saturating_add(table.as_ref().len())
            .saturating_add(where_string.as_ref().len())
            .saturating_add(where_len),
    );
    query.push_str(str_constants::SELECT_ALT);
    query.push_str(select_string.as_ref());
    query.push_str(str_constants::FROM_ALT);
    query.push_str(table.as_ref());
    match select_where_fmt {
        SelectWhereFmt::Plain => query.push(' '),
        SelectWhereFmt::Where => query.push_str(str_constants::WHERE),
    }
    query.push_str(where_string.as_ref());
    PgTableQueryString::try_from(query).unwrap_or_else(PgTableQueryString::from)
}
fn generate_update_query_string(
    table: PgTableNameRef<'_>,
    cols_or_els: PgTableSqlFragmentRef<'_>,
    primary_key_field_name: PgTableSqlFragmentRef<'_>,
    primary_key_selector: PgTableSqlFragmentRef<'_>,
    cols_to_return: PgTableSqlFragmentRef<'_>,
    update_selector_fmt: UpdateSelectorFmt,
) -> PgTableQueryString {
    let selector_len = match update_selector_fmt {
        UpdateSelectorFmt::Eq => 3usize,
        UpdateSelectorFmt::InList => 6usize,
    };
    let mut query = String::with_capacity(
        30usize
            .saturating_add(table.as_ref().len())
            .saturating_add(cols_or_els.as_ref().len())
            .saturating_add(primary_key_field_name.as_ref().len())
            .saturating_add(primary_key_selector.as_ref().len())
            .saturating_add(cols_to_return.as_ref().len())
            .saturating_add(selector_len),
    );
    query.push_str(str_constants::UPDATE_ALT);
    query.push_str(table.as_ref());
    query.push_str(str_constants::SET);
    query.push_str(cols_or_els.as_ref());
    query.push_str(str_constants::WHERE);
    query.push_str(primary_key_field_name.as_ref());
    match update_selector_fmt {
        UpdateSelectorFmt::Eq => query.push_str(str_constants::TEXT_ALT),
        UpdateSelectorFmt::InList => query.push_str(str_constants::IN),
    }
    query.push_str(primary_key_selector.as_ref());
    if matches!(update_selector_fmt, UpdateSelectorFmt::InList) {
        query.push(')');
    }
    query.push_str(str_constants::RETURNING);
    query.push_str(cols_to_return.as_ref());
    PgTableQueryString::try_from(query).unwrap_or_else(PgTableQueryString::from)
}
fn generate_delete_query_string(
    table: PgTableNameRef<'_>,
    primary_key_field_name: PgTableSqlFragmentRef<'_>,
    where_string: Option<PgTableSqlFragmentRef<'_>>,
) -> PgTableQueryString {
    let where_len = where_string.map_or_else(
        || 12usize.saturating_add(primary_key_field_name.as_ref().len()),
        |v| v.as_ref().len(),
    );
    let mut query = String::with_capacity(
        24usize
            .saturating_add(table.as_ref().len())
            .saturating_add(where_len)
            .saturating_add(primary_key_field_name.as_ref().len()),
    );
    query.push_str(str_constants::DELETE_FROM);
    query.push_str(table.as_ref());
    query.push(' ');
    if let Some(v) = where_string {
        query.push_str(v.as_ref());
    } else {
        query.push_str(str_constants::WHERE_ALT);
        query.push_str(primary_key_field_name.as_ref());
        query.push_str(str_constants::DOLLAR_1);
    }
    query.push_str(str_constants::RETURNING);
    query.push_str(primary_key_field_name.as_ref());
    PgTableQueryString::try_from(query).unwrap_or_else(PgTableQueryString::from)
}
#[must_use]
pub fn generate_cm_query_string(
    table: PgTableNameRef<'_>,
    cols: PgTableSqlFragmentRef<'_>,
    values: PgTableSqlFragmentRef<'_>,
    cols_to_return: PgTableSqlFragmentRef<'_>,
) -> PgTableQueryString {
    generate_insert_query_string(table, cols, values, cols_to_return, InsertValuesFmt::Raw)
}
#[must_use]
pub fn generate_co_query_string(
    table: PgTableNameRef<'_>,
    cols: PgTableSqlFragmentRef<'_>,
    values: PgTableSqlFragmentRef<'_>,
    cols_to_return: PgTableSqlFragmentRef<'_>,
) -> PgTableQueryString {
    generate_insert_query_string(
        table,
        cols,
        values,
        cols_to_return,
        InsertValuesFmt::Wrapped,
    )
}
#[must_use]
pub fn generate_rm_query_string(
    table: PgTableNameRef<'_>,
    select_string: PgTableSqlFragmentRef<'_>,
    where_string: PgTableSqlFragmentRef<'_>,
) -> PgTableQueryString {
    generate_select_query_string(table, select_string, where_string, SelectWhereFmt::Plain)
}
#[must_use]
pub fn generate_ro_query_string(
    table: PgTableNameRef<'_>,
    select_string: PgTableSqlFragmentRef<'_>,
    where_string: PgTableSqlFragmentRef<'_>,
) -> PgTableQueryString {
    generate_select_query_string(table, select_string, where_string, SelectWhereFmt::Where)
}
#[must_use]
pub fn generate_column_queals_v_comma_uo_query_part(
    column: PgTableSqlFragmentRef<'_>,
    value: PgTableSqlFragmentRef<'_>,
) -> PgTableQueryPartFragment {
    let mut query_part = String::with_capacity(
        column
            .as_ref()
            .len()
            .saturating_add(value.as_ref().len())
            .saturating_add(5),
    );
    if std::fmt::Write::write_fmt(&mut query_part, format_args!("{column} = {value},")).is_err() {
        return PgTableQueryPartFragment::try_from(String::default())
            .unwrap_or_else(PgTableQueryPartFragment::from);
    }
    PgTableQueryPartFragment::try_from(query_part).unwrap_or_else(PgTableQueryPartFragment::from)
}
#[must_use]
pub fn generate_when_column_id_then_v_um_query_part(
    column: PgTableSqlFragmentRef<'_>,
    id: PgTableSqlFragmentRef<'_>,
    value: PgTableSqlFragmentRef<'_>,
) -> PgTableQueryPartFragment {
    let mut query_part = String::with_capacity(
        column
            .as_ref()
            .len()
            .saturating_add(id.as_ref().len())
            .saturating_add(value.as_ref().len())
            .saturating_add(15),
    );
    if std::fmt::Write::write_fmt(
        &mut query_part,
        format_args!("when {column} = {id} then {value} "),
    )
    .is_err()
    {
        return PgTableQueryPartFragment::try_from(String::default())
            .unwrap_or_else(PgTableQueryPartFragment::from);
    }
    PgTableQueryPartFragment::try_from(query_part).unwrap_or_else(PgTableQueryPartFragment::from)
}
#[must_use]
pub fn generate_column_eqs_case_accumulator_else_column_end_comma_um_query_part(
    column: PgTableSqlFragmentRef<'_>,
    accumulator: PgTableSqlFragmentRef<'_>,
) -> PgTableQueryPartFragment {
    let mut query_part = String::with_capacity(
        column
            .as_ref()
            .len()
            .saturating_mul(2)
            .saturating_add(accumulator.as_ref().len())
            .saturating_add(19),
    );
    if std::fmt::Write::write_fmt(
        &mut query_part,
        format_args!("{column} = case {accumulator}else {column} end,"),
    )
    .is_err()
    {
        return PgTableQueryPartFragment::try_from(String::default())
            .unwrap_or_else(PgTableQueryPartFragment::from);
    }
    PgTableQueryPartFragment::try_from(query_part).unwrap_or_else(PgTableQueryPartFragment::from)
}
//todo extra param for cols_to_return instead of primary_key_field_name in "returning {primary_key_field_name}""
#[must_use]
pub fn generate_um_query_string(
    table: PgTableNameRef<'_>,
    els: PgTableSqlFragmentRef<'_>,
    primary_key_field_name: PgTableSqlFragmentRef<'_>,
    pks: PgTableSqlFragmentRef<'_>,
    cols_to_return: PgTableSqlFragmentRef<'_>,
) -> PgTableQueryString {
    generate_update_query_string(
        table,
        els,
        primary_key_field_name,
        pks,
        cols_to_return,
        UpdateSelectorFmt::InList,
    )
}
//todo extra param for cols_to_return instead of primary_key_field_name in "returning {primary_key_field_name}""
#[must_use]
pub fn generate_uo_query_string(
    table: PgTableNameRef<'_>,
    cols: PgTableSqlFragmentRef<'_>,
    primary_key_field_name: PgTableSqlFragmentRef<'_>,
    primary_key_query_part: PgTableSqlFragmentRef<'_>,
    cols_to_return: PgTableSqlFragmentRef<'_>,
) -> PgTableQueryString {
    generate_update_query_string(
        table,
        cols,
        primary_key_field_name,
        primary_key_query_part,
        cols_to_return,
        UpdateSelectorFmt::Eq,
    )
}
#[must_use]
pub fn add_uo_optimistic_revision_predicate(
    query: PgTableQueryString,
    revision_column: PgTableSqlFragmentRef<'_>,
    expected_revision_query_part: PgTableSqlFragmentRef<'_>,
) -> PgTableQueryString {
    let query_text = query.to_string();
    let Some((statement, returning)) = query_text.rsplit_once(str_constants::RETURNING) else {
        return query;
    };
    let mut optimistic_query = String::with_capacity(
        query_text
            .len()
            .saturating_add(revision_column.as_ref().len().saturating_mul(2usize))
            .saturating_add(expected_revision_query_part.as_ref().len())
            .saturating_add(9usize),
    );
    optimistic_query.push_str(statement);
    optimistic_query.push_str(str_constants::AND);
    optimistic_query.push_str(revision_column.as_ref());
    optimistic_query.push_str(str_constants::TEXT_ALT);
    optimistic_query.push_str(expected_revision_query_part.as_ref());
    optimistic_query.push_str(str_constants::RETURNING);
    optimistic_query.push_str(returning);
    PgTableQueryString::try_from(optimistic_query).unwrap_or_else(PgTableQueryString::from)
}
#[must_use]
pub fn generate_dm_query_string(
    table: PgTableNameRef<'_>,
    where_string: PgTableSqlFragmentRef<'_>,
    primary_key_field_name: PgTableSqlFragmentRef<'_>,
) -> PgTableQueryString {
    generate_delete_query_string(table, primary_key_field_name, Some(where_string))
}
#[must_use]
pub fn generate_dlo_query_string(
    table: PgTableNameRef<'_>,
    primary_key_field_name: PgTableSqlFragmentRef<'_>,
) -> PgTableQueryString {
    generate_delete_query_string(table, primary_key_field_name, None)
}
#[cfg(test)]
mod tests {
    fn table(v: &'static str) -> super::PgTableNameRef<'static> {
        super::PgTableNameRef::from(v)
    }
    fn sql(v: &'static str) -> super::PgTableSqlFragmentRef<'static> {
        super::PgTableSqlFragmentRef::from(v)
    }
    fn users_base() -> (
        super::PgTableNameRef<'static>,
        super::PgTableSqlFragmentRef<'static>,
    ) {
        (
            table(str_constants::USERS_ALT),
            sql(str_constants::SQL_NAMES_ID),
        )
    }
    fn assert_q(actual: &str, expected: &'static str) {
        assert_eq!(actual, expected);
    }
    #[test]
    fn generate_cm_query_string_is_expected() {
        assert_q(
            &super::generate_cm_query_string(
                table(str_constants::USERS_ALT),
                sql(str_constants::ID_NAME),
                sql(str_constants::DOLLAR_1_DOLLAR_2_DOLLAR_3_DOLLAR_4),
                sql(str_constants::SQL_NAMES_ID),
            ),
            str_constants::INSERT_INTO_USERS_ID_NAME_VALUES_DOLLAR_1_DOLLAR_2_DOLLAR_3,
        );
    }
    #[test]
    fn generate_co_query_string_is_expected() {
        assert_q(
            &super::generate_co_query_string(
                table(str_constants::USERS_ALT),
                sql(str_constants::ID_NAME),
                sql(str_constants::DOLLAR_1_DOLLAR_2),
                sql(str_constants::SQL_NAMES_ID),
            ),
            str_constants::INSERT_INTO_USERS_ID_NAME_VALUES_DOLLAR_1_DOLLAR_2_RETURNING_ID,
        );
    }
    #[test]
    fn generate_rm_query_string_is_expected() {
        assert_q(
            &super::generate_rm_query_string(
                table(str_constants::USERS_ALT),
                sql(str_constants::ID_NAME),
                sql(str_constants::ORDER_BY_ID),
            ),
            str_constants::SELECT_ID_NAME_FROM_USERS_ORDER_BY_ID,
        );
    }
    #[test]
    fn generate_ro_query_string_is_expected() {
        assert_q(
            &super::generate_ro_query_string(
                table(str_constants::USERS_ALT),
                sql(str_constants::ID_NAME),
                sql(str_constants::ID_DOLLAR_1),
            ),
            str_constants::SELECT_ID_NAME_FROM_USERS_WHERE_ID_DOLLAR_1,
        );
    }
    #[test]
    fn generate_column_queals_v_comma_uo_query_part_is_expected() {
        assert_q(
            &super::generate_column_queals_v_comma_uo_query_part(
                sql(str_constants::NAME),
                sql(str_constants::DOLLAR_2),
            ),
            str_constants::NAME_DOLLAR_2_ALT,
        );
    }
    #[test]
    fn generate_when_column_id_then_v_um_query_part_is_expected() {
        assert_q(
            &super::generate_when_column_id_then_v_um_query_part(
                sql(str_constants::SQL_NAMES_ID),
                sql(str_constants::DOLLAR_1_ALT),
                sql(str_constants::DOLLAR_2),
            ),
            str_constants::WHEN_ID_DOLLAR_1_THEN_DOLLAR_2,
        );
    }
    #[test]
    fn generate_column_eqs_case_accumulator_else_column_end_comma_um_query_part_is_expected() {
        assert_q(
            &super::generate_column_eqs_case_accumulator_else_column_end_comma_um_query_part(
                sql(str_constants::NAME),
                sql(str_constants::WHEN_ID_DOLLAR_1_THEN_DOLLAR_2),
            ),
            str_constants::NAME_CASE_WHEN_ID_DOLLAR_1_THEN_DOLLAR_2_ELSE_NAME_END,
        );
    }
    #[test]
    fn generate_um_query_string_is_expected() {
        assert_q(
            &super::generate_um_query_string(
                table(str_constants::USERS_ALT),
                sql(str_constants::NAME_CASE_END),
                sql(str_constants::SQL_NAMES_ID),
                sql(str_constants::DOLLAR_1_DOLLAR_2),
                sql(str_constants::ID_NAME),
            ),
            str_constants::UPDATE_USERS_SET_NAME_CASE_END_WHERE_ID_IN_DOLLAR_1_DOLLAR,
        );
    }
    #[test]
    fn generate_uo_query_string_is_expected() {
        assert_q(
            &super::generate_uo_query_string(
                table(str_constants::USERS_ALT),
                sql(str_constants::NAME_DOLLAR_2),
                sql(str_constants::SQL_NAMES_ID),
                sql(str_constants::DOLLAR_1_ALT),
                sql(str_constants::ID_NAME),
            ),
            str_constants::UPDATE_USERS_SET_NAME_DOLLAR_2_WHERE_ID_DOLLAR_1_RETURNING_ID,
        );
    }
    #[test]
    fn optimistic_uo_query_requires_matching_revision() {
        let query = super::add_uo_optimistic_revision_predicate(
            super::generate_uo_query_string(
                table(str_constants::USERS_ALT),
                sql(str_constants::NAME_DOLLAR_1_REVISION_REVISION_PLUS_1),
                sql(str_constants::SQL_NAMES_ID),
                sql(str_constants::DOLLAR_2),
                sql(str_constants::ID_REVISION),
            ),
            sql(str_constants::REVISION),
            sql(str_constants::DOLLAR_3),
        );
        assert_q(
            &query,
            str_constants::UPDATE_USERS_SET_NAME_DOLLAR_1_REVISION_REVISION_PLUS_1_WHERE_ID,
        );
    }
    #[test]
    fn revision_rejects_invalid_and_negative_values() {
        assert!(matches!(
            super::PgTableRevision::try_from("invalid".to_owned()),
            Err(super::PgTableRevisionTryFromStringError::Invalid(_))
        ));
        assert!(matches!(
            super::PgTableRevision::try_from("-1".to_owned()),
            Err(super::PgTableRevisionTryFromStringError::Negative)
        ));
        assert_eq!(
            super::PgTableRevision::try_from("7".to_owned())
                .expect("63520e0f")
                .to_string(),
            "7"
        );
    }
    #[test]
    fn generate_dm_query_string_is_expected() {
        assert_q(
            &super::generate_dm_query_string(
                table(str_constants::USERS_ALT),
                sql(str_constants::WHERE_ID_IN_DOLLAR_1_DOLLAR_2),
                sql(str_constants::SQL_NAMES_ID),
            ),
            str_constants::DELETE_FROM_USERS_WHERE_ID_IN_DOLLAR_1_DOLLAR_2_RETURNING_ID,
        );
    }
    #[test]
    fn generate_dlo_query_string_is_expected() {
        let (table, primary_key) = users_base();
        assert_q(
            &super::generate_dlo_query_string(table, primary_key),
            str_constants::DELETE_FROM_USERS_WHERE_ID_DOLLAR_1_RETURNING_ID,
        );
    }
    #[test]
    fn generate_um_query_string_wraps_primary_key_selector_for_in_clause() {
        let v = super::generate_um_query_string(
            table(str_constants::USERS_ALT),
            sql(str_constants::NAME_CASE_END),
            sql(str_constants::SQL_NAMES_ID),
            sql(str_constants::DOLLAR_1_DOLLAR_2),
            sql(str_constants::ID_NAME),
        );
        assert!(v.contains("where id in ($1,$2)"));
    }
    #[test]
    fn generate_delete_query_string_uses_provided_filter_without_rewrite() {
        let (table, primary_key) = users_base();
        assert_q(
            &super::generate_delete_query_string(
                table,
                primary_key,
                Some(sql(
                    str_constants::WHERE_ID_IN_DOLLAR_1_DOLLAR_2_AND_ACTIVE_TRUE,
                )),
            ),
            str_constants::DELETE_FROM_USERS_WHERE_ID_IN_DOLLAR_1_DOLLAR_2_AND_ACTIVE,
        );
    }
    #[test]
    fn generate_update_query_string_eq_keeps_selector_without_extra_wrapping() {
        assert_q(
            &super::generate_update_query_string(
                table(str_constants::USERS_ALT),
                sql(str_constants::NAME_DOLLAR_2),
                sql(str_constants::SQL_NAMES_ID),
                sql(str_constants::DOLLAR_1_ALT),
                sql(str_constants::ID_NAME),
                super::UpdateSelectorFmt::Eq,
            ),
            str_constants::UPDATE_USERS_SET_NAME_DOLLAR_2_WHERE_ID_DOLLAR_1_RETURNING_ID,
        );
    }
    #[test]
    fn generate_update_query_string_in_list_wraps_selector_once() {
        assert_q(
            &super::generate_update_query_string(
                table(str_constants::USERS_ALT),
                sql(str_constants::NAME_CASE_END),
                sql(str_constants::SQL_NAMES_ID),
                sql(str_constants::DOLLAR_1_DOLLAR_2),
                sql(str_constants::ID_NAME),
                super::UpdateSelectorFmt::InList,
            ),
            str_constants::UPDATE_USERS_SET_NAME_CASE_END_WHERE_ID_IN_DOLLAR_1_DOLLAR,
        );
    }
}
