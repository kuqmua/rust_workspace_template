#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    generate_accessor_traits_for_struct_fields::GenerateAccessorTrait,
    newtype::DerefInner,
    newtype::FromInner,
)]
pub struct AdminAccessTokenTtlSeconds(super::ConfigNonZeroU64);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    generate_accessor_traits_for_struct_fields::GenerateAccessorTrait,
    newtype::DerefInner,
    newtype::FromInner,
)]
pub struct AdminRefreshTokenTtlSeconds(super::ConfigNonZeroU64);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    generate_accessor_traits_for_struct_fields::GenerateAccessorTrait,
    newtype::DerefInner,
    newtype::FromInner,
)]
pub struct AdminLoginFailureLimit(super::ConfigNonZeroU64);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    generate_accessor_traits_for_struct_fields::GenerateAccessorTrait,
    newtype::DerefInner,
    newtype::FromInner,
)]
pub struct AdminSignInRateLimit(super::ConfigNonZeroU64);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    generate_accessor_traits_for_struct_fields::GenerateAccessorTrait,
    newtype::DerefInner,
    newtype::FromInner,
)]
pub struct AdminSessionLimit(super::ConfigNonZeroUsize);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, newtype::DebugTransparent, newtype::FromInner,
)]
pub struct AdminPositiveU64ParsingError(super::ParseIntError);
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
pub enum TryFromStdEnvVarOkAdminPositiveU64Error {
    #[error("administrator duration must be greater than zero")]
    IsZero,
    #[error("{admin_positive_u64_parsing:?}")]
    Parse {
        admin_positive_u64_parsing: AdminPositiveU64ParsingError,
    },
}
fn parse_admin_positive_u64(
    v: &super::StdEnvVarOk,
) -> Result<super::ConfigNonZeroU64, TryFromStdEnvVarOkAdminPositiveU64Error> {
    let parsed = v.0.parse::<u64>().map_err(|admin_positive_u64_parsing| {
        TryFromStdEnvVarOkAdminPositiveU64Error::Parse {
            admin_positive_u64_parsing: AdminPositiveU64ParsingError::from(
                super::ParseIntError::from(admin_positive_u64_parsing),
            ),
        }
    })?;
    std::num::NonZeroU64::new(parsed)
        .map(super::ConfigNonZeroU64::from)
        .ok_or(TryFromStdEnvVarOkAdminPositiveU64Error::IsZero)
}
impl super::TryFromStdEnvVarOk for AdminAccessTokenTtlSeconds {
    type Error = TryFromStdEnvVarOkAdminPositiveU64Error;
    fn try_from_std_env_var_ok(v: super::StdEnvVarOk) -> Result<Self, Self::Error> {
        parse_admin_positive_u64(&v).map(Self)
    }
}
impl super::TryFromStdEnvVarOk for AdminRefreshTokenTtlSeconds {
    type Error = TryFromStdEnvVarOkAdminPositiveU64Error;
    fn try_from_std_env_var_ok(v: super::StdEnvVarOk) -> Result<Self, Self::Error> {
        parse_admin_positive_u64(&v).map(Self)
    }
}
impl super::TryFromStdEnvVarOk for AdminLoginFailureLimit {
    type Error = TryFromStdEnvVarOkAdminPositiveU64Error;
    fn try_from_std_env_var_ok(v: super::StdEnvVarOk) -> Result<Self, Self::Error> {
        parse_admin_positive_u64(&v).map(Self)
    }
}
impl super::TryFromStdEnvVarOk for AdminSignInRateLimit {
    type Error = TryFromStdEnvVarOkAdminPositiveU64Error;
    fn try_from_std_env_var_ok(v: super::StdEnvVarOk) -> Result<Self, Self::Error> {
        parse_admin_positive_u64(&v).map(Self)
    }
}
impl super::TryFromStdEnvVarOk for AdminSessionLimit {
    type Error = TryFromStdEnvVarOkAdminPositiveU64Error;
    fn try_from_std_env_var_ok(v: super::StdEnvVarOk) -> Result<Self, Self::Error> {
        let value = parse_admin_positive_u64(&v)?;
        usize::try_from(value.0.get())
            .ok()
            .and_then(std::num::NonZeroUsize::new)
            .map(super::ConfigNonZeroUsize::from)
            .map(Self)
            .ok_or(TryFromStdEnvVarOkAdminPositiveU64Error::IsZero)
    }
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    generate_accessor_traits_for_struct_fields::GenerateAccessorTrait,
    newtype::DerefInner,
    newtype::FromInner,
)]
pub struct AdminPasswordHashConcurrency(super::ConfigNonZeroUsize);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, newtype::DebugTransparent, newtype::FromInner,
)]
pub struct AdminPositiveUsizeParsingError(super::ParseIntError);
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
pub enum TryFromStdEnvVarOkAdminPasswordHashConcurrencyError {
    #[error("administrator password hash concurrency must be greater than zero")]
    IsZero,
    #[error("{admin_positive_usize_parsing:?}")]
    Parse {
        admin_positive_usize_parsing: AdminPositiveUsizeParsingError,
    },
}
impl super::TryFromStdEnvVarOk for AdminPasswordHashConcurrency {
    type Error = TryFromStdEnvVarOkAdminPasswordHashConcurrencyError;
    fn try_from_std_env_var_ok(v: super::StdEnvVarOk) -> Result<Self, Self::Error> {
        let parsed =
            v.0.parse::<usize>()
                .map_err(|admin_positive_usize_parsing| Self::Error::Parse {
                    admin_positive_usize_parsing: AdminPositiveUsizeParsingError::from(
                        super::ParseIntError::from(admin_positive_usize_parsing),
                    ),
                })?;
        std::num::NonZeroUsize::new(parsed)
            .map(super::ConfigNonZeroUsize::from)
            .map(Self)
            .ok_or(Self::Error::IsZero)
    }
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    PartialEq,
    Eq,
    generate_accessor_traits_for_struct_fields::GenerateAccessorTrait,
    serde::Deserialize,
    serde::Serialize,
    newtype::BoundedString,
    newtype::AsRefOwned,
)]
#[bounded_string(max = 256, description = "administrator token issuer")]
#[serde(try_from = "String")]
pub struct AdminTokenIssuer(String);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    PartialEq,
    Eq,
    generate_accessor_traits_for_struct_fields::GenerateAccessorTrait,
    serde::Deserialize,
    serde::Serialize,
    newtype::BoundedString,
    newtype::AsRefOwned,
)]
#[bounded_string(max = 256, description = "administrator token audience")]
#[serde(try_from = "String")]
pub struct AdminTokenAudience(String);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, Clone, Copy, PartialEq, Eq, thiserror::Error,
)]
pub enum TryFromStdEnvVarOkAdminTokenTextError {
    #[error("administrator token text is empty")]
    Empty,
    #[error("administrator token text is too long")]
    TooLong,
}
fn parse_admin_token_text<T, Error>(
    v: super::StdEnvVarOk,
    map: impl FnOnce(String) -> Result<T, Error>,
) -> Result<T, TryFromStdEnvVarOkAdminTokenTextError> {
    if v.0.is_empty() {
        return Err(TryFromStdEnvVarOkAdminTokenTextError::Empty);
    }
    map(v.0).map_err(|_bounded_string_error| TryFromStdEnvVarOkAdminTokenTextError::TooLong)
}
impl super::TryFromStdEnvVarOk for AdminTokenIssuer {
    type Error = TryFromStdEnvVarOkAdminTokenTextError;
    fn try_from_std_env_var_ok(v: super::StdEnvVarOk) -> Result<Self, Self::Error> {
        parse_admin_token_text(v, Self::try_from)
    }
}
impl super::TryFromStdEnvVarOk for AdminTokenAudience {
    type Error = TryFromStdEnvVarOkAdminTokenTextError;
    fn try_from_std_env_var_ok(v: super::StdEnvVarOk) -> Result<Self, Self::Error> {
        parse_admin_token_text(v, Self::try_from)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn positive_values_and_token_text_preserve_validation() {
        let ttl = <super::AdminAccessTokenTtlSeconds as super::super::TryFromStdEnvVarOk>::try_from_std_env_var_ok(
            super::super::StdEnvVarOk::try_from(String::from(constants_str::VALUE_1)).expect("f39b6c2a positive_values_and_token_text_preserve_validation invariant must hold"),
        )
        .expect("de4810af positive_values_and_token_text_preserve_validation invariant must hold");
        assert_eq!(ttl.0.0.get(), 1u64);
        let zero = <super::AdminAccessTokenTtlSeconds as super::super::TryFromStdEnvVarOk>::try_from_std_env_var_ok(
            super::super::StdEnvVarOk::try_from(String::from(constants_str::VALUE_0)).expect("a48e903d positive_values_and_token_text_preserve_validation invariant must hold"),
        );
        assert!(matches!(
            zero,
            Err(super::TryFromStdEnvVarOkAdminPositiveU64Error::IsZero)
        ));
        let issuer =
            <super::AdminTokenIssuer as super::super::TryFromStdEnvVarOk>::try_from_std_env_var_ok(
                super::super::StdEnvVarOk::try_from(String::from(constants_str::VALUE_535C6F8E)).expect("01f2db8a positive_values_and_token_text_preserve_validation invariant must hold"),
            )
            .expect("80c5df37 positive_values_and_token_text_preserve_validation invariant must hold");
        assert_eq!(issuer.as_ref(), "issuer");
    }
}
