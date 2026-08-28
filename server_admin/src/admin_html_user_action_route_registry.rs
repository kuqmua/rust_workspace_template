use super::create_user::create_user;
use super::delete_user::delete_user;
use super::update_user::update_user;
use super::user_ban::user_ban;
use super::user_password::user_password;
use super::user_roles::user_roles;

#[derive(optimal_memory_layout::OptimalMemoryLayout)]
#[frontend_contract::domain_types::endpoint_registry(
    state = super::super::super::super::SharedAdminAuthSvcStateArc;
    (server_admin_contract::domain_types::AdminHtmlAction::UserCreate, create_user),
    (server_admin_contract::domain_types::AdminHtmlAction::UserUpdate, update_user),
    (server_admin_contract::domain_types::AdminHtmlAction::UserPassword, user_password),
    (server_admin_contract::domain_types::AdminHtmlAction::UserBan, user_ban),
    (server_admin_contract::domain_types::AdminHtmlAction::UserDelete, delete_user),
    (server_admin_contract::domain_types::AdminHtmlAction::UserRoles, user_roles),
)]
pub(super) struct AdminHtmlUserActionRouteRegistry;
