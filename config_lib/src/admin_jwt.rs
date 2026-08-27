#[path = "admin_jwt/admin_jwt_secret.rs"]
mod admin_jwt_secret;
#[path = "admin_jwt/admin_jwt_secret_max_count.rs"]
mod admin_jwt_secret_max_count;
#[path = "admin_jwt/admin_jwt_secret_min_len.rs"]
mod admin_jwt_secret_min_len;
#[path = "admin_jwt/try_from_std_env_var_ok_admin_jwt_secret_error.rs"]
mod try_from_std_env_var_ok_admin_jwt_secret_error;

pub use admin_jwt_secret::{AdminJwtSecret, AdminJwtSecretProvider};
use admin_jwt_secret_max_count::ADMIN_JWT_SECRET_MAX_COUNT;
use admin_jwt_secret_min_len::ADMIN_JWT_SECRET_MIN_LEN;
pub use try_from_std_env_var_ok_admin_jwt_secret_error::TryFromStdEnvVarOkAdminJwtSecretError;

#[cfg(test)]
mod tests {
    #[test]
    #[cfg_attr(
        miri,
        ignore = "Miri interpretation is prohibitively slow when zeroizing the intentional oversized allocation"
    )]
    fn secret_box_string_rejects_values_above_shared_limit() {
        let value = constants_str::TEST_JWT_SECRET_CHARACTER_A.repeat(
            super::super::CONFIG_LIB_STRING_WRAPPER_MAX_LEN.saturating_add(constants_usize::ONE),
        );
        let Err(_error) = super::super::SecrecySecretBoxString::try_from(value) else {
            panic!("41c03fcc");
        };
    }

    #[test]
    fn parses_primary_and_verification_secrets() {
        let first =
            constants_str::TEST_JWT_SECRET_CHARACTER_A.repeat(super::ADMIN_JWT_SECRET_MIN_LEN);
        let second =
            constants_str::TEST_JWT_SECRET_CHARACTER_B.repeat(super::ADMIN_JWT_SECRET_MIN_LEN);
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
                    constants_str::TEST_EMPTY_DELIMITED_LIST,
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
            constants_str::TEST_JWT_SECRET_CHARACTER_A.repeat(super::ADMIN_JWT_SECRET_MIN_LEN);
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
