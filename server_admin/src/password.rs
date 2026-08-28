impl crate::AdminPasswordHasher {
    #[must_use]
    #[allow(clippy::missing_const_for_fn)] // Tokio semaphore and Arc constructors are not const
    pub fn new(max_concurrent_hashes: crate::AdminPasswordHashConcurrency) -> Self {
        Self::from_semaphore(crate::AdminSharedSemaphoreArc::from(std::sync::Arc::new(
            tokio::sync::Semaphore::new(max_concurrent_hashes.get().get().get()),
        )))
    }
    pub async fn hash(
        &self,
        password: crate::AdminPassword,
    ) -> Result<crate::AdminPasswordHash, crate::AdminPasswordHashError> {
        let permit = self.acquire().await?;
        tokio::task::spawn_blocking(move || {
            let result = {
                let password_secret = password.into_inner();
                argon2::PasswordHasher::hash_password(
                    &argon2::Argon2::default(),
                    secrecy::ExposeSecret::expose_secret(password_secret.as_ref()).as_bytes(),
                )
                .map(|hash| {
                    crate::AdminPasswordHash::new(
                        pg_types_text_misc::StringAsNonNullTextSecret::from(hash.to_string()),
                    )
                })
                .map_err(|error| {
                    crate::AdminPasswordHashError::PasswordHash(
                        crate::Argon2AdminPasswordHashError::from(error),
                    )
                })
            };
            drop(permit);
            result
        })
        .await
        .map_err(|error| {
            crate::AdminPasswordHashError::Join(crate::TokioAdminJoinError::from(error))
        })?
    }
    pub async fn verify(
        &self,
        password: crate::AdminPassword,
        expected_hash: crate::AdminPasswordHash,
    ) -> Result<crate::StdAdminBool, crate::AdminPasswordHashError> {
        let permit = self.acquire().await?;
        tokio::task::spawn_blocking(move || {
            let result = {
                let password_secret = password.into_inner();
                let expected_hash_text = expected_hash.expose();
                let parsed_hash =
                    argon2::PasswordHash::new(expected_hash_text.as_ref()).map_err(|error| {
                        crate::AdminPasswordHashError::PasswordHash(
                            crate::Argon2AdminPasswordHashError::from(
                                argon2::password_hash::Error::from(error),
                            ),
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
                Ok(()) => Ok(crate::StdAdminBool::from(true)),
                Err(argon2::password_hash::Error::PasswordInvalid) => {
                    Ok(crate::StdAdminBool::from(false))
                }
                Err(error) => Err(crate::AdminPasswordHashError::PasswordHash(
                    crate::Argon2AdminPasswordHashError::from(error),
                )),
            }
        })
        .await
        .map_err(|error| {
            crate::AdminPasswordHashError::Join(crate::TokioAdminJoinError::from(error))
        })?
    }
}
