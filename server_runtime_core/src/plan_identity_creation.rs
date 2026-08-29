#[must_use]
pub const fn plan_identity_creation(
    identity: crate::identity_presence::IdentityPresence,
    role: crate::identity_role_presence::IdentityRolePresence,
) -> crate::identity_creation_decision::IdentityCreationDecision {
    match (identity, role) {
        (
            crate::identity_presence::IdentityPresence::Present,
            crate::identity_role_presence::IdentityRolePresence::Missing
            | crate::identity_role_presence::IdentityRolePresence::Present,
        ) => crate::identity_creation_decision::IdentityCreationDecision::AlreadyExists,
        (
            crate::identity_presence::IdentityPresence::Missing,
            crate::identity_role_presence::IdentityRolePresence::Missing,
        ) => crate::identity_creation_decision::IdentityCreationDecision::MissingRole,
        (
            crate::identity_presence::IdentityPresence::Missing,
            crate::identity_role_presence::IdentityRolePresence::Present,
        ) => crate::identity_creation_decision::IdentityCreationDecision::Create,
    }
}
