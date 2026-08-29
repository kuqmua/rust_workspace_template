pub(crate) use super::admin_button::AdminButton;
pub(crate) use super::admin_button_kind::AdminButtonKind;
pub(crate) use super::admin_button_link::AdminButtonLink;
pub(crate) use super::admin_button_variant::AdminButtonVariant;
// Root-owned module compatibility wrappers.
pub(crate) mod admin_button {
    pub use super::super::admin_button::*;
}
pub(crate) mod admin_button_kind {
    pub use super::super::admin_button_kind::*;
}
pub(crate) mod admin_button_link {
    pub use super::super::admin_button_link::*;
}
pub(crate) mod admin_button_variant {
    pub use super::super::admin_button_variant::*;
}
