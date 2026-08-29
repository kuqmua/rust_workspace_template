use super::*;
pub fn token(token: &AdminOpaqueToken) -> Result<AdminTokenHash, AdminSecretTextError> {
    hash_opaque_token(token)
}
