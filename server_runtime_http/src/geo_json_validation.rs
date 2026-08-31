pub(super) trait GeoJsonValidation {
    fn validate_geo_json(
        &self,
    ) -> Result<(), crate::geo_json_validation_error::GeoJsonValidationError>;
}

impl GeoJsonValidation for geojson::GeoJson {
    fn validate_geo_json(
        &self,
    ) -> Result<(), crate::geo_json_validation_error::GeoJsonValidationError> {
        match self {
            Self::Feature(feature) => feature.validate_geo_json(),
            Self::FeatureCollection(collection) => collection
                .features
                .iter()
                .try_for_each(GeoJsonValidation::validate_geo_json),
            Self::Geometry(geometry) => geometry.validate_geo_json(),
        }
    }
}

impl GeoJsonValidation for geojson::Feature {
    fn validate_geo_json(
        &self,
    ) -> Result<(), crate::geo_json_validation_error::GeoJsonValidationError> {
        self.geometry
            .as_ref()
            .map_or(Ok(()), GeoJsonValidation::validate_geo_json)
    }
}

impl GeoJsonValidation for geojson::Geometry {
    fn validate_geo_json(
        &self,
    ) -> Result<(), crate::geo_json_validation_error::GeoJsonValidationError> {
        match &self.value {
            geojson::GeometryValue::Point { coordinates } => coordinates.validate_geo_json(),
            geojson::GeometryValue::LineString { coordinates }
            | geojson::GeometryValue::MultiPoint { coordinates } => coordinates.validate_geo_json(),
            geojson::GeometryValue::MultiLineString { coordinates }
            | geojson::GeometryValue::Polygon { coordinates } => coordinates.validate_geo_json(),
            geojson::GeometryValue::MultiPolygon { coordinates } => coordinates.validate_geo_json(),
            geojson::GeometryValue::GeometryCollection { geometries } => geometries
                .iter()
                .try_for_each(GeoJsonValidation::validate_geo_json),
        }
    }
}

impl GeoJsonValidation for geojson::Position {
    fn validate_geo_json(
        &self,
    ) -> Result<(), crate::geo_json_validation_error::GeoJsonValidationError> {
        let longitude_valid = self.as_slice().first().is_some_and(|coordinate| {
            coordinate.is_finite() && (-180.0f64..=180.0f64).contains(coordinate)
        });
        let latitude_valid = self
            .as_slice()
            .get(constants_usize::ONE)
            .is_some_and(|coordinate| {
                coordinate.is_finite() && (-90.0f64..=90.0f64).contains(coordinate)
            });
        if longitude_valid && latitude_valid {
            Ok(())
        } else {
            Err(crate::geo_json_validation_error::GeoJsonValidationError::Coordinates)
        }
    }
}

impl GeoJsonValidation for Vec<geojson::Position> {
    fn validate_geo_json(
        &self,
    ) -> Result<(), crate::geo_json_validation_error::GeoJsonValidationError> {
        if self.is_empty() {
            return Err(crate::geo_json_validation_error::GeoJsonValidationError::Coordinates);
        }
        self.iter()
            .try_for_each(GeoJsonValidation::validate_geo_json)
    }
}

impl GeoJsonValidation for Vec<Vec<geojson::Position>> {
    fn validate_geo_json(
        &self,
    ) -> Result<(), crate::geo_json_validation_error::GeoJsonValidationError> {
        if self.is_empty() {
            return Err(crate::geo_json_validation_error::GeoJsonValidationError::Coordinates);
        }
        self.iter()
            .try_for_each(GeoJsonValidation::validate_geo_json)
    }
}

impl GeoJsonValidation for Vec<Vec<Vec<geojson::Position>>> {
    fn validate_geo_json(
        &self,
    ) -> Result<(), crate::geo_json_validation_error::GeoJsonValidationError> {
        if self.is_empty() {
            return Err(crate::geo_json_validation_error::GeoJsonValidationError::Coordinates);
        }
        self.iter()
            .try_for_each(GeoJsonValidation::validate_geo_json)
    }
}

