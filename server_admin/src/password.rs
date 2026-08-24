impl super::AdminPasswordHasher {
    #[must_use]
    #[allow(clippy::missing_const_for_fn)] // Tokio semaphore and Arc constructors are not const
    pub fn new(max_concurrent_hashes: super::AdminPasswordHashConcurrency) -> Self {
        Self {
            semaphore: super::AdminSharedSemaphoreArc(std::sync::Arc::new(
                tokio::sync::Semaphore::new(max_concurrent_hashes.0.get().get()),
            )),
        }
    }
    pub async fn hash(
        &self,
        password: super::AdminPassword,
    ) -> Result<super::AdminPasswordHash, super::AdminPasswordHashError> {
        let permit = std::sync::Arc::<tokio::sync::Semaphore>::clone(&self.semaphore.0)
            .acquire_owned()
            .await
            .map_err(|error| {
                super::AdminPasswordHashError::SemaphoreClosed(super::TokioAdminAcquireError::from(
                    error,
                ))
            })?;
        tokio::task::spawn_blocking(move || {
            let result = {
                let password_secret = password.into_inner();
                let salt = argon2::password_hash::SaltString::generate(
                    &mut argon2::password_hash::rand_core::OsRng,
                );
                argon2::PasswordHasher::hash_password(
                    &argon2::Argon2::default(),
                    secrecy::ExposeSecret::expose_secret(password_secret.as_ref()).as_bytes(),
                    &salt,
                )
                .map(|hash| {
                    super::AdminPasswordHash::new(
                        pg_types_text_misc::StringAsNonNullTextSecret::from(hash.to_string()),
                    )
                })
                .map_err(|error| {
                    super::AdminPasswordHashError::PasswordHash(
                        super::Argon2AdminPasswordHashError::from(error),
                    )
                })
            };
            drop(permit);
            result
        })
        .await
        .map_err(|error| {
            super::AdminPasswordHashError::Join(super::TokioAdminJoinError::from(error))
        })?
    }
    pub async fn verify(
        &self,
        password: super::AdminPassword,
        expected_hash: super::AdminPasswordHash,
    ) -> Result<super::StdAdminBool, super::AdminPasswordHashError> {
        let permit = std::sync::Arc::<tokio::sync::Semaphore>::clone(&self.semaphore.0)
            .acquire_owned()
            .await
            .map_err(|error| {
                super::AdminPasswordHashError::SemaphoreClosed(super::TokioAdminAcquireError::from(
                    error,
                ))
            })?;
        tokio::task::spawn_blocking(move || {
            let result = {
                let password_secret = password.into_inner();
                let parsed_hash =
                    argon2::PasswordHash::new(expected_hash.0.as_ref()).map_err(|error| {
                        super::AdminPasswordHashError::PasswordHash(
                            super::Argon2AdminPasswordHashError::from(error),
                        )
                    })?;
                argon2::PasswordVerifier::verify_password(
                    &argon2::Argon2::default(),
                    secrecy::ExposeSecret::expose_secret(password_secret.as_ref()).as_bytes(),
                    &parsed_hash,
                )
            };
            drop(permit);
            match result {
                Ok(()) => Ok(super::StdAdminBool::from(true)),
                Err(argon2::password_hash::Error::Password) => Ok(super::StdAdminBool::from(false)),
                Err(error) => Err(super::AdminPasswordHashError::PasswordHash(
                    super::Argon2AdminPasswordHashError::from(error),
                )),
            }
        })
        .await
        .map_err(|error| {
            super::AdminPasswordHashError::Join(super::TokioAdminJoinError::from(error))
        })?
    }
}
