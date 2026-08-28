#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "HTML form DTOs are deserialized in this module and consumed by the sibling action adapter"
)]

#[path = "sign_in_form.rs"]
mod sign_in_form;
pub(super) use sign_in_form::*;
#[path = "change_password_form.rs"]
mod change_password_form;
pub(super) use change_password_form::*;
#[path = "revoke_session_form.rs"]
mod revoke_session_form;
pub(super) use revoke_session_form::*;
#[path = "create_user_form.rs"]
mod create_user_form;
pub(super) use create_user_form::*;
#[path = "update_user_form.rs"]
mod update_user_form;
pub(super) use update_user_form::*;
#[path = "user_password_form.rs"]
mod user_password_form;
pub(super) use user_password_form::*;
#[path = "user_ban_form.rs"]
mod user_ban_form;
pub(super) use user_ban_form::*;
#[path = "user_id_form.rs"]
mod user_id_form;
pub(super) use user_id_form::*;
#[path = "user_roles_form.rs"]
mod user_roles_form;
pub(super) use user_roles_form::*;
#[path = "create_role_form.rs"]
mod create_role_form;
pub(super) use create_role_form::*;
#[path = "update_role_form.rs"]
mod update_role_form;
pub(super) use update_role_form::*;
#[path = "role_id_form.rs"]
mod role_id_form;
pub(super) use role_id_form::*;
#[path = "role_permissions_form.rs"]
mod role_permissions_form;
pub(super) use role_permissions_form::*;
#[path = "admin_html_form_text_error.rs"]
mod admin_html_form_text_error;
pub(super) use admin_html_form_text_error::*;
#[path = "admin_html_form_key_error.rs"]
mod admin_html_form_key_error;
pub(super) use admin_html_form_key_error::*;
#[path = "std_admin_html_selected_error.rs"]
mod std_admin_html_selected_error;
pub(super) use std_admin_html_selected_error::*;
#[path = "admin_html_form_text.rs"]
mod admin_html_form_text;
pub(super) use admin_html_form_text::*;
#[path = "admin_html_form_key.rs"]
mod admin_html_form_key;
pub(super) use admin_html_form_key::*;
#[path = "std_admin_html_selected.rs"]
mod std_admin_html_selected;
pub(super) use std_admin_html_selected::*;
#[path = "settings_form.rs"]
mod settings_form;
pub(super) use settings_form::*;
#[path = "admin_html_form_selected_max_items.rs"]
mod admin_html_form_selected_max_items;
pub(super) use admin_html_form_selected_max_items::*;
