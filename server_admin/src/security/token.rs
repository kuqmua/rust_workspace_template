use super::*;
pub fn token(token: &AdminOpaqueToken) -> Result<AdminTokenHash, AdminSecretTextError> {
    super::super::hash_opaque_token::hash_opaque_token(token)
}
