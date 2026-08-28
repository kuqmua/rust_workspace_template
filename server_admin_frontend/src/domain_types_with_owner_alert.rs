pub(crate) use crate::admin_alert::AdminAlert;
pub(crate) use crate::admin_alert_variant::AdminAlertVariant;

// Root-owned module compatibility wrappers.
pub(crate) mod admin_alert {
    pub use crate::admin_alert::*;
}
pub(crate) mod admin_alert_variant {
    pub use crate::admin_alert_variant::*;
}
