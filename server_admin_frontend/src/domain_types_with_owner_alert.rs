pub(crate) use super::admin_alert::AdminAlert;
pub(crate) use super::admin_alert_variant::AdminAlertVariant;
// Root-owned module compatibility wrappers.
pub(crate) mod admin_alert {
    pub use super::super::admin_alert::*;
}
pub(crate) mod admin_alert_variant {
    pub use super::super::admin_alert_variant::*;
}
