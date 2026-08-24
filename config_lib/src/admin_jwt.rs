const ADMIN_JWT_SECRET_MIN_LEN: usize = 32;
const ADMIN_JWT_SECRET_MAX_COUNT: usize = 8;

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    generate_getter_traits_for_struct_fields::GenerateGetterTrait,
    newtype::AsRefOwned,
    newtype::DebugRedacted,
    newtype::FromInner,
)]
pub struct AdminJwtSecret(
    bounded_types::BoundedVec<super::SecrecySecretBoxString, 1, ADMIN_JWT_SECRET_MAX_COUNT>,
);

impl AdminJwtSecret {
    #[must_use]
    pub fn primary(&self) -> Option<&super::SecrecySecretBoxString> {
        self.0.first()
    }

    #[must_use]
    pub const fn verification_secrets(&self) -> &[super::SecrecySecretBoxString] {
        self.0.as_slice()
    }
}

#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, Clone, Copy, PartialEq, Eq, thiserror::Error,
)]
pub enum TryFromStdEnvVarOkAdminJwtSecretError {
    #[error("administrator JWT secret list must not be empty")]
    Empty,
    #[error("administrator JWT secret list contains an empty entry")]
    EmptyEntry,
    #[error(
        "administrator JWT secret list must contain at most {ADMIN_JWT_SECRET_MAX_COUNT} entries"
    )]
    TooMany,
    #[error("administrator JWT secret must contain at least {ADMIN_JWT_SECRET_MIN_LEN} bytes")]
    TooShort,
    #[error("administrator JWT secret is too long")]
    TooLong,
}

impl super::TryFromStdEnvVarOk for AdminJwtSecret {
    type Error = TryFromStdEnvVarOkAdminJwtSecretError;

    fn try_from_std_env_var_ok(v: super::StdEnvVarOk) -> Result<Self, Self::Error> {
        if v.0.split(',').map(str::trim).all(str::is_empty) {
            return Err(Self::Error::Empty);
        }
        let raw_secrets = v.0.split(',').map(str::trim);
        let raw_secret_count = raw_secrets.clone().count();
        if raw_secret_count > ADMIN_JWT_SECRET_MAX_COUNT {
            return Err(Self::Error::TooMany);
        }
        let secrets = raw_secrets
            .into_iter()
            .map(|value| {
                if value.is_empty() {
                    Err(Self::Error::EmptyEntry)
                } else if value.len() < ADMIN_JWT_SECRET_MIN_LEN {
                    Err(Self::Error::TooShort)
                } else {
                    super::SecrecySecretBoxString::try_from(value.to_owned())
                        .map_err(|_error| Self::Error::TooLong)
                }
            })
            .collect::<Result<Vec<_>, _>>()?;
        bounded_types::BoundedVec::try_from(secrets)
            .map(Self)
            .map_err(|error| match error {
                bounded_types::BoundedValueError::BelowMin { .. } => Self::Error::Empty,
                bounded_types::BoundedValueError::AboveMax { .. }
                | bounded_types::BoundedValueError::InvalidBounds { .. } => Self::Error::TooMany,
            })
    }
}

#[cfg(test)]
mod tests {
    #[test]
    #[cfg_attr(
        miri,
        ignore = "Miri interpretation is prohibitively slow when zeroizing the intentional oversized allocation"
    )]
    fn secret_box_string_rejects_values_above_shared_limit() {
        let value = str_constants::TEST_JWT_SECRET_CHARACTER_A.repeat(
            super::super::CONFIG_LIB_STRING_WRAPPER_MAX_LEN.saturating_add(usize_constants::ONE),
        );
        let Err(_error) = super::super::SecrecySecretBoxString::try_from(value) else {
            panic!("41c03fcc");
        };
    }

    #[test]
    fn parses_primary_and_verification_secrets() {
        let first =
            str_constants::TEST_JWT_SECRET_CHARACTER_A.repeat(super::ADMIN_JWT_SECRET_MIN_LEN);
        let second =
            str_constants::TEST_JWT_SECRET_CHARACTER_B.repeat(super::ADMIN_JWT_SECRET_MIN_LEN);
        let parsed =
            <super::AdminJwtSecret as super::super::TryFromStdEnvVarOk>::try_from_std_env_var_ok(
                super::super::StdEnvVarOk::try_from(format!("{first}, {second}"))
                    .expect("12fd7c6a parses_primary_and_verification_secrets invariant must hold"),
            )
            .expect("2c18577d parses_primary_and_verification_secrets invariant must hold");
        assert_eq!(parsed.verification_secrets().len(), 2usize);
        assert_eq!(
            parsed
                .primary()
                .map(|secret| { secrecy::ExposeSecret::expose_secret(secret.as_ref()).as_ref() }),
            Some(&first)
        );
    }

    #[test]
    fn rejects_empty_effective_secret_list() {
        let result =
            <super::AdminJwtSecret as super::super::TryFromStdEnvVarOk>::try_from_std_env_var_ok(
                super::super::StdEnvVarOk::try_from(String::from(
                    str_constants::TEST_EMPTY_DELIMITED_LIST,
                ))
                .expect("86c514b2 rejects_empty_effective_secret_list invariant must hold"),
            );
        assert!(matches!(
            result,
            Err(super::TryFromStdEnvVarOkAdminJwtSecretError::Empty)
        ));
    }

    #[test]
    fn rejects_empty_secret_between_rotation_keys() {
        let secret =
            str_constants::TEST_JWT_SECRET_CHARACTER_A.repeat(super::ADMIN_JWT_SECRET_MIN_LEN);
        let result =
            <super::AdminJwtSecret as super::super::TryFromStdEnvVarOk>::try_from_std_env_var_ok(
                super::super::StdEnvVarOk::try_from(format!("{secret},,{secret}")).expect(
                    "9674829d rejects_empty_secret_between_rotation_keys invariant must hold",
                ),
            );
        assert!(matches!(
            result,
            Err(super::TryFromStdEnvVarOkAdminJwtSecretError::EmptyEntry)
        ));
    }
}
