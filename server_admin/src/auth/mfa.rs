#![allow(
    clippy::arithmetic_side_effects,
    clippy::big_endian_bytes,
    clippy::indexing_slicing,
    clippy::integer_division,
    clippy::integer_division_remainder_used,
    clippy::single_call_fn,
    clippy::unused_trait_names
)] // cohesive TOTP helpers encode fixed-width cryptographic steps; bounds and byte order are proven by those formats

#[derive(Debug, thiserror::Error)]
pub(super) enum AdminMfaError {
    #[error("MFA cryptographic operation failed")]
    Crypto,
    #[error("system clock is before Unix epoch")]
    SystemClock,
    #[error("MFA value does not satisfy its contract")]
    Validation,
}
#[derive(Clone, Copy, Debug, newtype::FromInner)]
struct AdminMfaTotpCounter(u64);
#[derive(Clone, Copy, Debug, newtype::AsRefOwned, newtype::FromInner)]
struct AdminMfaTotpBytes([u8; 6]);

pub(super) struct AesGcmAdminMfaCipher(Vec<aes_gcm::Aes256Gcm>);
impl std::fmt::Debug for AesGcmAdminMfaCipher {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(str_constants::AES_GCM_ADMIN_MFA_CIPHER_REDACTED)
    }
}
pub(super) struct AdminEncryptedMfaSecret {
    ciphertext: super::super::StdAdminMfaEncryptedBytes,
    nonce: super::super::StdAdminMfaNonceBytes,
}
impl AdminEncryptedMfaSecret {
    pub(super) const fn ciphertext(&self) -> &super::super::StdAdminMfaEncryptedBytes {
        &self.ciphertext
    }
    pub(super) const fn nonce(&self) -> &super::super::StdAdminMfaNonceBytes {
        &self.nonce
    }
}
impl AesGcmAdminMfaCipher {
    pub(super) fn from_config(secret: &config_lib::AdminJwtSecret) -> Self {
        Self(
            secret
                .verification_secrets()
                .iter()
                .map(|value| {
                    let exposed = secrecy::ExposeSecret::expose_secret(value.as_ref());
                    let digest = <sha2::Sha256 as sha2::Digest>::digest(exposed.as_bytes());
                    let key = aes_gcm::Key::<aes_gcm::Aes256Gcm>::from(<[u8; 32]>::from(digest));
                    <aes_gcm::Aes256Gcm as aes_gcm::KeyInit>::new(&key)
                })
                .collect(),
        )
    }
    pub(super) fn encrypt(
        &self,
        secret: &super::super::StdAdminMfaSecretBytes,
        user_id: super::super::AdminUserId,
    ) -> Result<AdminEncryptedMfaSecret, AdminMfaError> {
        let nonce =
            <aes_gcm::Nonce<aes_gcm::aead::consts::U12> as aes_gcm::aead::Generate>::generate();
        let user_id_text = user_id.0.to_string();
        let payload = aes_gcm::aead::Payload {
            msg: secret.as_ref(),
            aad: user_id_text.as_bytes(),
        };
        let cipher = self.0.first().ok_or(AdminMfaError::Crypto)?;
        let ciphertext =
            <aes_gcm::Aes256Gcm as aes_gcm::aead::Aead>::encrypt(cipher, &nonce, payload)
                .map_err(|_error| AdminMfaError::Crypto)?;
        Ok(AdminEncryptedMfaSecret {
            ciphertext: super::super::StdAdminMfaEncryptedBytes::from(ciphertext),
            nonce: super::super::StdAdminMfaNonceBytes::from(nonce.to_vec()),
        })
    }
    pub(super) fn decrypt(
        &self,
        ciphertext: &super::super::StdAdminMfaEncryptedBytes,
        nonce_bytes: &super::super::StdAdminMfaNonceBytes,
        user_id: super::super::AdminUserId,
    ) -> Result<super::super::StdAdminMfaSecretBytes, AdminMfaError> {
        let nonce = aes_gcm::Nonce::try_from(nonce_bytes.as_ref().as_slice())
            .map_err(|_error| AdminMfaError::Validation)?;
        let user_id_text = user_id.0.to_string();
        self.0
            .iter()
            .find_map(|cipher| {
                <aes_gcm::Aes256Gcm as aes_gcm::aead::Aead>::decrypt(
                    cipher,
                    &nonce,
                    aes_gcm::aead::Payload {
                        msg: ciphertext.as_ref(),
                        aad: user_id_text.as_bytes(),
                    },
                )
                .ok()
            })
            .map(super::super::StdAdminMfaSecretBytes::from)
            .ok_or(AdminMfaError::Crypto)
    }
}

