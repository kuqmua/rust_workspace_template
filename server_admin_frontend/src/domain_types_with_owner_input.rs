pub(crate) use super::admin_input::AdminInput;
pub(crate) use super::admin_input_group::AdminInputGroup;
pub(crate) use super::admin_input_kind::AdminInputKind;
pub(crate) use super::admin_input_name::AdminInputName;
pub(crate) use super::leptos_admin_input_signal::LeptosAdminInputSignal;
// Root-owned module compatibility wrappers.
pub(crate) mod admin_input {
    pub use super::super::admin_input::*;
}
pub(crate) mod admin_input_group {
    pub use super::super::admin_input_group::*;
}
pub(crate) mod admin_input_kind {
    pub use super::super::admin_input_kind::*;
}
pub(crate) mod admin_input_name {
    pub use super::super::admin_input_name::*;
}
pub(crate) mod leptos_admin_input_signal {
    pub use super::super::leptos_admin_input_signal::*;
}
