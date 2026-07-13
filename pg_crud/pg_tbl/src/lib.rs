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
pub struct PgTblIdempotencyActor(String);
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PgTblIdempotencyKey(String);
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PgTblIdempotencyMethod(String);
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PgTblIdempotencyRoute(String);
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PgTblIdempotencyRequestHash([u8; 32usize]);
#[derive(Clone, Debug, Eq, PartialEq, newtype::Newtype)]
#[newtype(as_ref_target, from_inner)]
pub struct PgTblIdempotencyBody(Vec<u8>);
#[derive(Clone, Copy, Debug, Eq, PartialEq, newtype::Newtype)]
#[newtype(as_ref_inner, from_inner)]
pub struct PgTblIdempotencyBodyRef<'body_lt>(&'body_lt [u8]);
#[derive(Clone, Copy, Debug, Eq, PartialEq, newtype::Newtype)]
#[newtype(from_inner, into_inner_from)]
pub struct PgTblIdempotencyResponseStatus(u16);
#[derive(Clone, Copy, Debug, Eq, PartialEq, newtype::Newtype)]
#[newtype(display, from_inner)]
pub struct PgTblIdempotencyTextBytes(usize);
#[derive(Clone, Copy, Debug, Eq, PartialEq, newtype::Newtype)]
#[newtype(from_inner)]
pub struct PgTblIdempotencyCleanupRetentionSeconds(i64);
#[derive(Clone, Copy, Debug, Eq, PartialEq, newtype::Newtype)]
#[newtype(from_inner)]
pub struct PgTblIdempotencyCleanupBatchSize(i64);
#[derive(Clone, Copy, Debug, Eq, PartialEq, newtype::Newtype)]
#[newtype(from_inner, into_inner_from)]
pub struct PgTblIdempotencyCleanupRows(u64);
#[derive(Debug)]
pub struct SqlxPgTblPgConnectionRef<'connection_lt>(&'connection_lt mut sqlx::PgConnection);
#[derive(Clone, Copy, Debug, Eq, PartialEq, newtype::Newtype, sqlx::Type)]
#[newtype(display)]
#[sqlx(transparent)]
pub struct PgTblRevision(i64);
#[derive(Debug)]
pub struct StdPgTblRevisionParseIntEr(std::num::ParseIntError);
#[derive(Debug)]
pub enum PgTblRevisionTryFromStringEr {
    Invalid(StdPgTblRevisionParseIntEr),
    Negative,
}
impl std::fmt::Display for PgTblRevisionTryFromStringEr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Invalid(_error) => f.write_str("revision must be a decimal integer"),
            Self::Negative => f.write_str("revision must not be negative"),
        }
    }
}
impl std::error::Error for PgTblRevisionTryFromStringEr {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Invalid(error) => Some(&error.0),
            Self::Negative => None,
        }
    }
}
impl TryFrom<String> for PgTblRevision {
    type Error = PgTblRevisionTryFromStringEr;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let parsed = value.parse::<i64>().map_err(|error| {
            PgTblRevisionTryFromStringEr::Invalid(StdPgTblRevisionParseIntEr(error))
        })?;
        if parsed < 0i64 {
            Err(PgTblRevisionTryFromStringEr::Negative)
        } else {
            Ok(Self(parsed))
        }
    }
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PgTblIdempotencyScope {
    route: PgTblIdempotencyRoute,
    method: PgTblIdempotencyMethod,
    key: PgTblIdempotencyKey,
    actor: PgTblIdempotencyActor,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PgTblIdempotencyRequest {
    scope: PgTblIdempotencyScope,
    request_hash: PgTblIdempotencyRequestHash,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PgTblIdempotencyReplay {
    response_body: PgTblIdempotencyBody,
    response_status: PgTblIdempotencyResponseStatus,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PgTblIdempotencyBegin {
    Acquired,
    Conflict,
    InProgress,
    Replay(PgTblIdempotencyReplay),
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PgTblIdempotencyTextEr {
    Empty,
    InvalidMethod,
    InvalidRoute,
    TooLong {
        actual_bytes: PgTblIdempotencyTextBytes,
        maximum_bytes: PgTblIdempotencyTextBytes,
    },
}
impl std::fmt::Display for PgTblIdempotencyTextEr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => f.write_str("idempotency text must not be empty"),
            Self::InvalidMethod => f.write_str("idempotency method must be POST, PATCH, or DELETE"),
            Self::InvalidRoute => f.write_str("idempotency route must start with a slash"),
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
impl std::error::Error for PgTblIdempotencyTextEr {}
#[derive(Debug)]
pub struct SqlxPgTblIdempotencyEr(sqlx::Error);
impl std::fmt::Display for SqlxPgTblIdempotencyEr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("PostgreSQL idempotency operation failed")
    }
}
impl std::error::Error for SqlxPgTblIdempotencyEr {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.0)
    }
}
impl to_err_string::ToErrString for SqlxPgTblIdempotencyEr {
    fn to_err_string(&self) -> to_err_string::ToErrStringValue {
        to_err_string::ToErrStringValue::try_from(self.to_string())
            .unwrap_or_else(to_err_string::ToErrStringValue::from)
    }
}
impl From<sqlx::Error> for SqlxPgTblIdempotencyEr {
    fn from(value: sqlx::Error) -> Self {
        Self(value)
    }
}
impl TryFrom<String> for PgTblIdempotencyActor {
    type Error = PgTblIdempotencyTextEr;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            Err(PgTblIdempotencyTextEr::Empty)
        } else if value.len() > PG_TBL_IDEMPOTENCY_TEXT_MAX_BYTES {
            Err(PgTblIdempotencyTextEr::TooLong {
                actual_bytes: PgTblIdempotencyTextBytes::from(value.len()),
                maximum_bytes: PgTblIdempotencyTextBytes::from(PG_TBL_IDEMPOTENCY_TEXT_MAX_BYTES),
            })
        } else {
            Ok(Self(value))
        }
    }
}
impl TryFrom<String> for PgTblIdempotencyKey {
    type Error = PgTblIdempotencyTextEr;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            Err(PgTblIdempotencyTextEr::Empty)
        } else if value.len() > PG_TBL_IDEMPOTENCY_TEXT_MAX_BYTES {
            Err(PgTblIdempotencyTextEr::TooLong {
                actual_bytes: PgTblIdempotencyTextBytes::from(value.len()),
                maximum_bytes: PgTblIdempotencyTextBytes::from(PG_TBL_IDEMPOTENCY_TEXT_MAX_BYTES),
            })
        } else {
            Ok(Self(value))
        }
    }
}
impl AsRef<str> for PgTblIdempotencyKey {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}
#[must_use]
pub fn new_pg_tbl_idempotency_key() -> PgTblIdempotencyKey {
    match PgTblIdempotencyKey::try_from(uuid::Uuid::new_v4().to_string()) {
        Ok(value) => value,
        Err(_error) => std::process::abort(),
    }
}
impl TryFrom<String> for PgTblIdempotencyMethod {
    type Error = PgTblIdempotencyTextEr;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(PgTblIdempotencyTextEr::Empty);
        }
        if value.len() > PG_TBL_IDEMPOTENCY_TEXT_MAX_BYTES {
            return Err(PgTblIdempotencyTextEr::TooLong {
                actual_bytes: PgTblIdempotencyTextBytes::from(value.len()),
                maximum_bytes: PgTblIdempotencyTextBytes::from(PG_TBL_IDEMPOTENCY_TEXT_MAX_BYTES),
            });
        }
        if matches!(value.as_str(), "POST" | "PATCH" | "DELETE") {
            Ok(Self(value))
        } else {
            Err(PgTblIdempotencyTextEr::InvalidMethod)
        }
    }
}
impl TryFrom<String> for PgTblIdempotencyRoute {
    type Error = PgTblIdempotencyTextEr;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(PgTblIdempotencyTextEr::Empty);
        }
        if value.len() > PG_TBL_IDEMPOTENCY_ROUTE_MAX_BYTES {
            return Err(PgTblIdempotencyTextEr::TooLong {
                actual_bytes: PgTblIdempotencyTextBytes::from(value.len()),
                maximum_bytes: PgTblIdempotencyTextBytes::from(PG_TBL_IDEMPOTENCY_ROUTE_MAX_BYTES),
            });
        }
        if value.starts_with('/') {
            Ok(Self(value))
        } else {
            Err(PgTblIdempotencyTextEr::InvalidRoute)
        }
    }
}
impl PgTblIdempotencyScope {
    #[must_use]
    pub const fn new(
        actor: PgTblIdempotencyActor,
        method: PgTblIdempotencyMethod,
        route: PgTblIdempotencyRoute,
        key: PgTblIdempotencyKey,
    ) -> Self {
        Self {
            route,
            method,
            key,
            actor,
        }
    }
}
impl PgTblIdempotencyRequest {
    #[must_use]
    pub fn new(scope: PgTblIdempotencyScope, body: PgTblIdempotencyBodyRef<'_>) -> Self {
        Self {
            scope,
            request_hash: pg_tbl_idempotency_request_hash(body),
        }
    }
    #[must_use]
    pub const fn scope(&self) -> &PgTblIdempotencyScope {
        &self.scope
    }
}
impl PgTblIdempotencyReplay {
    #[must_use]
    pub fn into_parts(self) -> (PgTblIdempotencyResponseStatus, PgTblIdempotencyBody) {
        (self.response_status, self.response_body)
    }
}
#[must_use]
pub fn pg_tbl_idempotency_request_hash(
    body: PgTblIdempotencyBodyRef<'_>,
) -> PgTblIdempotencyRequestHash {
    let digest = <sha2::Sha256 as sha2::Digest>::digest(body.0);
    let mut bytes = [0u8; 32usize];
    bytes.copy_from_slice(&digest);
    PgTblIdempotencyRequestHash(bytes)
}
pub async fn ensure_pg_tbl_idempotency_schema(
    pool: app_state::SqlxPgPoolRef<'_>,
) -> Result<(), SqlxPgTblIdempotencyEr> {
    let _query_result = sqlx::query("CREATE TABLE IF NOT EXISTS pg_tbl_idempotency (actor TEXT NOT NULL, http_method TEXT NOT NULL CHECK (http_method IN ('POST','PATCH','DELETE')), route_path TEXT NOT NULL CHECK (route_path LIKE '/%'), idempotency_key TEXT NOT NULL, request_hash BYTEA NOT NULL CHECK (octet_length(request_hash) = 32), response_status SMALLINT, response_body BYTEA, state TEXT NOT NULL CHECK (state IN ('pending','completed')), created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(), completed_at TIMESTAMPTZ, PRIMARY KEY (actor,http_method,route_path,idempotency_key), CHECK ((state = 'pending' AND response_status IS NULL AND response_body IS NULL AND completed_at IS NULL) OR (state = 'completed' AND response_status IS NOT NULL AND response_body IS NOT NULL AND completed_at IS NOT NULL)))").execute(pool.as_ref()).await.map_err(SqlxPgTblIdempotencyEr::from)?;
    let _index_result = sqlx::query("CREATE INDEX IF NOT EXISTS pg_tbl_idempotency_created_at_idx ON pg_tbl_idempotency(created_at)").execute(pool.as_ref()).await.map_err(SqlxPgTblIdempotencyEr::from)?;
    Ok(())
}
pub async fn begin_pg_tbl_idempotency(
    pool: app_state::SqlxPgPoolRef<'_>,
    request: &PgTblIdempotencyRequest,
) -> Result<PgTblIdempotencyBegin, SqlxPgTblIdempotencyEr> {
    let inserted = sqlx::query_scalar::<_, bool>("INSERT INTO pg_tbl_idempotency (actor,http_method,route_path,idempotency_key,request_hash,state) VALUES ($1,$2,$3,$4,$5,'pending') ON CONFLICT DO NOTHING RETURNING TRUE")
        .bind(request.scope.actor.0.as_str()).bind(request.scope.method.0.as_str()).bind(request.scope.route.0.as_str()).bind(request.scope.key.0.as_str()).bind(request.request_hash.0.as_slice()).fetch_optional(pool.as_ref()).await.map_err(SqlxPgTblIdempotencyEr::from)?;
    if inserted == Some(true) {
        return Ok(PgTblIdempotencyBegin::Acquired);
    }
    let existing = sqlx::query_as::<_, (Vec<u8>, String, Option<i16>, Option<Vec<u8>>)>("SELECT request_hash,state,response_status,response_body FROM pg_tbl_idempotency WHERE actor=$1 AND http_method=$2 AND route_path=$3 AND idempotency_key=$4")
        .bind(request.scope.actor.0.as_str()).bind(request.scope.method.0.as_str()).bind(request.scope.route.0.as_str()).bind(request.scope.key.0.as_str()).fetch_one(pool.as_ref()).await.map_err(SqlxPgTblIdempotencyEr::from)?;
    if existing.0.as_slice() != request.request_hash.0.as_slice() {
        return Ok(PgTblIdempotencyBegin::Conflict);
    }
    if existing.1 == "pending" {
        return Ok(PgTblIdempotencyBegin::InProgress);
    }
    match (existing.2, existing.3) {
        (Some(status), Some(response_body)) => {
            let response_status = match u16::try_from(status) {
                Ok(value) => value,
                Err(_error) => return Ok(PgTblIdempotencyBegin::InProgress),
            };
            Ok(PgTblIdempotencyBegin::Replay(PgTblIdempotencyReplay {
                response_body: PgTblIdempotencyBody::from(response_body),
                response_status: PgTblIdempotencyResponseStatus::from(response_status),
            }))
        }
        (None | Some(_), None) | (None, Some(_)) => Ok(PgTblIdempotencyBegin::InProgress),
    }
}
pub async fn complete_pg_tbl_idempotency(
    pool: app_state::SqlxPgPoolRef<'_>,
    request: &PgTblIdempotencyRequest,
    response_status: PgTblIdempotencyResponseStatus,
    response_body: PgTblIdempotencyBodyRef<'_>,
) -> Result<(), SqlxPgTblIdempotencyEr> {
    if response_body.0.len() > PG_TBL_IDEMPOTENCY_RESPONSE_MAX_BYTES {
        return release_pg_tbl_idempotency(pool, request).await;
    }
    let response_status_i16 = match i16::try_from(response_status.0) {
        Ok(value) => value,
        Err(_error) => return release_pg_tbl_idempotency(pool, request).await,
    };
    let _query_result = sqlx::query("UPDATE pg_tbl_idempotency SET state='completed',response_status=$6,response_body=$7,completed_at=NOW() WHERE actor=$1 AND http_method=$2 AND route_path=$3 AND idempotency_key=$4 AND request_hash=$5 AND state='pending'")
        .bind(request.scope.actor.0.as_str()).bind(request.scope.method.0.as_str()).bind(request.scope.route.0.as_str()).bind(request.scope.key.0.as_str()).bind(request.request_hash.0.as_slice()).bind(response_status_i16).bind(response_body.0).execute(pool.as_ref()).await.map_err(SqlxPgTblIdempotencyEr::from)?;
    Ok(())
}
pub async fn complete_pg_tbl_idempotency_in_connection(
    mut connection: SqlxPgTblPgConnectionRef<'_>,
    request: &PgTblIdempotencyRequest,
    response_status: PgTblIdempotencyResponseStatus,
    response_body: PgTblIdempotencyBodyRef<'_>,
) -> Result<(), SqlxPgTblIdempotencyEr> {
    if response_body.0.len() > PG_TBL_IDEMPOTENCY_RESPONSE_MAX_BYTES {
        return Err(SqlxPgTblIdempotencyEr(sqlx::Error::Protocol(
            "idempotency response exceeds the storage limit".to_owned(),
        )));
    }
    let response_status_i16 = i16::try_from(response_status.0).map_err(|_error| {
        SqlxPgTblIdempotencyEr(sqlx::Error::Protocol(
            "idempotency response status is outside SMALLINT".to_owned(),
        ))
    })?;
    let result = sqlx::query("UPDATE pg_tbl_idempotency SET state='completed',response_status=$6,response_body=$7,completed_at=NOW() WHERE actor=$1 AND http_method=$2 AND route_path=$3 AND idempotency_key=$4 AND request_hash=$5 AND state='pending'")
        .bind(request.scope.actor.0.as_str()).bind(request.scope.method.0.as_str()).bind(request.scope.route.0.as_str()).bind(request.scope.key.0.as_str()).bind(request.request_hash.0.as_slice()).bind(response_status_i16).bind(response_body.0).execute(connection.as_mut()).await.map_err(SqlxPgTblIdempotencyEr::from)?;
    if result.rows_affected() == 1u64 {
        Ok(())
    } else {
        Err(SqlxPgTblIdempotencyEr(sqlx::Error::Protocol(
            "idempotency reservation is unavailable for completion".to_owned(),
        )))
    }
}
pub async fn release_pg_tbl_idempotency(
    pool: app_state::SqlxPgPoolRef<'_>,
    request: &PgTblIdempotencyRequest,
) -> Result<(), SqlxPgTblIdempotencyEr> {
    let _query_result = sqlx::query("DELETE FROM pg_tbl_idempotency WHERE actor=$1 AND http_method=$2 AND route_path=$3 AND idempotency_key=$4 AND request_hash=$5 AND state='pending'")
        .bind(request.scope.actor.0.as_str()).bind(request.scope.method.0.as_str()).bind(request.scope.route.0.as_str()).bind(request.scope.key.0.as_str()).bind(request.request_hash.0.as_slice()).execute(pool.as_ref()).await.map_err(SqlxPgTblIdempotencyEr::from)?;
    Ok(())
}
impl<'connection_lt> From<&'connection_lt mut sqlx::PgConnection>
    for SqlxPgTblPgConnectionRef<'connection_lt>
{
    fn from(value: &'connection_lt mut sqlx::PgConnection) -> Self {
        Self(value)
    }
}
impl AsMut<sqlx::PgConnection> for SqlxPgTblPgConnectionRef<'_> {
    fn as_mut(&mut self) -> &mut sqlx::PgConnection {
        self.0
    }
}
pub async fn cleanup_pg_tbl_idempotency(
    pool: app_state::SqlxPgPoolRef<'_>,
    completed_retention_seconds: PgTblIdempotencyCleanupRetentionSeconds,
    pending_retention_seconds: PgTblIdempotencyCleanupRetentionSeconds,
    batch_size: PgTblIdempotencyCleanupBatchSize,
) -> Result<PgTblIdempotencyCleanupRows, SqlxPgTblIdempotencyEr> {
    let result = sqlx::query("WITH expired AS (SELECT actor,http_method,route_path,idempotency_key FROM pg_tbl_idempotency WHERE (state='completed' AND completed_at < NOW() - make_interval(secs => $1)) OR (state='pending' AND created_at < NOW() - make_interval(secs => $2)) ORDER BY created_at LIMIT $3) DELETE FROM pg_tbl_idempotency target USING expired WHERE target.actor=expired.actor AND target.http_method=expired.http_method AND target.route_path=expired.route_path AND target.idempotency_key=expired.idempotency_key")
        .bind(completed_retention_seconds.0).bind(pending_retention_seconds.0).bind(batch_size.0).execute(pool.as_ref()).await.map_err(SqlxPgTblIdempotencyEr::from)?;
    Ok(PgTblIdempotencyCleanupRows::from(result.rows_affected()))
}
#[cfg(test)]
mod idempotency_tests {
    #[test]
    fn request_hash_is_stable_and_payload_sensitive() {
        let first = super::pg_tbl_idempotency_request_hash(super::PgTblIdempotencyBodyRef::from(
            b"same payload".as_slice(),
        ));
        let second = super::pg_tbl_idempotency_request_hash(super::PgTblIdempotencyBodyRef::from(
            b"same payload".as_slice(),
        ));
        let changed = super::pg_tbl_idempotency_request_hash(super::PgTblIdempotencyBodyRef::from(
            b"changed payload".as_slice(),
        ));
        assert_eq!(first, second);
        assert_ne!(first, changed);
    }
    #[test]
    fn idempotency_text_types_enforce_boundaries_and_protocol_shape() {
        assert_eq!(
            super::PgTblIdempotencyActor::try_from(String::new()),
            Err(super::PgTblIdempotencyTextEr::Empty)
        );
        assert_eq!(
            super::PgTblIdempotencyMethod::try_from("GET".to_owned()),
            Err(super::PgTblIdempotencyTextEr::InvalidMethod)
        );
        assert_eq!(
            super::PgTblIdempotencyRoute::try_from("without-slash".to_owned()),
            Err(super::PgTblIdempotencyTextEr::InvalidRoute)
        );
        let oversized = "a".repeat(super::PG_TBL_IDEMPOTENCY_TEXT_MAX_BYTES.saturating_add(1usize));
        assert_eq!(
            super::PgTblIdempotencyKey::try_from(oversized.clone()),
            Err(super::PgTblIdempotencyTextEr::TooLong {
                actual_bytes: super::PgTblIdempotencyTextBytes::from(oversized.len()),
                maximum_bytes: super::PgTblIdempotencyTextBytes::from(
                    super::PG_TBL_IDEMPOTENCY_TEXT_MAX_BYTES,
                ),
            })
        );
    }
    #[test]
    fn generated_idempotency_keys_are_valid_and_distinct() {
        let first = super::new_pg_tbl_idempotency_key();
        let second = super::new_pg_tbl_idempotency_key();
        assert_ne!(first, second);
        assert!(!first.as_ref().is_empty());
        assert!(first.as_ref().len() <= super::PG_TBL_IDEMPOTENCY_TEXT_MAX_BYTES);
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
#[derive(Debug, Clone, Copy, newtype::Newtype)]
#[newtype(as_ref_inner, display)]
pub struct PgTblNameRef<'lt>(&'lt str);
impl<'lt, T> From<&'lt T> for PgTblNameRef<'lt>
where
    T: AsRef<str> + ?Sized,
{
    fn from(value: &'lt T) -> Self {
        Self(value.as_ref())
    }
}
#[derive(Debug, Clone, Copy, newtype::Newtype)]
#[newtype(as_ref_inner, display)]
pub struct PgTblSqlFragmentRef<'lt>(&'lt str);
impl<'lt, T> From<&'lt T> for PgTblSqlFragmentRef<'lt>
where
    T: AsRef<str> + ?Sized,
{
    fn from(value: &'lt T) -> Self {
        Self(value.as_ref())
    }
}
#[derive(Debug, Clone, newtype::Newtype)]
#[newtype(deref_target, display)]
pub struct PgTblQueryString(String);
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PgTblStringWrapperTryFromStringEr {
    TooLong { len: usize, max: usize },
}
impl std::fmt::Display for PgTblStringWrapperTryFromStringEr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TooLong { len, max } => {
                write!(
                    f,
                    "pg tbl string wrapper length {len} exceeds maximum {max}"
                )
            }
        }
    }
}
impl From<PgTblStringWrapperTryFromStringEr> for PgTblQueryString {
    fn from(value: PgTblStringWrapperTryFromStringEr) -> Self {
        Self(value.to_string())
    }
}
impl TryFrom<String> for PgTblQueryString {
    type Error = PgTblStringWrapperTryFromStringEr;
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
#[derive(Debug, Clone, newtype::Newtype)]
#[newtype(deref_target, display)]
pub struct PgTblQpFragment(String);
impl From<PgTblStringWrapperTryFromStringEr> for PgTblQpFragment {
    fn from(value: PgTblStringWrapperTryFromStringEr) -> Self {
        Self(value.to_string())
    }
}
impl TryFrom<String> for PgTblQpFragment {
    type Error = PgTblStringWrapperTryFromStringEr;
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
fn gen_insert_query_string(
    tbl: PgTblNameRef<'_>,
    cols: PgTblSqlFragmentRef<'_>,
    values: PgTblSqlFragmentRef<'_>,
    cols_to_return: PgTblSqlFragmentRef<'_>,
    insert_values_fmt: InsertValuesFmt,
) -> PgTblQueryString {
    let wrapper_len = match insert_values_fmt {
        InsertValuesFmt::Raw => 0usize,
        InsertValuesFmt::Wrapped => 2usize,
    };
    let mut query = String::with_capacity(
        34usize
            .saturating_add(tbl.as_ref().len())
            .saturating_add(cols.as_ref().len())
            .saturating_add(values.as_ref().len())
            .saturating_add(cols_to_return.as_ref().len())
            .saturating_add(wrapper_len),
    );
    query.push_str("insert into ");
    query.push_str(tbl.as_ref());
    query.push_str(" (");
    query.push_str(cols.as_ref());
    query.push_str(") values ");
    if matches!(insert_values_fmt, InsertValuesFmt::Wrapped) {
        query.push('(');
    }
    query.push_str(values.as_ref());
    if matches!(insert_values_fmt, InsertValuesFmt::Wrapped) {
        query.push(')');
    }
    query.push_str(" returning ");
    query.push_str(cols_to_return.as_ref());
    PgTblQueryString::try_from(query).unwrap_or_else(PgTblQueryString::from)
}
fn gen_select_query_string(
    tbl: PgTblNameRef<'_>,
    sel_string: PgTblSqlFragmentRef<'_>,
    wh_string: PgTblSqlFragmentRef<'_>,
    select_where_fmt: SelectWhereFmt,
) -> PgTblQueryString {
    let where_len = match select_where_fmt {
        SelectWhereFmt::Plain => 1usize,
        SelectWhereFmt::Where => 7usize,
    };
    let mut query = String::with_capacity(
        13usize
            .saturating_add(sel_string.as_ref().len())
            .saturating_add(tbl.as_ref().len())
            .saturating_add(wh_string.as_ref().len())
            .saturating_add(where_len),
    );
    query.push_str("select ");
    query.push_str(sel_string.as_ref());
    query.push_str(" from ");
    query.push_str(tbl.as_ref());
    match select_where_fmt {
        SelectWhereFmt::Plain => query.push(' '),
        SelectWhereFmt::Where => query.push_str(" where "),
    }
    query.push_str(wh_string.as_ref());
    PgTblQueryString::try_from(query).unwrap_or_else(PgTblQueryString::from)
}
fn gen_update_query_string(
    tbl: PgTblNameRef<'_>,
    cols_or_els: PgTblSqlFragmentRef<'_>,
    pk_field_name: PgTblSqlFragmentRef<'_>,
    pk_selector: PgTblSqlFragmentRef<'_>,
    cols_to_return: PgTblSqlFragmentRef<'_>,
    update_selector_fmt: UpdateSelectorFmt,
) -> PgTblQueryString {
    let selector_len = match update_selector_fmt {
        UpdateSelectorFmt::Eq => 3usize,
        UpdateSelectorFmt::InList => 6usize,
    };
    let mut query = String::with_capacity(
        30usize
            .saturating_add(tbl.as_ref().len())
            .saturating_add(cols_or_els.as_ref().len())
            .saturating_add(pk_field_name.as_ref().len())
            .saturating_add(pk_selector.as_ref().len())
            .saturating_add(cols_to_return.as_ref().len())
            .saturating_add(selector_len),
    );
    query.push_str("update ");
    query.push_str(tbl.as_ref());
    query.push_str(" set ");
    query.push_str(cols_or_els.as_ref());
    query.push_str(" where ");
    query.push_str(pk_field_name.as_ref());
    match update_selector_fmt {
        UpdateSelectorFmt::Eq => query.push_str(" = "),
        UpdateSelectorFmt::InList => query.push_str(" in ("),
    }
    query.push_str(pk_selector.as_ref());
    if matches!(update_selector_fmt, UpdateSelectorFmt::InList) {
        query.push(')');
    }
    query.push_str(" returning ");
    query.push_str(cols_to_return.as_ref());
    PgTblQueryString::try_from(query).unwrap_or_else(PgTblQueryString::from)
}
fn gen_delete_query_string(
    tbl: PgTblNameRef<'_>,
    pk_field_name: PgTblSqlFragmentRef<'_>,
    wh_string: Option<PgTblSqlFragmentRef<'_>>,
) -> PgTblQueryString {
    let wh_len = wh_string.map_or_else(
        || 12usize.saturating_add(pk_field_name.as_ref().len()),
        |v| v.as_ref().len(),
    );
    let mut query = String::with_capacity(
        24usize
            .saturating_add(tbl.as_ref().len())
            .saturating_add(wh_len)
            .saturating_add(pk_field_name.as_ref().len()),
    );
    query.push_str("delete from ");
    query.push_str(tbl.as_ref());
    query.push(' ');
    if let Some(v) = wh_string {
        query.push_str(v.as_ref());
    } else {
        query.push_str("where ");
        query.push_str(pk_field_name.as_ref());
        query.push_str(" = $1");
    }
    query.push_str(" returning ");
    query.push_str(pk_field_name.as_ref());
    PgTblQueryString::try_from(query).unwrap_or_else(PgTblQueryString::from)
}
#[must_use]
pub fn gen_cm_query_string(
    tbl: PgTblNameRef<'_>,
    cols: PgTblSqlFragmentRef<'_>,
    values: PgTblSqlFragmentRef<'_>,
    cols_to_return: PgTblSqlFragmentRef<'_>,
) -> PgTblQueryString {
    gen_insert_query_string(tbl, cols, values, cols_to_return, InsertValuesFmt::Raw)
}
#[must_use]
pub fn gen_co_query_string(
    tbl: PgTblNameRef<'_>,
    cols: PgTblSqlFragmentRef<'_>,
    values: PgTblSqlFragmentRef<'_>,
    cols_to_return: PgTblSqlFragmentRef<'_>,
) -> PgTblQueryString {
    gen_insert_query_string(tbl, cols, values, cols_to_return, InsertValuesFmt::Wrapped)
}
#[must_use]
pub fn gen_rm_query_string(
    tbl: PgTblNameRef<'_>,
    sel_string: PgTblSqlFragmentRef<'_>,
    wh_string: PgTblSqlFragmentRef<'_>,
) -> PgTblQueryString {
    gen_select_query_string(tbl, sel_string, wh_string, SelectWhereFmt::Plain)
}
#[must_use]
pub fn gen_ro_query_string(
    tbl: PgTblNameRef<'_>,
    sel_string: PgTblSqlFragmentRef<'_>,
    wh_string: PgTblSqlFragmentRef<'_>,
) -> PgTblQueryString {
    gen_select_query_string(tbl, sel_string, wh_string, SelectWhereFmt::Where)
}
#[must_use]
pub fn gen_col_queals_v_comma_uo_qp(
    col: PgTblSqlFragmentRef<'_>,
    value: PgTblSqlFragmentRef<'_>,
) -> PgTblQpFragment {
    let mut qp = String::with_capacity(
        col.as_ref()
            .len()
            .saturating_add(value.as_ref().len())
            .saturating_add(5),
    );
    if std::fmt::Write::write_fmt(&mut qp, format_args!("{col} = {value},")).is_err() {
        return PgTblQpFragment::try_from(String::default()).unwrap_or_else(PgTblQpFragment::from);
    }
    PgTblQpFragment::try_from(qp).unwrap_or_else(PgTblQpFragment::from)
}
#[must_use]
pub fn gen_when_col_id_then_v_um_qp(
    col: PgTblSqlFragmentRef<'_>,
    id: PgTblSqlFragmentRef<'_>,
    value: PgTblSqlFragmentRef<'_>,
) -> PgTblQpFragment {
    let mut qp = String::with_capacity(
        col.as_ref()
            .len()
            .saturating_add(id.as_ref().len())
            .saturating_add(value.as_ref().len())
            .saturating_add(15),
    );
    if std::fmt::Write::write_fmt(&mut qp, format_args!("when {col} = {id} then {value} ")).is_err()
    {
        return PgTblQpFragment::try_from(String::default()).unwrap_or_else(PgTblQpFragment::from);
    }
    PgTblQpFragment::try_from(qp).unwrap_or_else(PgTblQpFragment::from)
}
#[must_use]
pub fn gen_col_eqs_case_acc_else_col_end_comma_um_qp(
    col: PgTblSqlFragmentRef<'_>,
    acc: PgTblSqlFragmentRef<'_>,
) -> PgTblQpFragment {
    let mut qp = String::with_capacity(
        col.as_ref()
            .len()
            .saturating_mul(2)
            .saturating_add(acc.as_ref().len())
            .saturating_add(19),
    );
    if std::fmt::Write::write_fmt(&mut qp, format_args!("{col} = case {acc}else {col} end,"))
        .is_err()
    {
        return PgTblQpFragment::try_from(String::default()).unwrap_or_else(PgTblQpFragment::from);
    }
    PgTblQpFragment::try_from(qp).unwrap_or_else(PgTblQpFragment::from)
}
//todo extra param for cols_to_return instead of pk_field_name in "returning {pk_field_name}""
#[must_use]
pub fn gen_um_query_string(
    tbl: PgTblNameRef<'_>,
    els: PgTblSqlFragmentRef<'_>,
    pk_field_name: PgTblSqlFragmentRef<'_>,
    pks: PgTblSqlFragmentRef<'_>,
    cols_to_return: PgTblSqlFragmentRef<'_>,
) -> PgTblQueryString {
    gen_update_query_string(
        tbl,
        els,
        pk_field_name,
        pks,
        cols_to_return,
        UpdateSelectorFmt::InList,
    )
}
//todo extra param for cols_to_return instead of pk_field_name in "returning {pk_field_name}""
#[must_use]
pub fn gen_uo_query_string(
    tbl: PgTblNameRef<'_>,
    cols: PgTblSqlFragmentRef<'_>,
    pk_field_name: PgTblSqlFragmentRef<'_>,
    pk_qp: PgTblSqlFragmentRef<'_>,
    cols_to_return: PgTblSqlFragmentRef<'_>,
) -> PgTblQueryString {
    gen_update_query_string(
        tbl,
        cols,
        pk_field_name,
        pk_qp,
        cols_to_return,
        UpdateSelectorFmt::Eq,
    )
}
#[must_use]
pub fn add_uo_optimistic_revision_predicate(
    query: PgTblQueryString,
    revision_col: PgTblSqlFragmentRef<'_>,
    expected_revision_qp: PgTblSqlFragmentRef<'_>,
) -> PgTblQueryString {
    let query_text = query.to_string();
    let Some((statement, returning)) = query_text.rsplit_once(" returning ") else {
        return query;
    };
    let mut optimistic_query = String::with_capacity(
        query_text
            .len()
            .saturating_add(revision_col.as_ref().len().saturating_mul(2usize))
            .saturating_add(expected_revision_qp.as_ref().len())
            .saturating_add(9usize),
    );
    optimistic_query.push_str(statement);
    optimistic_query.push_str(" and ");
    optimistic_query.push_str(revision_col.as_ref());
    optimistic_query.push_str(" = ");
    optimistic_query.push_str(expected_revision_qp.as_ref());
    optimistic_query.push_str(" returning ");
    optimistic_query.push_str(returning);
    PgTblQueryString::try_from(optimistic_query).unwrap_or_else(PgTblQueryString::from)
}
#[must_use]
pub fn gen_dm_query_string(
    tbl: PgTblNameRef<'_>,
    wh_string: PgTblSqlFragmentRef<'_>,
    pk_field_name: PgTblSqlFragmentRef<'_>,
) -> PgTblQueryString {
    gen_delete_query_string(tbl, pk_field_name, Some(wh_string))
}
#[must_use]
pub fn gen_dlo_query_string(
    tbl: PgTblNameRef<'_>,
    pk_field_name: PgTblSqlFragmentRef<'_>,
) -> PgTblQueryString {
    gen_delete_query_string(tbl, pk_field_name, None)
}
#[cfg(test)]
mod tests {
    fn tbl(v: &'static str) -> super::PgTblNameRef<'static> {
        super::PgTblNameRef::from(v)
    }
    fn sql(v: &'static str) -> super::PgTblSqlFragmentRef<'static> {
        super::PgTblSqlFragmentRef::from(v)
    }
    fn users_base() -> (
        super::PgTblNameRef<'static>,
        super::PgTblSqlFragmentRef<'static>,
    ) {
        (tbl("users"), sql("id"))
    }
    fn assert_q(actual: &str, expected: &'static str) {
        assert_eq!(actual, expected);
    }
    #[test]
    fn gen_cm_query_string_is_expected() {
        assert_q(
            &super::gen_cm_query_string(
                tbl("users"),
                sql("id,name"),
                sql("($1,$2),($3,$4)"),
                sql("id"),
            ),
            "insert into users (id,name) values ($1,$2),($3,$4) returning id",
        );
    }
    #[test]
    fn gen_co_query_string_is_expected() {
        assert_q(
            &super::gen_co_query_string(tbl("users"), sql("id,name"), sql("$1,$2"), sql("id")),
            "insert into users (id,name) values ($1,$2) returning id",
        );
    }
    #[test]
    fn gen_rm_query_string_is_expected() {
        assert_q(
            &super::gen_rm_query_string(tbl("users"), sql("id,name"), sql("order by id")),
            "select id,name from users order by id",
        );
    }
    #[test]
    fn gen_ro_query_string_is_expected() {
        assert_q(
            &super::gen_ro_query_string(tbl("users"), sql("id,name"), sql("id = $1")),
            "select id,name from users where id = $1",
        );
    }
    #[test]
    fn gen_col_queals_v_comma_uo_qp_is_expected() {
        assert_q(
            &super::gen_col_queals_v_comma_uo_qp(sql("name"), sql("$2")),
            "name = $2,",
        );
    }
    #[test]
    fn gen_when_col_id_then_v_um_qp_is_expected() {
        assert_q(
            &super::gen_when_col_id_then_v_um_qp(sql("id"), sql("$1"), sql("$2")),
            "when id = $1 then $2 ",
        );
    }
    #[test]
    fn gen_col_eqs_case_acc_else_col_end_comma_um_qp_is_expected() {
        assert_q(
            &super::gen_col_eqs_case_acc_else_col_end_comma_um_qp(
                sql("name"),
                sql("when id = $1 then $2 "),
            ),
            "name = case when id = $1 then $2 else name end,",
        );
    }
    #[test]
    fn gen_um_query_string_is_expected() {
        assert_q(
            &super::gen_um_query_string(
                tbl("users"),
                sql("name = case ... end,"),
                sql("id"),
                sql("$1,$2"),
                sql("id,name"),
            ),
            "update users set name = case ... end, where id in ($1,$2) returning id,name",
        );
    }
    #[test]
    fn gen_uo_query_string_is_expected() {
        assert_q(
            &super::gen_uo_query_string(
                tbl("users"),
                sql("name = $2"),
                sql("id"),
                sql("$1"),
                sql("id,name"),
            ),
            "update users set name = $2 where id = $1 returning id,name",
        );
    }
    #[test]
    fn optimistic_uo_query_requires_matching_revision() {
        let query = super::add_uo_optimistic_revision_predicate(
            super::gen_uo_query_string(
                tbl("users"),
                sql("name = $1, revision = revision + 1"),
                sql("id"),
                sql("$2"),
                sql("id,revision"),
            ),
            sql("revision"),
            sql("$3"),
        );
        assert_q(
            &query,
            "update users set name = $1, revision = revision + 1 where id = $2 and revision = $3 returning id,revision",
        );
    }
    #[test]
    fn revision_rejects_invalid_and_negative_values() {
        assert!(matches!(
            super::PgTblRevision::try_from("invalid".to_owned()),
            Err(super::PgTblRevisionTryFromStringEr::Invalid(_))
        ));
        assert!(matches!(
            super::PgTblRevision::try_from("-1".to_owned()),
            Err(super::PgTblRevisionTryFromStringEr::Negative)
        ));
        assert_eq!(
            super::PgTblRevision::try_from("7".to_owned())
                .expect("63520e0f")
                .to_string(),
            "7"
        );
    }
    #[test]
    fn gen_dm_query_string_is_expected() {
        assert_q(
            &super::gen_dm_query_string(tbl("users"), sql("where id in ($1,$2)"), sql("id")),
            "delete from users where id in ($1,$2) returning id",
        );
    }
    #[test]
    fn gen_dlo_query_string_is_expected() {
        let (tbl, pk) = users_base();
        assert_q(
            &super::gen_dlo_query_string(tbl, pk),
            "delete from users where id = $1 returning id",
        );
    }
    #[test]
    fn gen_um_query_string_wraps_pk_selector_for_in_clause() {
        let v = super::gen_um_query_string(
            tbl("users"),
            sql("name = case ... end,"),
            sql("id"),
            sql("$1,$2"),
            sql("id,name"),
        );
        assert!(v.contains("where id in ($1,$2)"));
    }
    #[test]
    fn gen_delete_query_string_uses_provided_filter_without_rewrite() {
        let (tbl, pk) = users_base();
        assert_q(
            &super::gen_delete_query_string(
                tbl,
                pk,
                Some(sql("where id in ($1,$2) and active = true")),
            ),
            "delete from users where id in ($1,$2) and active = true returning id",
        );
    }
    #[test]
    fn gen_update_query_string_eq_keeps_selector_without_extra_wrapping() {
        assert_q(
            &super::gen_update_query_string(
                tbl("users"),
                sql("name = $2"),
                sql("id"),
                sql("$1"),
                sql("id,name"),
                super::UpdateSelectorFmt::Eq,
            ),
            "update users set name = $2 where id = $1 returning id,name",
        );
    }
    #[test]
    fn gen_update_query_string_in_list_wraps_selector_once() {
        assert_q(
            &super::gen_update_query_string(
                tbl("users"),
                sql("name = case ... end,"),
                sql("id"),
                sql("$1,$2"),
                sql("id,name"),
                super::UpdateSelectorFmt::InList,
            ),
            "update users set name = case ... end, where id in ($1,$2) returning id,name",
        );
    }
}
