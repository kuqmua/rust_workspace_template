#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    generate_accessor_traits_for_struct_fields::GenerateAccessorTrait,
    newtype::AsRefOwned,
    newtype::DebugRedacted,
    newtype::FromInner,
)]
pub struct AdminJwtSecret(
    bounded_types::bounded_vec::BoundedVec<
        crate::secrecy_secret_box_string::SecrecySecretBoxString,
        1,
        { crate::admin_jwt_secret_max_count::ADMIN_JWT_SECRET_MAX_COUNT },
    >,
);

impl AdminJwtSecret {
    #[must_use]
    pub fn primary(&self) -> Option<&crate::secrecy_secret_box_string::SecrecySecretBoxString> {
        self.0.first()
    }

    #[must_use]
    pub const fn verification_secrets(
        &self,
    ) -> &[crate::secrecy_secret_box_string::SecrecySecretBoxString] {
        self.0.as_slice()
    }
}

impl crate::try_from_std_env_var_ok::TryFromStdEnvVarOk for AdminJwtSecret {
    type Error = crate::try_from_std_env_var_ok_admin_jwt_secret_error::TryFromStdEnvVarOkAdminJwtSecretError;

    fn try_from_std_env_var_ok(v: crate::std_env_var_ok::StdEnvVarOk) -> Result<Self, Self::Error> {
        if v.0.split(',').map(str::trim).all(str::is_empty) {
            return Err(Self::Error::Empty);
        }
        let raw_secrets = v.0.split(',').map(str::trim);
        let raw_secret_count = raw_secrets.clone().count();
        if raw_secret_count > crate::admin_jwt_secret_max_count::ADMIN_JWT_SECRET_MAX_COUNT {
            return Err(Self::Error::TooMany);
        }
        let secrets = raw_secrets
            .into_iter()
            .map(|value| {
                if value.is_empty() {
                    Err(Self::Error::EmptyEntry)
                } else if value.len() < crate::admin_jwt_secret_min_len::ADMIN_JWT_SECRET_MIN_LEN {
                    Err(Self::Error::TooShort)
                } else {
                    crate::secrecy_secret_box_string::SecrecySecretBoxString::try_from(
                        value.to_owned(),
                    )
                    .map_err(|_error| Self::Error::TooLong)
                }
            })
            .collect::<Result<Vec<_>, _>>()?;
        bounded_types::bounded_vec::BoundedVec::try_from(secrets)
            .map(Self)
            .map_err(|error| match error {
                bounded_types::bounded_value_error::BoundedValueError::BelowMin { .. } => {
                    Self::Error::Empty
                }
                bounded_types::bounded_value_error::BoundedValueError::AboveMax { .. }
                | bounded_types::bounded_value_error::BoundedValueError::InvalidBounds { .. } => {
                    Self::Error::TooMany
                }
            })
    }
}
