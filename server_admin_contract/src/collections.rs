#[path = "collections/admin_collection_error.rs"]
mod admin_collection_error;
pub use admin_collection_error::*;
#[path = "collections/admin_permission_values.rs"]
mod admin_permission_values;
pub use admin_permission_values::*;
#[path = "collections/admin_role_names.rs"]
mod admin_role_names;
pub use admin_role_names::*;
#[path = "collections/admin_role_ids.rs"]
mod admin_role_ids;
pub use admin_role_ids::*;
#[path = "collections/admin_permission_ids.rs"]
mod admin_permission_ids;
pub use admin_permission_ids::*;
#[path = "collections/admin_user_summaries.rs"]
mod admin_user_summaries;
pub use admin_user_summaries::*;
#[path = "collections/admin_role_summaries.rs"]
mod admin_role_summaries;
pub use admin_role_summaries::*;
#[path = "collections/admin_permission_summaries.rs"]
mod admin_permission_summaries;
pub use admin_permission_summaries::*;
#[path = "collections/admin_audit_views.rs"]
mod admin_audit_views;
pub use admin_audit_views::*;
#[path = "collections/admin_texts.rs"]
mod admin_texts;
pub use admin_texts::*;
#[path = "collections/admin_data_rows.rs"]
mod admin_data_rows;
pub use admin_data_rows::*;
#[path = "collections/admin_data_tables.rs"]
mod admin_data_tables;
pub use admin_data_tables::*;
#[path = "collections/admin_optional_settings.rs"]
mod admin_optional_settings;
pub use admin_optional_settings::*;
#[path = "collections/admin_session_views.rs"]
mod admin_session_views;
pub use admin_session_views::*;
#[path = "collections/admin_bounded_vec.rs"]
mod admin_bounded_vec;
pub(crate) use admin_bounded_vec::*;
#[path = "collections/admin_open_api_vec.rs"]
mod admin_open_api_vec;
pub(crate) use admin_open_api_vec::*;
#[path = "collections/admin_open_api_vec_phantom_data.rs"]
mod admin_open_api_vec_phantom_data;
use admin_open_api_vec_phantom_data::AdminOpenApiVecPhantomData;
#[path = "collections/admin_empty_collection.rs"]
mod admin_empty_collection;
use admin_empty_collection::AdminEmptyCollection;
#[path = "collections/admin_collection_max_items.rs"]
mod admin_collection_max_items;
pub(crate) use admin_collection_max_items::*;
