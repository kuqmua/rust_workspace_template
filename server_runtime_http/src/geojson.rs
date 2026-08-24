const GEO_JSON_MAXIMUM_BYTES: usize = 16_777_216usize;

#[derive(optml::Optml, Clone, Debug, Eq, PartialEq, newtype::AsRefStr)]
pub struct GeoJsonDocumentText(String);
impl TryFrom<String> for GeoJsonDocumentText {
    type Error = GeoJsonValidationError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > GEO_JSON_MAXIMUM_BYTES {
            return Err(GeoJsonValidationError::TooLarge);
        }
        let json_document = serde_json::from_str::<serde_json::Value>(value.as_str())
            .map_err(|error| GeoJsonValidationError::SerdeJson(SerdeJsonGeoJsonError(error)))?;
        SupportedGeoJsonTypeValidation::validate_supported_geo_json_types(&json_document)?;
        let Ok(geo_json_document) = serde_json::from_value::<geojson::GeoJson>(json_document)
        else {
            return Err(GeoJsonValidationError::Document);
        };
        GeoJsonValidation::validate_geo_json(&geo_json_document)?;
        Ok(Self(value))
    }
}

#[derive(optml::Optml, Debug, thiserror::Error, newtype::FromInner)]
#[error("{0}")]
pub struct SerdeJsonGeoJsonError(serde_json::Error);

