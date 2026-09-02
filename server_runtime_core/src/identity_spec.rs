#[derive(proc_macro_getters::Getters)]
#[getters(bare)]
#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq)]
pub struct IdentitySpec<Login, DisplayName, Role, SecretSource> {
    display_name: DisplayName,
    login: Login,
    role: Role,
    secret_source: SecretSource,
}

impl<Login, DisplayName, Role, SecretSource> IdentitySpec<Login, DisplayName, Role, SecretSource> {
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
}
