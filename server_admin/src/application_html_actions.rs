use assignment_action::assignment_action;
pub(crate) use root::AdminHtmlActionRouteRegistry;
#[cfg(test)]
pub(crate) use root::root;

// Root-owned module compatibility wrappers.
mod auth {
    pub use crate::application_html_actions_auth::*;
}
mod roles {
    pub use crate::application_html_actions_roles::*;
}
mod sessions {
    pub use crate::application_html_actions_sessions::*;
}
mod settings {
    pub use crate::application_html_actions_settings::*;
}
mod users {
    pub use crate::application_html_actions_users::*;
}
mod assignment_action {
    pub use crate::assignment_action::*;
}
mod root {
    pub use crate::root::*;
}
