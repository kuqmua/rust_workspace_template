const MAXIMUM_RESOURCE_COUNT: usize = 10_000usize;

#[derive(
    optml::Optml, Clone, Copy, Debug, Eq, PartialEq, newtype::FromInner, newtype::IntoInnerFrom,
)]
pub struct PgRelationRowCount(u64);

#[derive(optml::Optml, Clone, Copy, Debug, Eq, PartialEq)]
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

#[derive(optml::Optml, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
pub enum PgRelationCapacityError {
    #[error("PostgreSQL relation capacity would be exceeded")]
    Exceeded,
    #[error("PostgreSQL relation row count overflowed")]
    Overflow,
    #[error("PostgreSQL relation capacity maximum must be greater than zero")]
    ZeroMaximum,
}

#[derive(optml::Optml, Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, newtype::FromInner)]
pub struct PgRelationResourceId(i64);

#[derive(optml::Optml, Clone, Debug, Eq, PartialEq)]
pub struct PgRelationLockNamespace(String);
impl TryFrom<String> for PgRelationLockNamespace {
    type Error = PgRelationLockError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > 128usize {
            return Err(PgRelationLockError::InvalidNamespace);
        }
        text_policy::validate_url_safe_token_part(
            text_policy::UrlSafeTokenPartRef::from(value.as_str()),
            text_policy::UrlSafeTokenPartMaximumBytes::from(128usize),
        )
        .map_err(|_error| PgRelationLockError::InvalidNamespace)?;
        Ok(Self(value))
    }
}

#[derive(optml::Optml, Clone, Debug, Eq, PartialEq)]
pub struct PgRelationResourceIds(
    bounded_types::BoundedVec<PgRelationResourceId, 0usize, MAXIMUM_RESOURCE_COUNT>,
);
impl TryFrom<Vec<PgRelationResourceId>> for PgRelationResourceIds {
    type Error = PgRelationLockError;
    fn try_from(value: Vec<PgRelationResourceId>) -> Result<Self, Self::Error> {
        let mut resources = bounded_types::BoundedVec::<
            PgRelationResourceId,
            0usize,
            MAXIMUM_RESOURCE_COUNT,
        >::try_from(value)
        .map_err(|_error| PgRelationLockError::TooManyResources)?
        .into_inner();
        resources.sort_unstable();
        resources.dedup();
        bounded_types::BoundedVec::try_from(resources)
            .map(Self)
            .map_err(|_error| PgRelationLockError::TooManyResources)
    }
}

#[derive(optml::Optml, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
pub enum PgRelationLockError {
    #[error("PostgreSQL relation lock namespace is invalid")]
    InvalidNamespace,
    #[error("PostgreSQL relation lock resource count exceeds 10000")]
    TooManyResources,
}

#[derive(optml::Optml, Debug, thiserror::Error, newtype::FromInner)]
#[error(transparent)]
pub struct SqlxPgRelationLockError(sqlx::Error);

#[derive(optml::Optml, Debug, newtype::AsMut, newtype::FromInner)]
pub struct SqlxPgRelationLockConnectionRef<'connection>(&'connection mut sqlx::PgConnection);

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
        Ok(PgRelationRowCount::from(projected))
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
            resources.0.as_slice(),
            [
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
