const GEO_JSON_MAXIMUM_BYTES: usize = 16_777_216usize;

#[derive(Clone, Debug, Eq, PartialEq, newtype::AsRefStr)]
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

#[derive(Debug, thiserror::Error, newtype::FromInner)]
#[error("{0}")]
pub struct SerdeJsonGeoJsonError(serde_json::Error);

#[derive(Debug, thiserror::Error)]
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
        let children = match value_type {
            str_constants::GEO_JSON_FEATURE => object
                .get(str_constants::GEO_JSON_GEOMETRY)
                .filter(|geometry| !geometry.is_null())
                .into_iter()
                .collect::<Vec<_>>(),
            str_constants::GEO_JSON_FEATURE_COLLECTION => object
                .get(str_constants::GEO_JSON_FEATURES)
                .and_then(Self::as_array)
                .ok_or(GeoJsonValidationError::Document)?
                .iter()
                .collect::<Vec<_>>(),
            str_constants::GEO_JSON_GEOMETRY_COLLECTION => object
                .get(str_constants::GEO_JSON_GEOMETRIES)
                .and_then(Self::as_array)
                .ok_or(GeoJsonValidationError::Document)?
                .iter()
                .collect::<Vec<_>>(),
            str_constants::GEO_JSON_POINT
            | str_constants::GEO_JSON_LINE_STRING
            | str_constants::GEO_JSON_MULTI_POINT
            | str_constants::GEO_JSON_MULTI_LINE_STRING
            | str_constants::GEO_JSON_POLYGON
            | str_constants::GEO_JSON_MULTI_POLYGON => Vec::new(),
            _ => return Err(GeoJsonValidationError::UnsupportedGeometry),
        };
        children
            .into_iter()
            .try_for_each(Self::validate_supported_geo_json_types)
    }
}

#[cfg(test)]
mod tests {
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
