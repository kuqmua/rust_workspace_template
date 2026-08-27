#[path = "admin_button.rs"]
mod admin_button;
#[path = "admin_button_kind.rs"]
mod admin_button_kind;
#[path = "admin_button_link.rs"]
mod admin_button_link;
#[path = "admin_button_variant.rs"]
mod admin_button_variant;

pub(crate) use admin_button::AdminButton;
pub(crate) use admin_button_kind::AdminButtonKind;
pub(crate) use admin_button_link::AdminButtonLink;
pub(crate) use admin_button_variant::AdminButtonVariant;
