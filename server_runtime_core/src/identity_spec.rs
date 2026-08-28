#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq)]
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
