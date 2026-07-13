impl super::AdminPasswordHasher {
    #[must_use]
    #[allow(clippy::missing_const_for_fn)] // Tokio semaphore and Arc constructors are not const
    pub fn new(max_concurrent_hashes: super::AdminPasswordHashConcurrency) -> Self {
        Self {
            semaphore: super::StdAdminSharedSemaphore(std::sync::Arc::new(
                tokio::sync::Semaphore::new(max_concurrent_hashes.0.0.get()),
            )),
        }
    }
    pub async fn hash(
        &self,
        password: super::AdminPassword,
    ) -> Result<super::AdminPasswordHash, super::AdminPasswordHashEr> {
        let permit = std::sync::Arc::<tokio::sync::Semaphore>::clone(&self.semaphore.0)
            .acquire_owned()
            .await
            .map_err(|er| {
                super::AdminPasswordHashEr::SemaphoreClosed(super::TokioAdminAcquireEr::from(er))
            })?;
        tokio::task::spawn_blocking(move || {
            let password_secret = password.into_inner();
            let salt = argon2::password_hash::SaltString::generate(
                &mut argon2::password_hash::rand_core::OsRng,
            );
            let result = argon2::PasswordHasher::hash_password(
                &argon2::Argon2::default(),
                secrecy::ExposeSecret::expose_secret(password_secret.as_ref()).as_bytes(),
                &salt,
            )
            .map(|hash| {
                super::AdminPasswordHash::new(pg_types_text_misc::StringAsNnTextSecret::from(
                    hash.to_string(),
                ))
            })
            .map_err(|er| {
                super::AdminPasswordHashEr::PasswordHash(super::Argon2AdminPasswordHashEr::from(er))
            });
            drop(permit);
            result
        })
        .await
        .map_err(|er| super::AdminPasswordHashEr::Join(super::TokioAdminJoinEr::from(er)))?
    }
    pub async fn verify(
        &self,
        password: super::AdminPassword,
        expected_hash: super::AdminPasswordHash,
    ) -> Result<super::StdAdminBool, super::AdminPasswordHashEr> {
        let permit = std::sync::Arc::<tokio::sync::Semaphore>::clone(&self.semaphore.0)
            .acquire_owned()
            .await
            .map_err(|er| {
                super::AdminPasswordHashEr::SemaphoreClosed(super::TokioAdminAcquireEr::from(er))
            })?;
        tokio::task::spawn_blocking(move || {
            let password_secret = password.into_inner();
            let parsed_hash =
                argon2::PasswordHash::new(expected_hash.0.as_ref()).map_err(|er| {
                    super::AdminPasswordHashEr::PasswordHash(
                        super::Argon2AdminPasswordHashEr::from(er),
                    )
                })?;
            let result = argon2::PasswordVerifier::verify_password(
                &argon2::Argon2::default(),
                secrecy::ExposeSecret::expose_secret(password_secret.as_ref()).as_bytes(),
                &parsed_hash,
            );
            drop(permit);
            match result {
                Ok(()) => Ok(super::StdAdminBool::from(true)),
                Err(argon2::password_hash::Error::Password) => Ok(super::StdAdminBool::from(false)),
                Err(er) => Err(super::AdminPasswordHashEr::PasswordHash(
                    super::Argon2AdminPasswordHashEr::from(er),
                )),
            }
        })
        .await
        .map_err(|er| super::AdminPasswordHashEr::Join(super::TokioAdminJoinEr::from(er)))?
    }
}
