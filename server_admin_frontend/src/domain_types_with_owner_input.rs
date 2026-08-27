#[path = "admin_input.rs"]
mod admin_input;
#[path = "admin_input_group.rs"]
mod admin_input_group;
#[path = "admin_input_kind.rs"]
mod admin_input_kind;
#[path = "admin_input_name.rs"]
mod admin_input_name;
#[path = "leptos_admin_input_signal.rs"]
mod leptos_admin_input_signal;

pub(crate) use admin_input::AdminInput;
pub(crate) use admin_input_group::AdminInputGroup;
pub(crate) use admin_input_kind::AdminInputKind;
pub(crate) use admin_input_name::AdminInputName;
pub(crate) use leptos_admin_input_signal::LeptosAdminInputSignal;
