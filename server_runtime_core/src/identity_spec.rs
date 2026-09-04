#[derive(proc_macro_getters::Getters)]
#[getters(bare)]
#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    PartialEq,
    proc_macro_new::New,
)]
pub struct IdentitySpec<Login, DisplayName, Role, SecretSource> {
    #[constructor(order = 1)]
    display_name: DisplayName,
    #[constructor(order = 0)]
    login: Login,
    #[constructor(order = 2)]
    role: Role,
    #[constructor(order = 3)]
    secret_source: SecretSource,
}
