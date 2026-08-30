#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq, newtype::AsRefStr,
)]
pub struct GeoJsonDocumentText(String);

impl TryFrom<String> for GeoJsonDocumentText {
    type Error = crate::geo_json_validation_error::GeoJsonValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > constants_usize::VALUE_16_777_216 {
            return Err(crate::geo_json_validation_error::GeoJsonValidationError::TooLarge);
        }
        let json_document =
            serde_json::from_str::<serde_json::Value>(value.as_str()).map_err(|error| {
                crate::geo_json_validation_error::GeoJsonValidationError::SerdeJson(
                    crate::serde_json_geo_json_error::SerdeJsonGeoJsonError::from(error),
                )
            })?;
        crate::supported_geo_json_type_validation::SupportedGeoJsonTypeValidation::validate_supported_geo_json_types(&json_document)?;
        let Ok(geo_json_document) = serde_json::from_value::<geojson::GeoJson>(json_document)
        else {
            return Err(crate::geo_json_validation_error::GeoJsonValidationError::Document);
        };
        crate::geo_json_validation::GeoJsonValidation::validate_geo_json(&geo_json_document)?;
        Ok(Self(value))
    }
}
