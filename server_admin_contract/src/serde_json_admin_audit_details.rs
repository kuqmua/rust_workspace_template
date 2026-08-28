#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
    newtype::AsRefOwned,
    newtype::Display,
    newtype::IntoInnerFrom,
)]
#[serde(try_from = "serde_json::Value", into = "serde_json::Value")]
pub struct SerdeJsonAdminAuditDetails(serde_json::Value);

impl TryFrom<serde_json::Value> for SerdeJsonAdminAuditDetails {
    type Error = super::AdminAuditDetailsTooLarge;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        let actual_bytes = value.to_string().len();
        if actual_bytes > super::ADMIN_AUDIT_DETAILS_MAX_BYTES {
            return Err(super::AdminAuditDetailsTooLarge::from(
                super::AdminAuditDetailsBytes::from(actual_bytes),
            ));
        }
        Ok(Self(value))
    }
}
