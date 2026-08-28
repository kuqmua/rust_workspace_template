pub use crate::identity_creation_decision::IdentityCreationDecision;
pub use crate::identity_presence::IdentityPresence;
pub use crate::identity_role_presence::IdentityRolePresence;
pub use crate::identity_spec::IdentitySpec;
pub use crate::plan_identity_creation::plan_identity_creation;

#[cfg(test)]
mod tests {
    #[test]
    fn desired_state_planning_is_idempotent_and_requires_role() {
        assert_eq!(
            super::plan_identity_creation(
                super::IdentityPresence::Present,
                super::IdentityRolePresence::Present,
            ),
            super::IdentityCreationDecision::AlreadyExists
        );
        assert_eq!(
            super::plan_identity_creation(
                super::IdentityPresence::Missing,
                super::IdentityRolePresence::Missing,
            ),
            super::IdentityCreationDecision::MissingRole
        );
        assert_eq!(
            super::plan_identity_creation(
                super::IdentityPresence::Missing,
                super::IdentityRolePresence::Present,
            ),
            super::IdentityCreationDecision::Create
        );
    }

    #[test]
    fn identity_spec_keeps_secret_source_separate_from_identity_fields() {
        let spec = super::IdentitySpec::new(1u8, 2u8, 3u8, 4u8);
        assert_eq!(spec.login(), &1u8);
        assert_eq!(spec.display_name(), &2u8);
        assert_eq!(spec.role(), &3u8);
        assert_eq!(spec.secret_source(), &4u8);
    }
}
