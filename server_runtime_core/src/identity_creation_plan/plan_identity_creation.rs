#[must_use]
pub const fn plan_identity_creation(
    identity: super::IdentityPresence,
    role: super::IdentityRolePresence,
) -> super::IdentityCreationDecision {
    match (identity, role) {
        (
            super::IdentityPresence::Present,
            super::IdentityRolePresence::Missing | super::IdentityRolePresence::Present,
        ) => super::IdentityCreationDecision::AlreadyExists,
        (super::IdentityPresence::Missing, super::IdentityRolePresence::Missing) => {
            super::IdentityCreationDecision::MissingRole
        }
        (super::IdentityPresence::Missing, super::IdentityRolePresence::Present) => {
            super::IdentityCreationDecision::Create
        }
    }
}
