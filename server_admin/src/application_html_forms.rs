pub(crate) use super::admin_html_form_key::*;
pub(crate) use super::admin_html_form_key_error::*;
pub(crate) use super::admin_html_form_selected_max_items::*;
pub(crate) use super::admin_html_form_text::*;
pub(crate) use super::admin_html_form_text_error::*;
pub(crate) use super::change_password_form::*;
pub(crate) use super::create_role_form::*;
pub(crate) use super::create_user_form::*;
pub(crate) use super::revoke_session_form::*;
pub(crate) use super::role_id_form::*;
pub(crate) use super::role_permissions_form::*;
pub(crate) use super::settings_form::*;
pub(crate) use super::sign_in_form::*;
pub(crate) use super::std_admin_html_selected::*;
pub(crate) use super::std_admin_html_selected_error::*;
pub(crate) use super::update_role_form::*;
pub(crate) use super::update_user_form::*;
pub(crate) use super::user_ban_form::*;
pub(crate) use super::user_id_form::*;
pub(crate) use super::user_password_form::*;
pub(crate) use super::user_roles_form::*;
// Root-owned module compatibility wrappers.
mod sign_in_form {
    pub use super::super::sign_in_form::*;
}
mod change_password_form {
    pub use super::super::change_password_form::*;
}
mod revoke_session_form {
    pub use super::super::revoke_session_form::*;
}
mod create_user_form {
    pub use super::super::create_user_form::*;
}
mod update_user_form {
    pub use super::super::update_user_form::*;
}
mod user_password_form {
    pub use super::super::user_password_form::*;
}
mod user_ban_form {
    pub use super::super::user_ban_form::*;
}
mod user_id_form {
    pub use super::super::user_id_form::*;
}
mod user_roles_form {
    pub use super::super::user_roles_form::*;
}
mod create_role_form {
    pub use super::super::create_role_form::*;
}
mod update_role_form {
    pub use super::super::update_role_form::*;
}
mod role_id_form {
    pub use super::super::role_id_form::*;
}
mod role_permissions_form {
    pub use super::super::role_permissions_form::*;
}
mod admin_html_form_text_error {
    pub use super::super::admin_html_form_text_error::*;
}
mod admin_html_form_key_error {
    pub use super::super::admin_html_form_key_error::*;
}
mod std_admin_html_selected_error {
    pub use super::super::std_admin_html_selected_error::*;
}
mod admin_html_form_text {
    pub use super::super::admin_html_form_text::*;
}
mod admin_html_form_key {
    pub use super::super::admin_html_form_key::*;
}
mod std_admin_html_selected {
    pub use super::super::std_admin_html_selected::*;
}
mod settings_form {
    pub use super::super::settings_form::*;
}
mod admin_html_form_selected_max_items {
    pub use super::super::admin_html_form_selected_max_items::*;
}
