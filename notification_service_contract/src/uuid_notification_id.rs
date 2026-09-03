#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    proc_macro_newtype_from_inner::FromInner,
    proc_macro_newtype_into_inner_from::IntoInnerFrom,
    utoipa::ToSchema,
)]
#[serde(from = "uuid::Uuid")]
#[schema(value_type = String, format = "uuid")]
pub struct UuidNotificationId(uuid::Uuid);
