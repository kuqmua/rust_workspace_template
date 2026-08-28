pub(crate) use crate::admin_input::AdminInput;
pub(crate) use crate::admin_input_group::AdminInputGroup;
pub(crate) use crate::admin_input_kind::AdminInputKind;
pub(crate) use crate::admin_input_name::AdminInputName;
pub(crate) use crate::leptos_admin_input_signal::LeptosAdminInputSignal;

// Root-owned module compatibility wrappers.
pub(crate) mod admin_input {
    pub use crate::admin_input::*;
}
pub(crate) mod admin_input_group {
    pub use crate::admin_input_group::*;
}
pub(crate) mod admin_input_kind {
    pub use crate::admin_input_kind::*;
}
pub(crate) mod admin_input_name {
    pub use crate::admin_input_name::*;
}
pub(crate) mod leptos_admin_input_signal {
    pub use crate::leptos_admin_input_signal::*;
}
