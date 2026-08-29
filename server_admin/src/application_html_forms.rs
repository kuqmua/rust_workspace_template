pub(crate) use admin_html_form_key::*;
pub(crate) use admin_html_form_key_error::*;
pub(crate) use admin_html_form_selected_max_items::*;
pub(crate) use admin_html_form_text::*;
pub(crate) use admin_html_form_text_error::*;
pub(crate) use change_password_form::*;
pub(crate) use create_role_form::*;
pub(crate) use create_user_form::*;
pub(crate) use revoke_session_form::*;
pub(crate) use role_id_form::*;
pub(crate) use role_permissions_form::*;
pub(crate) use settings_form::*;
pub(crate) use sign_in_form::*;
pub(crate) use std_admin_html_selected::*;
pub(crate) use std_admin_html_selected_error::*;
pub(crate) use update_role_form::*;
pub(crate) use update_user_form::*;
pub(crate) use user_ban_form::*;
pub(crate) use user_id_form::*;
pub(crate) use user_password_form::*;
pub(crate) use user_roles_form::*;

// Root-owned module compatibility wrappers.
mod sign_in_form {
    pub use crate::sign_in_form::*;
}
mod change_password_form {
    pub use crate::change_password_form::*;
}
mod revoke_session_form {
    pub use crate::revoke_session_form::*;
}
mod create_user_form {
    pub use crate::create_user_form::*;
}
mod update_user_form {
    pub use crate::update_user_form::*;
}
mod user_password_form {
    pub use crate::user_password_form::*;
}
mod user_ban_form {
    pub use crate::user_ban_form::*;
}
mod user_id_form {
    pub use crate::user_id_form::*;
}
mod user_roles_form {
    pub use crate::user_roles_form::*;
}
mod create_role_form {
    pub use crate::create_role_form::*;
}
mod update_role_form {
    pub use crate::update_role_form::*;
}
mod role_id_form {
    pub use crate::role_id_form::*;
}
mod role_permissions_form {
    pub use crate::role_permissions_form::*;
}
mod admin_html_form_text_error {
    pub use crate::admin_html_form_text_error::*;
}
mod admin_html_form_key_error {
    pub use crate::admin_html_form_key_error::*;
}
mod std_admin_html_selected_error {
    pub use crate::std_admin_html_selected_error::*;
}
mod admin_html_form_text {
    pub use crate::admin_html_form_text::*;
}
mod admin_html_form_key {
    pub use crate::admin_html_form_key::*;
}
mod std_admin_html_selected {
    pub use crate::std_admin_html_selected::*;
}
mod settings_form {
    pub use crate::settings_form::*;
}
mod admin_html_form_selected_max_items {
    pub use crate::admin_html_form_selected_max_items::*;
}