pub(super) fn generate_secret() -> super::super::StdAdminMfaSecretBytes {
    super::super::StdAdminMfaSecretBytes::from(
        <aes_gcm::Key<aes_gcm::Aes256Gcm> as aes_gcm::aead::Generate>::generate().to_vec(),
    )
}
pub(super) fn base32_encode(
    value: &super::super::StdAdminMfaSecretBytes,
) -> Result<server_admin_contract::AdminMfaSecret, AdminMfaError> {
    const ALPHABET: &[u8; 32] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ234567";
    let mut output =
        String::with_capacity(value.as_ref().len().saturating_mul(8usize).div_ceil(5usize));
    let mut buffer = 0u16;
    let mut bits = 0u8;
    value.as_ref().iter().for_each(|byte| {
        buffer = (buffer << 8u8) | u16::from(*byte);
        bits = bits.saturating_add(8u8);
        while bits >= 5u8 {
            bits -= 5u8;
            output.push(char::from(ALPHABET[usize::from((buffer >> bits) & 31u16)]));
        }
    });
    if bits > 0u8 {
        output.push(char::from(
            ALPHABET[usize::from((buffer << (5u8 - bits)) & 31u16)],
        ));
    }
    server_admin_contract::AdminMfaSecret::try_from(output)
        .map_err(|_error| AdminMfaError::Validation)
}
pub(super) fn enrollment_uri(
    secret: &server_admin_contract::AdminMfaSecret,
    login: &server_admin_contract::AdminLogin,
) -> Result<server_admin_contract::AdminMfaEnrollmentUri, AdminMfaError> {
    let uri_encode = |value: super::super::StdAdminStrRef<'_>| {
        value
            .as_ref()
            .bytes()
            .fold(String::new(), |mut output, byte| {
                if byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'.' | b'_' | b'~') {
                    output.push(char::from(byte));
                } else {
                    output.push_str(format!("%{byte:02X}").as_str());
                }
                output
            })
    };
    server_admin_contract::AdminMfaEnrollmentUri::try_from(format!(
        "otpauth://totp/{}:{}?secret={}&issuer={}&algorithm=SHA256&digits=6&period=30",
        uri_encode(super::super::StdAdminStrRef::from(
            str_constants::ADMIN_CONSOLE
        )),
        uri_encode(super::super::StdAdminStrRef::from(login.as_ref().as_str())),
        secret.as_ref(),
        uri_encode(super::super::StdAdminStrRef::from(
            str_constants::ADMIN_CONSOLE
        )),
    ))
    .map_err(|_error| AdminMfaError::Validation)
}
fn totp_at(
    secret: &super::super::StdAdminMfaSecretBytes,
    counter: AdminMfaTotpCounter,
) -> Result<AdminMfaTotpBytes, AdminMfaError> {
    let mut mac = <hmac::Hmac<sha2::Sha256> as hmac::Mac>::new_from_slice(secret.as_ref())
        .map_err(|_error| AdminMfaError::Crypto)?;
    hmac::Mac::update(&mut mac, &counter.0.to_be_bytes());
    let digest = hmac::Mac::finalize(mac).into_bytes();
    let offset = usize::from(digest[digest.len().saturating_sub(1usize)] & 0x0fu8);
    let binary = (u32::from(digest[offset] & 0x7fu8) << 24u32)
        | (u32::from(digest[offset + 1usize]) << 16u32)
        | (u32::from(digest[offset + 2usize]) << 8u32)
        | u32::from(digest[offset + 3usize]);
    let code = format!("{:06}", binary % 1_000_000u32);
    let bytes: [u8; 6] = code
        .as_bytes()
        .try_into()
        .map_err(|_error| AdminMfaError::Crypto)?;
    Ok(AdminMfaTotpBytes::from(bytes))
}
pub(super) fn verify_totp(
    secret: &super::super::StdAdminMfaSecretBytes,
    code: &server_admin_contract::AdminMfaCode,
) -> Result<Option<super::super::StdAdminMfaTotpCounter>, AdminMfaError> {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|_error| AdminMfaError::SystemClock)?
        .as_secs()
        / 30u64;
    let provided = code.as_ref().as_bytes();
    [now.saturating_sub(1u64), now, now.saturating_add(1u64)]
        .into_iter()
        .try_fold(None, |matched, counter| {
            totp_at(secret, AdminMfaTotpCounter::from(counter)).map(|expected| {
                if bool::from(<[u8] as subtle::ConstantTimeEq>::ct_eq(
                    expected.as_ref(),
                    provided,
                )) {
                    i64::try_from(counter)
                        .ok()
                        .map(super::super::StdAdminMfaTotpCounter::from)
                } else {
                    matched
                }
            })
        })
}
pub(super) fn recovery_code() -> Result<server_admin_contract::AdminRecoveryCode, AdminMfaError> {
    let compact = uuid::Uuid::new_v4().simple().to_string();
    let value = compact
        .as_bytes()
        .as_chunks::<4usize>()
        .0
        .iter()
        .take(4usize)
        .map(|chunk| std::str::from_utf8(chunk.as_slice()))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_error| AdminMfaError::Crypto)?
        .join(str_constants::HYPHEN);
    server_admin_contract::AdminRecoveryCode::try_from(value)
        .map_err(|_error| AdminMfaError::Validation)
}
pub(super) fn recovery_hash(
    code: &server_admin_contract::AdminRecoveryCode,
) -> super::super::StdAdminString {
    let digest = <sha2::Sha256 as sha2::Digest>::digest(code.as_ref().as_bytes());
    super::super::StdAdminString(format!("{digest:x}"))
}