#[derive(optml::Optml, Debug, thiserror::Error)]
pub enum GeoJsonValidationError {
    #[error("GeoJSON coordinates are invalid")]
    Coordinates,
    #[error("GeoJSON document shape is invalid")]
    Document,
    #[error("GeoJSON text is invalid")]
    SerdeJson(#[source] SerdeJsonGeoJsonError),
    #[error("GeoJSON document exceeds its maximum size")]
    TooLarge,
    #[error("GeoJSON geometry type is unsupported")]
    UnsupportedGeometry,
}

trait GeoJsonValidation {
    fn validate_geo_json(&self) -> Result<(), GeoJsonValidationError>;
}

trait SupportedGeoJsonTypeValidation {
    fn validate_supported_geo_json_types(&self) -> Result<(), GeoJsonValidationError>;
}

impl GeoJsonValidation for geojson::GeoJson {
    fn validate_geo_json(&self) -> Result<(), GeoJsonValidationError> {
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
    fn validate_geo_json(&self) -> Result<(), GeoJsonValidationError> {
        self.geometry
            .as_ref()
            .map_or(Ok(()), GeoJsonValidation::validate_geo_json)
    }
}

impl GeoJsonValidation for geojson::Geometry {
    fn validate_geo_json(&self) -> Result<(), GeoJsonValidationError> {
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
    fn validate_geo_json(&self) -> Result<(), GeoJsonValidationError> {
        let longitude_valid = self.as_slice().first().is_some_and(|coordinate| {
            coordinate.is_finite() && (-180.0f64..=180.0f64).contains(coordinate)
        });
        let latitude_valid = self.as_slice().get(1usize).is_some_and(|coordinate| {
            coordinate.is_finite() && (-90.0f64..=90.0f64).contains(coordinate)
        });
        if longitude_valid && latitude_valid {
            Ok(())
        } else {
            Err(GeoJsonValidationError::Coordinates)
        }
    }
}

impl GeoJsonValidation for Vec<geojson::Position> {
    fn validate_geo_json(&self) -> Result<(), GeoJsonValidationError> {
        if self.is_empty() {
            return Err(GeoJsonValidationError::Coordinates);
        }
        self.iter()
            .try_for_each(GeoJsonValidation::validate_geo_json)
    }
}

impl GeoJsonValidation for Vec<Vec<geojson::Position>> {
    fn validate_geo_json(&self) -> Result<(), GeoJsonValidationError> {
        if self.is_empty() {
            return Err(GeoJsonValidationError::Coordinates);
        }
        self.iter()
            .try_for_each(GeoJsonValidation::validate_geo_json)
    }
}

impl GeoJsonValidation for Vec<Vec<Vec<geojson::Position>>> {
    fn validate_geo_json(&self) -> Result<(), GeoJsonValidationError> {
        if self.is_empty() {
            return Err(GeoJsonValidationError::Coordinates);
        }
        self.iter()
            .try_for_each(GeoJsonValidation::validate_geo_json)
    }
}

impl SupportedGeoJsonTypeValidation for serde_json::Value {
    fn validate_supported_geo_json_types(&self) -> Result<(), GeoJsonValidationError> {
        let object = self.as_object().ok_or(GeoJsonValidationError::Document)?;
        let value_type = object
            .get(str_constants::GEO_JSON_TYPE)
            .and_then(Self::as_str)
            .ok_or(GeoJsonValidationError::Document)?;
        match value_type {
            str_constants::GEO_JSON_FEATURE => object
                .get(str_constants::GEO_JSON_GEOMETRY)
                .filter(|geometry| !geometry.is_null())
                .into_iter()
                .try_for_each(Self::validate_supported_geo_json_types),
            str_constants::GEO_JSON_FEATURE_COLLECTION => object
                .get(str_constants::GEO_JSON_FEATURES)
                .and_then(Self::as_array)
                .ok_or(GeoJsonValidationError::Document)?
                .iter()
                .try_for_each(Self::validate_supported_geo_json_types),
            str_constants::GEO_JSON_GEOMETRY_COLLECTION => object
                .get(str_constants::GEO_JSON_GEOMETRIES)
                .and_then(Self::as_array)
                .ok_or(GeoJsonValidationError::Document)?
                .iter()
                .try_for_each(Self::validate_supported_geo_json_types),
            str_constants::GEO_JSON_POINT
            | str_constants::GEO_JSON_LINE_STRING
            | str_constants::GEO_JSON_MULTI_POINT
            | str_constants::GEO_JSON_MULTI_LINE_STRING
            | str_constants::GEO_JSON_POLYGON
            | str_constants::GEO_JSON_MULTI_POLYGON => Ok(()),
            _ => Err(GeoJsonValidationError::UnsupportedGeometry),
        }
    }
}

#[cfg(test)]
mod tests {
    fn document(
        value: &serde_json::Value,
    ) -> Result<super::GeoJsonDocumentText, super::GeoJsonValidationError> {
        super::GeoJsonDocumentText::try_from(value.to_string())
    }
    #[test]
    fn document_validation_distinguishes_text_shape_type_and_size_errors() {
        assert!(matches!(
            super::GeoJsonDocumentText::try_from(String::from(str_constants::TEST_INVALID_JSON)),
            Err(super::GeoJsonValidationError::SerdeJson(_))
        ));
        assert!(matches!(
            document(&serde_json::json!([])),
            Err(super::GeoJsonValidationError::Document)
        ));
        assert!(matches!(
            document(&serde_json::json!({
                str_constants::GEO_JSON_TYPE: "Unsupported"
            })),
            Err(super::GeoJsonValidationError::UnsupportedGeometry)
        ));
        assert!(matches!(
            super::GeoJsonDocumentText::try_from(
                " ".repeat(super::GEO_JSON_MAXIMUM_BYTES + 1usize)
            ),
            Err(super::GeoJsonValidationError::TooLarge)
        ));
    }
    #[test]
    fn feature_and_geometry_collections_validate_children_recursively() {
        let properties = "properties";
        let feature = serde_json::json!({
            str_constants::GEO_JSON_TYPE: str_constants::GEO_JSON_FEATURE,
            str_constants::GEO_JSON_GEOMETRY: null,
            (properties): {}
        });
        let _feature = document(&feature).expect("c0bd64d6");
        let collection = serde_json::json!({
            str_constants::GEO_JSON_TYPE: str_constants::GEO_JSON_FEATURE_COLLECTION,
            str_constants::GEO_JSON_FEATURES: [{
                str_constants::GEO_JSON_TYPE: str_constants::GEO_JSON_FEATURE,
                str_constants::GEO_JSON_GEOMETRY: {
                    str_constants::GEO_JSON_TYPE: str_constants::GEO_JSON_POINT,
                    str_constants::GEO_JSON_COORDINATES: [-180.0f64, 90.0f64]
                },
                (properties): {}
            }]
        });
        let _collection = document(&collection).expect("bc4861b1");
        let geometry_collection = serde_json::json!({
            str_constants::GEO_JSON_TYPE: str_constants::GEO_JSON_GEOMETRY_COLLECTION,
            str_constants::GEO_JSON_GEOMETRIES: [{
                str_constants::GEO_JSON_TYPE: str_constants::GEO_JSON_POINT,
                str_constants::GEO_JSON_COORDINATES: [180.0f64, -90.0f64]
            }]
        });
        let _geometry_collection = document(&geometry_collection).expect("ba7f5e93");
    }
    #[test]
    fn coordinate_collections_reject_empty_levels() {
        let line = serde_json::json!({
            str_constants::GEO_JSON_TYPE: str_constants::GEO_JSON_LINE_STRING,
            str_constants::GEO_JSON_COORDINATES: []
        });
        assert!(matches!(
            document(&line),
            Err(super::GeoJsonValidationError::Coordinates)
        ));
        let polygon = serde_json::json!({
            str_constants::GEO_JSON_TYPE: str_constants::GEO_JSON_POLYGON,
            str_constants::GEO_JSON_COORDINATES: []
        });
        assert!(matches!(
            document(&polygon),
            Err(super::GeoJsonValidationError::Coordinates)
        ));
        let multi_polygon = serde_json::json!({
            str_constants::GEO_JSON_TYPE: str_constants::GEO_JSON_MULTI_POLYGON,
            str_constants::GEO_JSON_COORDINATES: []
        });
        assert!(matches!(
            document(&multi_polygon),
            Err(super::GeoJsonValidationError::Coordinates)
        ));
    }
    #[test]
    fn point_coordinates_are_range_checked() {
        let _document =
            super::GeoJsonDocumentText::try_from(String::from(str_constants::TEST_GEO_JSON_POINT))
                .expect("34818d2e");
        assert!(matches!(
            super::GeoJsonDocumentText::try_from(String::from(
                str_constants::TEST_GEO_JSON_INVALID_POINT
            )),
            Err(super::GeoJsonValidationError::Coordinates)
        ));
    }
}
