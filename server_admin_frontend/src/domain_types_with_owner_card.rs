pub(crate) use super::admin_card::AdminCard;
#[cfg(not(target_arch = "wasm32"))]
pub(crate) use super::admin_card_description::AdminCardDescription;
pub(crate) use super::admin_card_footer::AdminCardFooter;
pub(crate) use super::admin_card_header::AdminCardHeader;
pub(crate) use super::admin_card_title::AdminCardTitle;
pub(crate) use super::admin_card_variant::AdminCardVariant;
// Root-owned module compatibility wrappers.
pub(crate) mod admin_card {
    pub use super::super::admin_card::*;
}
pub(crate) mod admin_card_description {
    pub use super::super::admin_card_description::*;
}
pub(crate) mod admin_card_footer {
    pub use super::super::admin_card_footer::*;
}
pub(crate) mod admin_card_header {
    pub use super::super::admin_card_header::*;
}
pub(crate) mod admin_card_title {
    pub use super::super::admin_card_title::*;
}
pub(crate) mod admin_card_variant {
    pub use super::super::admin_card_variant::*;
}
