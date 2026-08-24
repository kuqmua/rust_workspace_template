#[derive(
    Debug,
    Clone,
    Copy,
    generate_getter_traits_for_struct_fields::GenerateGetterTrait,
    optimal_memory_layout::OptimalMemoryLayout,
    newtype::DerefInner,
)]
pub struct PgPoolMaxConnections(u32);
#[derive(
    Debug,
    Clone,
    Copy,
    optimal_memory_layout::OptimalMemoryLayout,
    newtype::DerefInner,
    newtype::FromInner,
)]
pub struct PgPoolMinConnections(u32);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    newtype::DerefInner,
    newtype::FromInner,
)]
pub struct PgPoolAcquireTimeoutSeconds(super::ConfigNonZeroU64);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    newtype::DerefInner,
    newtype::FromInner,
)]
pub struct PgPoolIdleTimeoutSeconds(super::ConfigNonZeroU64);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    newtype::DerefInner,
    newtype::FromInner,
)]
pub struct PgPoolMaxLifetimeSeconds(super::ConfigNonZeroU64);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    newtype::DerefInner,
    newtype::FromInner,
)]
pub struct RequestTimeoutSeconds(super::ConfigNonZeroU64);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, Clone, Copy, PartialEq, Eq, thiserror::Error,
)]
pub enum PgPoolConfigParseError {
    #[error("pg pool numeric configuration is invalid")]
    Parse,
    #[error("pg pool duration must be greater than zero")]
    Zero,
}
impl super::TryFromStdEnvVarOk for PgPoolMinConnections {
    type Error = PgPoolConfigParseError;
    fn try_from_std_env_var_ok(v: super::StdEnvVarOk) -> Result<Self, Self::Error> {
        v.0.parse::<u32>()
            .map(Self)
            .map_err(|_error| Self::Error::Parse)
    }
}
fn parse_pg_pool_non_zero_seconds(
    v: &super::StdEnvVarOk,
) -> Result<super::ConfigNonZeroU64, PgPoolConfigParseError> {
    let value =
        v.0.parse::<u64>()
            .map_err(|_error| PgPoolConfigParseError::Parse)?;
    std::num::NonZeroU64::new(value)
        .map(super::ConfigNonZeroU64::from)
        .ok_or(PgPoolConfigParseError::Zero)
}
impl super::TryFromStdEnvVarOk for PgPoolAcquireTimeoutSeconds {
    type Error = PgPoolConfigParseError;
    fn try_from_std_env_var_ok(v: super::StdEnvVarOk) -> Result<Self, Self::Error> {
        parse_pg_pool_non_zero_seconds(&v).map(Self)
    }
}
impl super::TryFromStdEnvVarOk for PgPoolIdleTimeoutSeconds {
    type Error = PgPoolConfigParseError;
    fn try_from_std_env_var_ok(v: super::StdEnvVarOk) -> Result<Self, Self::Error> {
        parse_pg_pool_non_zero_seconds(&v).map(Self)
    }
}
impl super::TryFromStdEnvVarOk for PgPoolMaxLifetimeSeconds {
    type Error = PgPoolConfigParseError;
    fn try_from_std_env_var_ok(v: super::StdEnvVarOk) -> Result<Self, Self::Error> {
        parse_pg_pool_non_zero_seconds(&v).map(Self)
    }
}
impl super::TryFromStdEnvVarOk for RequestTimeoutSeconds {
    type Error = PgPoolConfigParseError;
    fn try_from_std_env_var_ok(v: super::StdEnvVarOk) -> Result<Self, Self::Error> {
        parse_pg_pool_non_zero_seconds(&v).map(Self)
    }
}
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, thiserror::Error, optimal_memory_layout::OptimalMemoryLayout,
)]
pub enum PgPoolMaxConnectionsTryFromU32Error {
    #[error("pg pool max connections must be greater than zero")]
    IsZero,
}
impl TryFrom<u32> for PgPoolMaxConnections {
    type Error = PgPoolMaxConnectionsTryFromU32Error;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if value == 0 {
            Err(Self::Error::IsZero)
        } else {
            Ok(Self(value))
        }
    }
}
#[derive(Debug, thiserror::Error, optimal_memory_layout::OptimalMemoryLayout)]
pub enum TryFromStdEnvVarOkPgPoolMaxConnectionsError {
    #[error("{pg_pool_max_connections:?}")]
    PgPoolMaxConnections {
        pg_pool_max_connections: PgPoolMaxConnectionsTryFromU32Error,
    },
    #[error("{:?}", .u32_parsing)]
    U32Parsing {
        u32_parsing: super::U32ParseIntError,
    },
}
impl super::TryFromStdEnvVarOk for PgPoolMaxConnections {
    type Error = TryFromStdEnvVarOkPgPoolMaxConnectionsError;
    fn try_from_std_env_var_ok(v: super::StdEnvVarOk) -> Result<Self, Self::Error> {
        let parsed: u32 = super::parse_from_str_with_error(
            super::StdEnvVarOkRef::from(v.0.as_str()),
            |u32_parsing| Self::Error::U32Parsing {
                u32_parsing: super::U32ParseIntError::from(u32_parsing),
            },
        )?;
        Self::try_from(parsed).map_err(|pg_pool_max_connections| {
            Self::Error::PgPoolMaxConnections {
                pg_pool_max_connections,
            }
        })
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn pool_limits_and_timeouts_reject_zero() {
        let max = <super::PgPoolMaxConnections as super::super::TryFromStdEnvVarOk>::try_from_std_env_var_ok(
            super::super::StdEnvVarOk::try_from(String::from("1")).expect("6f71a4b9 pool_limits_and_timeouts_reject_zero invariant must hold"),
        )
        .expect("c8ef416d pool_limits_and_timeouts_reject_zero invariant must hold");
        assert_eq!(max.0, 1u32);
        let timeout = <super::RequestTimeoutSeconds as super::super::TryFromStdEnvVarOk>::try_from_std_env_var_ok(
            super::super::StdEnvVarOk::try_from(String::from("0")).expect("f02d58b1 pool_limits_and_timeouts_reject_zero invariant must hold"),
        );
        assert!(matches!(timeout, Err(super::PgPoolConfigParseError::Zero)));
    }
}
