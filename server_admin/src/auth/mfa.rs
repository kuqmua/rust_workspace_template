#[derive(Debug, thiserror::Error)]
pub(super) enum AdminMfaError {
    #[error("MFA cryptographic operation failed")]
    Crypto,
    #[error("system clock is before Unix epoch")]
    SystemClock,
    #[error("MFA value does not satisfy its contract")]
    Validation,
}

pub(super) struct AdminMfaCipher(aes_gcm::Aes256Gcm);
impl std::fmt::Debug for AdminMfaCipher {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("AdminMfaCipher([REDACTED])")
    }
}
pub(super) struct AdminEncryptedMfaSecret {
    pub(super) ciphertext: Vec<u8>,
    pub(super) nonce: Vec<u8>,
}
impl AdminMfaCipher {
    pub(super) fn from_root_secret(secret: &str) -> Self {
        use sha2::Digest;
        let digest = sha2::Sha256::digest(secret.as_bytes());
        Self(<aes_gcm::Aes256Gcm as aes_gcm::KeyInit>::new(
            aes_gcm::Key::<aes_gcm::Aes256Gcm>::from_slice(digest.as_slice()),
        ))
    }
    pub(super) fn encrypt(
        &self,
        secret: &[u8],
        user_id: super::super::AdminUserId,
    ) -> Result<AdminEncryptedMfaSecret, AdminMfaError> {
        use aes_gcm::aead::{Aead, Generate};
        let nonce = aes_gcm::Nonce::generate();
        let payload = aes_gcm::aead::Payload {
            msg: secret,
            aad: user_id.0.to_string().as_bytes(),
        };
        let ciphertext = self.0.encrypt(&nonce, payload).map_err(|_error| AdminMfaError::Crypto)?;
        Ok(AdminEncryptedMfaSecret { ciphertext, nonce: nonce.to_vec() })
    }
    pub(super) fn decrypt(
        &self,
        ciphertext: &[u8],
        nonce: &[u8],
        user_id: super::super::AdminUserId,
    ) -> Result<Vec<u8>, AdminMfaError> {
        use aes_gcm::aead::Aead;
        let nonce = aes_gcm::Nonce::try_from(nonce).map_err(|_error| AdminMfaError::Validation)?;
        self.0
            .decrypt(
                &nonce,
                aes_gcm::aead::Payload {
                    msg: ciphertext,
                    aad: user_id.0.to_string().as_bytes(),
                },
            )
            .map_err(|_error| AdminMfaError::Crypto)
    }
}

pub(super) fn generate_secret() -> Vec<u8> {
    use aes_gcm::aead::Generate;
    aes_gcm::Key::<aes_gcm::Aes256Gcm>::generate().to_vec()
}
pub(super) fn base32_encode(value: &[u8]) -> String {
    const ALPHABET: &[u8; 32] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ234567";
    let mut output = String::with_capacity(value.len().saturating_mul(8usize).div_ceil(5usize));
    let mut buffer = 0u16;
    let mut bits = 0u8;
    value.iter().for_each(|byte| {
        buffer = (buffer << 8u8) | u16::from(*byte);
        bits = bits.saturating_add(8u8);
        while bits >= 5u8 {
            bits -= 5u8;
            output.push(char::from(ALPHABET[usize::from((buffer >> bits) & 31u16)]));
        }
    });
    if bits > 0u8 {
        output.push(char::from(ALPHABET[usize::from((buffer << (5u8 - bits)) & 31u16)]));
    }
    output
}
fn uri_encode(value: &str) -> String {
    value.bytes().fold(String::new(), |mut output, byte| {
        if byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'.' | b'_' | b'~') {
            output.push(char::from(byte));
        } else {
            output.push_str(format!("%{byte:02X}").as_str());
        }
        output
    })
}
pub(super) fn enrollment_uri(
    secret: &server_admin_contract::AdminMfaSecret,
    login: &server_admin_contract::AdminLogin,
) -> Result<server_admin_contract::AdminMfaEnrollmentUri, AdminMfaError> {
    server_admin_contract::AdminMfaEnrollmentUri::try_from(format!(
        "otpauth://totp/{}:{}?secret={}&issuer={}&algorithm=SHA256&digits=6&period=30",
        uri_encode("Admin Console"),
        uri_encode(login.as_ref()),
        secret.as_ref(),
        uri_encode("Admin Console"),
    ))
    .map_err(|_error| AdminMfaError::Validation)
}
fn totp_at(secret: &[u8], counter: u64) -> Result<[u8; 6], AdminMfaError> {
    use hmac::Mac;
    let mut mac = <hmac::Hmac<sha2::Sha256> as hmac::Mac>::new_from_slice(secret)
        .map_err(|_error| AdminMfaError::Crypto)?;
    mac.update(&counter.to_be_bytes());
    let digest = mac.finalize().into_bytes();
    let offset = usize::from(digest[digest.len().saturating_sub(1usize)] & 0x0fu8);
    let binary = (u32::from(digest[offset] & 0x7fu8) << 24u32)
        | (u32::from(digest[offset + 1usize]) << 16u32)
        | (u32::from(digest[offset + 2usize]) << 8u32)
        | u32::from(digest[offset + 3usize]);
    let code = format!("{:06}", binary % 1_000_000u32);
    code.as_bytes().try_into().map_err(|_error| AdminMfaError::Crypto)
}
pub(super) fn verify_totp(
    secret: &[u8],
    code: &server_admin_contract::AdminMfaCode,
) -> Result<bool, AdminMfaError> {
    use subtle::ConstantTimeEq;
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|_error| AdminMfaError::SystemClock)?
        .as_secs()
        / 30u64;
    let provided = code.as_ref().as_bytes();
    [now.saturating_sub(1u64), now, now.saturating_add(1u64)]
        .into_iter()
        .try_fold(false, |matched, counter| {
            totp_at(secret, counter)
                .map(|expected| matched | bool::from(expected.as_slice().ct_eq(provided)))
        })
}
pub(super) fn recovery_code() -> Result<server_admin_contract::AdminRecoveryCode, AdminMfaError> {
    let compact = uuid::Uuid::new_v4().simple().to_string();
    let value = format!("{}-{}-{}-{}", &compact[0..4], &compact[4..8], &compact[8..12], &compact[12..16]);
    server_admin_contract::AdminRecoveryCode::try_from(value).map_err(|_error| AdminMfaError::Validation)
}
pub(super) fn recovery_hash(
    code: &server_admin_contract::AdminRecoveryCode,
) -> super::super::StdAdminString {
    use sha2::Digest;
    let digest = sha2::Sha256::digest(code.as_ref().as_bytes());
    super::super::StdAdminString::try_from(format!("{digest:x}")).unwrap_or_default()
}

#[cfg(test)]
mod tests {
    #[test]
    fn base32_and_totp_are_stable() {
        assert_eq!(super::base32_encode(b"foo"), "MZXW6");
        assert_eq!(super::totp_at(b"12345678901234567890123456789012", 1u64).expect("d20cf539"), *b"461192");
    }
    #[test]
    fn encrypted_secret_is_bound_to_user() {
        let cipher = super::AdminMfaCipher::from_root_secret("root secret");
        let encrypted = cipher.encrypt(b"mfa secret", crate::AdminUserId::from(1i64)).expect("a128a7a9");
        assert_eq!(cipher.decrypt(&encrypted.ciphertext, &encrypted.nonce, crate::AdminUserId::from(1i64)).expect("65466f55"), b"mfa secret");
        assert!(cipher.decrypt(&encrypted.ciphertext, &encrypted.nonce, crate::AdminUserId::from(2i64)).is_err());
    }
}
