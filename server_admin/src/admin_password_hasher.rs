#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, generate_accessor::Getters)]
pub struct AdminPasswordHasher {
    semaphore: crate::admin_shared_semaphore_arc::AdminSharedSemaphoreArc,
}

impl AdminPasswordHasher {
    #[must_use]
    pub fn new(
        max_concurrent_hashes: crate::runtime_admin_password_hash_concurrency::RuntimeAdminPasswordHashConcurrency,
    ) -> Self {
        Self {
            semaphore: crate::admin_shared_semaphore_arc::AdminSharedSemaphoreArc::new(
                max_concurrent_hashes,
            ),
        }
    }
    pub async fn hash(
        &self,
        password: crate::runtime_admin_password::RuntimeAdminPassword,
    ) -> Result<
        crate::admin_password_hash::AdminPasswordHash,
        crate::admin_password_hash_error::AdminPasswordHashError,
    > {
        let permit = self.acquire().await?;
        tokio::task::spawn_blocking(move || {
            let result = {
                let password_secret = password.into_inner();
                argon2::PasswordHasher::hash_password(
                    &argon2::Argon2::default(),
                    secrecy::ExposeSecret::expose_secret(password_secret.as_ref()).as_bytes(),
                )
                .map(|hash| {
                    crate::admin_password_hash::AdminPasswordHash::new(
                        pg_types_text_misc::generate_pg_types_mod::StringAsNonNullTextSecret::from(
                            hash.to_string(),
                        ),
                    )
                })
                .map_err(|error| {
                    crate::admin_password_hash_error::AdminPasswordHashError::PasswordHash(
                        crate::argon2_admin_password_hash_error::Argon2AdminPasswordHashError::from(
                            error,
                        ),
                    )
                })
            };
            drop(permit);
            result
        })
        .await
        .map_err(|error| {
            crate::admin_password_hash_error::AdminPasswordHashError::Join(
                crate::tokio_admin_join_error::TokioAdminJoinError::from(error),
            )
        })?
    }
    pub async fn verify(
        &self,
        password: crate::runtime_admin_password::RuntimeAdminPassword,
        expected_hash: crate::admin_password_hash::AdminPasswordHash,
    ) -> Result<
        server_admin_core::std_admin_bool::StdAdminBool,
        crate::admin_password_hash_error::AdminPasswordHashError,
    > {
        let permit = self.acquire().await?;
        tokio::task::spawn_blocking(move || {
            let result = {
                let password_secret = password.into_inner();
                let expected_hash_text = expected_hash.expose();
                let parsed_hash =
                    argon2::PasswordHash::new(expected_hash_text.as_ref()).map_err(|error| {
                        crate::admin_password_hash_error::AdminPasswordHashError::PasswordHash(
                            crate::argon2_admin_password_hash_error::Argon2AdminPasswordHashError::from(
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
                Ok(()) => Ok(server_admin_core::std_admin_bool::StdAdminBool::from(true)),
                Err(argon2::password_hash::Error::PasswordInvalid) => {
                    Ok(server_admin_core::std_admin_bool::StdAdminBool::from(false))
                }
                Err(error) => Err(crate::admin_password_hash_error::AdminPasswordHashError::PasswordHash(
                    crate::argon2_admin_password_hash_error::Argon2AdminPasswordHashError::from(error),
                )),
            }
        })
        .await
        .map_err(|error| {
            crate::admin_password_hash_error::AdminPasswordHashError::Join(crate::tokio_admin_join_error::TokioAdminJoinError::from(error))
        })?
    }
}
