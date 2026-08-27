#[path = "admin_card.rs"]
mod admin_card;
#[path = "admin_card_description.rs"]
mod admin_card_description;
#[path = "admin_card_footer.rs"]
mod admin_card_footer;
#[path = "admin_card_header.rs"]
mod admin_card_header;
#[path = "admin_card_title.rs"]
mod admin_card_title;
#[path = "admin_card_variant.rs"]
mod admin_card_variant;

pub(crate) use admin_card::AdminCard;
#[cfg(not(target_arch = "wasm32"))]
pub(crate) use admin_card_description::AdminCardDescription;
pub(crate) use admin_card_footer::AdminCardFooter;
pub(crate) use admin_card_header::AdminCardHeader;
pub(crate) use admin_card_title::AdminCardTitle;
pub(crate) use admin_card_variant::AdminCardVariant;