#[cfg(test)]
mod tests {
    fn document(
        value: &serde_json::Value,
    ) -> Result<
        crate::geo_json_document_text::GeoJsonDocumentText,
        crate::geo_json_validation_error::GeoJsonValidationError,
    > {
        crate::geo_json_document_text::GeoJsonDocumentText::try_from(value.to_string())
    }
    #[test]
    fn test_document_validation_distinguishes_text_shape_type_and_size_errors() {
        assert!(matches!(
            crate::geo_json_document_text::GeoJsonDocumentText::try_from(String::from(
                constants_str::TEST_INVALID_JSON
            )),
            Err(crate::geo_json_validation_error::GeoJsonValidationError::SerdeJson(_))
        ));
        assert!(matches!(
            document(&serde_json::json!([])),
            Err(crate::geo_json_validation_error::GeoJsonValidationError::Document)
        ));
        assert!(matches!(
            document(&serde_json::json!({
                constants_str::GEO_JSON_TYPE: "Unsupported"
            })),
            Err(crate::geo_json_validation_error::GeoJsonValidationError::UnsupportedGeometry)
        ));
        assert!(matches!(
            crate::geo_json_document_text::GeoJsonDocumentText::try_from(
                " ".repeat(constants_usize::VALUE_16_777_216 + constants_usize::ONE)
            ),
            Err(crate::geo_json_validation_error::GeoJsonValidationError::TooLarge)
        ));
    }
    #[test]
    fn test_feature_and_geometry_collections_validate_children_recursively() {
        let properties = constants_str::PROPERTIES;
        let feature = serde_json::json!({
            constants_str::GEO_JSON_TYPE: constants_str::GEO_JSON_FEATURE,
            constants_str::GEO_JSON_GEOMETRY: null,
            (properties): {}
        });
        let _feature = document(&feature).expect("c0bd64d6 feature_and_geometry_collections_validate_children_recursively invariant must hold");
        let collection = serde_json::json!({
            constants_str::GEO_JSON_TYPE: constants_str::GEO_JSON_FEATURE_COLLECTION,
            constants_str::GEO_JSON_FEATURES: [{
                constants_str::GEO_JSON_TYPE: constants_str::GEO_JSON_FEATURE,
                constants_str::GEO_JSON_GEOMETRY: {
                    constants_str::GEO_JSON_TYPE: constants_str::GEO_JSON_POINT,
                    constants_str::GEO_JSON_COORDINATES: [-180.0f64, 90.0f64]
                },
                (properties): {}
            }]
        });
        let _collection = document(&collection).expect("bc4861b1 feature_and_geometry_collections_validate_children_recursively invariant must hold");
        let geometry_collection = serde_json::json!({
            constants_str::GEO_JSON_TYPE: constants_str::GEO_JSON_GEOMETRY_COLLECTION,
            constants_str::GEO_JSON_GEOMETRIES: [{
                constants_str::GEO_JSON_TYPE: constants_str::GEO_JSON_POINT,
                constants_str::GEO_JSON_COORDINATES: [180.0f64, -90.0f64]
            }]
        });
        let _geometry_collection = document(&geometry_collection).expect("ba7f5e93 feature_and_geometry_collections_validate_children_recursively invariant must hold");
    }
    #[test]
    fn test_coordinate_collections_reject_empty_levels() {
        let line = serde_json::json!({
            constants_str::GEO_JSON_TYPE: constants_str::GEO_JSON_LINE_STRING,
            constants_str::GEO_JSON_COORDINATES: []
        });
        assert!(matches!(
            document(&line),
            Err(crate::geo_json_validation_error::GeoJsonValidationError::Coordinates)
        ));
        let polygon = serde_json::json!({
            constants_str::GEO_JSON_TYPE: constants_str::GEO_JSON_POLYGON,
            constants_str::GEO_JSON_COORDINATES: []
        });
        assert!(matches!(
            document(&polygon),
            Err(crate::geo_json_validation_error::GeoJsonValidationError::Coordinates)
        ));
        let multi_polygon = serde_json::json!({
            constants_str::GEO_JSON_TYPE: constants_str::GEO_JSON_MULTI_POLYGON,
            constants_str::GEO_JSON_COORDINATES: []
        });
        assert!(matches!(
            document(&multi_polygon),
            Err(crate::geo_json_validation_error::GeoJsonValidationError::Coordinates)
        ));
    }
    #[test]
    fn test_point_coordinates_are_range_checked() {
        let _document = crate::geo_json_document_text::GeoJsonDocumentText::try_from(String::from(
            constants_str::TEST_GEO_JSON_POINT,
        ))
        .expect("34818d2e point_coordinates_are_range_checked invariant must hold");
        assert!(matches!(
            crate::geo_json_document_text::GeoJsonDocumentText::try_from(String::from(
                constants_str::TEST_GEO_JSON_INVALID_POINT
            )),
            Err(crate::geo_json_validation_error::GeoJsonValidationError::Coordinates)
        ));
    }
}
