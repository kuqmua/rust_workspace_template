pub(super) trait SupportedGeoJsonTypeValidation {
    fn validate_supported_geo_json_types(
        &self,
    ) -> Result<(), crate::geo_json_validation_error::GeoJsonValidationError>;
}

impl SupportedGeoJsonTypeValidation for serde_json::Value {
    fn validate_supported_geo_json_types(
        &self,
    ) -> Result<(), crate::geo_json_validation_error::GeoJsonValidationError> {
        let object = self
            .as_object()
            .ok_or(crate::geo_json_validation_error::GeoJsonValidationError::Document)?;
        let value_type = object
            .get(constants_str::GEO_JSON_TYPE)
            .and_then(Self::as_str)
            .ok_or(crate::geo_json_validation_error::GeoJsonValidationError::Document)?;
        match value_type {
            constants_str::GEO_JSON_FEATURE => object
                .get(constants_str::GEO_JSON_GEOMETRY)
                .filter(|geometry| !geometry.is_null())
                .into_iter()
                .try_for_each(Self::validate_supported_geo_json_types),
            constants_str::GEO_JSON_FEATURE_COLLECTION => object
                .get(constants_str::GEO_JSON_FEATURES)
                .and_then(Self::as_array)
                .ok_or(crate::geo_json_validation_error::GeoJsonValidationError::Document)?
                .iter()
                .try_for_each(Self::validate_supported_geo_json_types),
            constants_str::GEO_JSON_GEOMETRY_COLLECTION => object
                .get(constants_str::GEO_JSON_GEOMETRIES)
                .and_then(Self::as_array)
                .ok_or(crate::geo_json_validation_error::GeoJsonValidationError::Document)?
                .iter()
                .try_for_each(Self::validate_supported_geo_json_types),
            constants_str::GEO_JSON_POINT
            | constants_str::GEO_JSON_LINE_STRING
            | constants_str::GEO_JSON_MULTI_POINT
            | constants_str::GEO_JSON_MULTI_LINE_STRING
            | constants_str::GEO_JSON_POLYGON
            | constants_str::GEO_JSON_MULTI_POLYGON => Ok(()),
            _ => Err(crate::geo_json_validation_error::GeoJsonValidationError::UnsupportedGeometry),
        }
    }
}
