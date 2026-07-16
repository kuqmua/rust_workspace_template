const MAXIMUM_RESOURCE_COUNT: usize = 10_000usize;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PgRelationRowCount(u64);
impl From<u64> for PgRelationRowCount {
    fn from(value: u64) -> Self {
        Self(value)
    }
}
impl From<PgRelationRowCount> for u64 {
    fn from(value: PgRelationRowCount) -> Self {
        value.0
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PgRelationCapacityMaximum(u64);
impl TryFrom<u64> for PgRelationCapacityMaximum {
    type Error = PgRelationCapacityError;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if value == 0u64 {
            Err(PgRelationCapacityError::ZeroMaximum)
        } else {
            Ok(Self(value))
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
pub enum PgRelationCapacityError {
    #[error("PostgreSQL relation capacity would be exceeded")]
    Exceeded,
    #[error("PostgreSQL relation row count overflowed")]
    Overflow,
    #[error("PostgreSQL relation capacity maximum must be greater than zero")]
    ZeroMaximum,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct PgRelationResourceId(i64);
impl From<i64> for PgRelationResourceId {
    fn from(value: i64) -> Self {
        Self(value)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PgRelationLockNamespace(String);
impl TryFrom<String> for PgRelationLockNamespace {
    type Error = PgRelationLockError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty()
            || value.len() > 128usize
            || !value
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'-'))
        {
            return Err(PgRelationLockError::InvalidNamespace);
        }
        Ok(Self(value))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PgRelationResourceIds(Vec<PgRelationResourceId>);
impl TryFrom<Vec<PgRelationResourceId>> for PgRelationResourceIds {
    type Error = PgRelationLockError;
    fn try_from(mut value: Vec<PgRelationResourceId>) -> Result<Self, Self::Error> {
        if value.len() > MAXIMUM_RESOURCE_COUNT {
            return Err(PgRelationLockError::TooManyResources);
        }
        value.sort_unstable();
        value.dedup();
        Ok(Self(value))
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
pub enum PgRelationLockError {
    #[error("PostgreSQL relation lock namespace is invalid")]
    InvalidNamespace,
    #[error("PostgreSQL relation lock resource count exceeds 10000")]
    TooManyResources,
}

#[derive(Debug)]
pub struct SqlxPgRelationLockError(sqlx::Error);
impl std::fmt::Display for SqlxPgRelationLockError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl std::error::Error for SqlxPgRelationLockError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.0)
    }
}

#[derive(Debug)]
pub struct SqlxPgRelationLockConnectionRef<'connection>(&'connection mut sqlx::PgConnection);
impl<'connection> From<&'connection mut sqlx::PgConnection>
    for SqlxPgRelationLockConnectionRef<'connection>
{
    fn from(value: &'connection mut sqlx::PgConnection) -> Self {
        Self(value)
    }
}
impl AsMut<sqlx::PgConnection> for SqlxPgRelationLockConnectionRef<'_> {
    fn as_mut(&mut self) -> &mut sqlx::PgConnection {
        self.0
    }
}

pub async fn lock_pg_relation_resources(
    mut connection: SqlxPgRelationLockConnectionRef<'_>,
    namespace: &PgRelationLockNamespace,
    resources: &PgRelationResourceIds,
) -> Result<(), SqlxPgRelationLockError> {
    if resources.0.is_empty() {
        return Ok(());
    }
    let resource_values = resources
        .0
        .iter()
        .map(|resource| resource.0)
        .collect::<Vec<_>>();
    let _result = sqlx::query(str_constants::PG_RELATION_RESOURCE_ADVISORY_LOCK_SQL)
        .bind(namespace.0.as_str())
        .bind(resource_values)
        .execute(connection.as_mut())
        .await
        .map_err(SqlxPgRelationLockError)?;
    Ok(())
}

pub fn validate_pg_relation_capacity(
    current: PgRelationRowCount,
    incoming: PgRelationRowCount,
    maximum: PgRelationCapacityMaximum,
) -> Result<PgRelationRowCount, PgRelationCapacityError> {
    let projected = current
        .0
        .checked_add(incoming.0)
        .ok_or(PgRelationCapacityError::Overflow)?;
    if projected > maximum.0 {
        Err(PgRelationCapacityError::Exceeded)
    } else {
        Ok(PgRelationRowCount(projected))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn capacity_rejects_excess_and_overflow() {
        let maximum = super::PgRelationCapacityMaximum::try_from(5u64).expect("4ddf36da");
        assert_eq!(
            super::validate_pg_relation_capacity(3u64.into(), 2u64.into(), maximum),
            Ok(super::PgRelationRowCount::from(5u64))
        );
        assert_eq!(
            super::validate_pg_relation_capacity(4u64.into(), 2u64.into(), maximum),
            Err(super::PgRelationCapacityError::Exceeded)
        );
        assert_eq!(
            super::validate_pg_relation_capacity(u64::MAX.into(), 1u64.into(), maximum),
            Err(super::PgRelationCapacityError::Overflow)
        );
    }

    #[test]
    fn resources_are_sorted_and_deduplicated_before_locking() {
        let resources = super::PgRelationResourceIds::try_from(vec![
            super::PgRelationResourceId::from(2i64),
            super::PgRelationResourceId::from(1i64),
            super::PgRelationResourceId::from(2i64),
        ])
        .expect("a9cf9ea3");
        assert_eq!(
            resources.0,
            vec![
                super::PgRelationResourceId::from(1i64),
                super::PgRelationResourceId::from(2i64),
            ]
        );
    }

    #[test]
    fn namespace_rejects_sql_syntax() {
        assert_eq!(
            super::PgRelationLockNamespace::try_from(String::from(
                str_constants::TEST_SQL_INJECTION
            )),
            Err(super::PgRelationLockError::InvalidNamespace)
        );
    }
}
