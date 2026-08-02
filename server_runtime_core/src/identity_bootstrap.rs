#[derive(optml::Optml, Clone, Debug, Eq, PartialEq)]
pub struct IdentitySpec<Login, DisplayName, Role, SecretSource> {
    display_name: DisplayName,
    login: Login,
    role: Role,
    secret_source: SecretSource,
}

impl<Login, DisplayName, Role, SecretSource> IdentitySpec<Login, DisplayName, Role, SecretSource> {
    #[must_use]
    pub const fn display_name(&self) -> &DisplayName {
        &self.display_name
    }

    #[must_use]
    pub const fn login(&self) -> &Login {
        &self.login
    }

    #[must_use]
    pub const fn new(
        login: Login,
        display_name: DisplayName,
        role: Role,
        secret_source: SecretSource,
    ) -> Self {
        Self {
            display_name,
            login,
            role,
            secret_source,
        }
    }

    #[must_use]
    pub const fn role(&self) -> &Role {
        &self.role
    }

    #[must_use]
    pub const fn secret_source(&self) -> &SecretSource {
        &self.secret_source
    }
}

#[derive(optml::Optml, Clone, Copy, Debug, Eq, PartialEq)]
pub enum IdentityPresence {
    Missing,
    Present,
}

#[derive(optml::Optml, Clone, Copy, Debug, Eq, PartialEq)]
pub enum IdentityRolePresence {
    Missing,
    Present,
}

#[derive(optml::Optml, Clone, Copy, Debug, Eq, PartialEq)]
pub enum IdentityBootstrapDecision {
    AlreadyExists,
    Create,
    MissingRole,
}

#[must_use]
pub const fn plan_identity_bootstrap(
    identity: IdentityPresence,
    role: IdentityRolePresence,
) -> IdentityBootstrapDecision {
    match (identity, role) {
        (
            IdentityPresence::Present,
            IdentityRolePresence::Missing | IdentityRolePresence::Present,
        ) => IdentityBootstrapDecision::AlreadyExists,
        (IdentityPresence::Missing, IdentityRolePresence::Missing) => {
            IdentityBootstrapDecision::MissingRole
        }
        (IdentityPresence::Missing, IdentityRolePresence::Present) => {
            IdentityBootstrapDecision::Create
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn desired_state_planning_is_idempotent_and_requires_role() {
        assert_eq!(
            super::plan_identity_bootstrap(
                super::IdentityPresence::Present,
                super::IdentityRolePresence::Present,
            ),
            super::IdentityBootstrapDecision::AlreadyExists
        );
        assert_eq!(
            super::plan_identity_bootstrap(
                super::IdentityPresence::Missing,
                super::IdentityRolePresence::Missing,
            ),
            super::IdentityBootstrapDecision::MissingRole
        );
        assert_eq!(
            super::plan_identity_bootstrap(
                super::IdentityPresence::Missing,
                super::IdentityRolePresence::Present,
            ),
            super::IdentityBootstrapDecision::Create
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