#[cfg(test)]
mod tests {
    #[test]
    fn base32_and_totp_are_stable() {
        let secret =
            crate::StdAdminMfaSecretBytes::from(b"12345678901234567890123456789012".to_vec());
        assert_eq!(
            super::base32_encode(&secret).expect("bd94fd6c").as_ref(),
            str_constants::MFA_BASE32_TEST
        );
        assert_eq!(
            super::totp_at(&secret, super::AdminMfaTotpCounter::from(1u64))
                .expect("d20cf539")
                .as_ref(),
            b"119246"
        );
    }
    #[test]
    fn encrypted_secret_is_bound_to_user() {
        let digest = <sha2::Sha256 as sha2::Digest>::digest(str_constants::ROOT_SECRET.as_bytes());
        let key = aes_gcm::Key::<aes_gcm::Aes256Gcm>::from(<[u8; 32]>::from(digest));
        let cipher =
            super::AesGcmAdminMfaCipher(vec![<aes_gcm::Aes256Gcm as aes_gcm::KeyInit>::new(&key)]);
        let secret = crate::StdAdminMfaSecretBytes::from(b"mfa secret".to_vec());
        let encrypted = cipher
            .encrypt(&secret, crate::AdminUserId::from(1i64))
            .expect("a128a7a9");
        assert_eq!(
            cipher
                .decrypt(
                    &encrypted.ciphertext,
                    &encrypted.nonce,
                    crate::AdminUserId::from(1i64)
                )
                .expect("65466f55")
                .as_ref(),
            b"mfa secret"
        );
        let Err(_wrong_user_error) = cipher.decrypt(
            &encrypted.ciphertext,
            &encrypted.nonce,
            crate::AdminUserId::from(2i64),
        ) else {
            panic!("99a6b96f");
        };
    }
}
