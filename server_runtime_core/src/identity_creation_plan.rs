#[cfg(test)]
mod tests {
    #[test]
    fn desired_state_planning_is_idempotent_and_requires_role() {
        assert_eq!(
            crate::plan_identity_creation::plan_identity_creation(
                crate::identity_presence::IdentityPresence::Present,
                crate::identity_role_presence::IdentityRolePresence::Present,
            ),
            crate::identity_creation_decision::IdentityCreationDecision::AlreadyExists
        );
        assert_eq!(
            crate::plan_identity_creation::plan_identity_creation(
                crate::identity_presence::IdentityPresence::Missing,
                crate::identity_role_presence::IdentityRolePresence::Missing,
            ),
            crate::identity_creation_decision::IdentityCreationDecision::MissingRole
        );
        assert_eq!(
            crate::plan_identity_creation::plan_identity_creation(
                crate::identity_presence::IdentityPresence::Missing,
                crate::identity_role_presence::IdentityRolePresence::Present,
            ),
            crate::identity_creation_decision::IdentityCreationDecision::Create
        );
    }

    #[test]
    fn identity_spec_keeps_secret_source_separate_from_identity_fields() {
        let spec = crate::identity_spec::IdentitySpec::new(1u8, 2u8, 3u8, 4u8);
        assert_eq!(spec.login(), &1u8);
        assert_eq!(spec.display_name(), &2u8);
        assert_eq!(spec.role(), &3u8);
        assert_eq!(spec.secret_source(), &4u8);
    }
}
