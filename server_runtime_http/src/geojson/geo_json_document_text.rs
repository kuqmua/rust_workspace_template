#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq, newtype::AsRefStr,
)]
pub struct GeoJsonDocumentText(String);

impl TryFrom<String> for GeoJsonDocumentText {
    type Error = super::GeoJsonValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > constants_usize::VALUE_16_777_216 {
            return Err(super::GeoJsonValidationError::TooLarge);
        }
        let json_document =
            serde_json::from_str::<serde_json::Value>(value.as_str()).map_err(|error| {
                super::GeoJsonValidationError::SerdeJson(super::SerdeJsonGeoJsonError(error))
            })?;
        super::SupportedGeoJsonTypeValidation::validate_supported_geo_json_types(&json_document)?;
        let Ok(geo_json_document) = serde_json::from_value::<geojson::GeoJson>(json_document)
        else {
            return Err(super::GeoJsonValidationError::Document);
        };
        super::GeoJsonValidation::validate_geo_json(&geo_json_document)?;
        Ok(Self(value))
    }
}
