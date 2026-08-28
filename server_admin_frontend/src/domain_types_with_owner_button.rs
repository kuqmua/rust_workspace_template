pub(crate) use crate::admin_button::AdminButton;
pub(crate) use crate::admin_button_kind::AdminButtonKind;
pub(crate) use crate::admin_button_link::AdminButtonLink;
pub(crate) use crate::admin_button_variant::AdminButtonVariant;

// Root-owned module compatibility wrappers.
pub(crate) mod admin_button {
    pub use crate::admin_button::*;
}
pub(crate) mod admin_button_kind {
    pub use crate::admin_button_kind::*;
}
pub(crate) mod admin_button_link {
    pub use crate::admin_button_link::*;
}
pub(crate) mod admin_button_variant {
    pub use crate::admin_button_variant::*;
}
