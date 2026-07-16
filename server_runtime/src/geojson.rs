const GEO_JSON_MAXIMUM_BYTES: usize = 16_777_216usize;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GeoJsonDocumentText(String);
impl AsRef<str> for GeoJsonDocumentText {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}
impl TryFrom<String> for GeoJsonDocumentText {
    type Error = GeoJsonValidationError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > GEO_JSON_MAXIMUM_BYTES {
            return Err(GeoJsonValidationError::TooLarge);
        }
        let document = serde_json::from_str::<serde_json::Value>(value.as_str())
            .map_err(|error| GeoJsonValidationError::SerdeJson(SerdeJsonGeoJsonError(error)))?;
        GeoJsonValueValidation::validate_geo_json(&document)?;
        Ok(Self(value))
    }
}

#[derive(Debug)]
pub struct SerdeJsonGeoJsonError(serde_json::Error);
impl std::fmt::Display for SerdeJsonGeoJsonError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl std::error::Error for SerdeJsonGeoJsonError {}

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

trait GeoJsonValueValidation {
    fn validate_coordinate_level_one(&self) -> Result<(), GeoJsonValidationError>;
    fn validate_coordinate_level_three(&self) -> Result<(), GeoJsonValidationError>;
    fn validate_coordinate_level_two(&self) -> Result<(), GeoJsonValidationError>;
    fn validate_geo_json(&self) -> Result<(), GeoJsonValidationError>;
    fn validate_position(&self) -> Result<(), GeoJsonValidationError>;
}
impl GeoJsonValueValidation for serde_json::Value {
    fn validate_coordinate_level_one(&self) -> Result<(), GeoJsonValidationError> {
        let values = self
            .as_array()
            .filter(|values| !values.is_empty())
            .ok_or(GeoJsonValidationError::Coordinates)?;
        values.iter().try_for_each(Self::validate_position)
    }

    fn validate_coordinate_level_three(&self) -> Result<(), GeoJsonValidationError> {
        let values = self
            .as_array()
            .filter(|values| !values.is_empty())
            .ok_or(GeoJsonValidationError::Coordinates)?;
        values
            .iter()
            .try_for_each(Self::validate_coordinate_level_two)
    }

    fn validate_coordinate_level_two(&self) -> Result<(), GeoJsonValidationError> {
        let values = self
            .as_array()
            .filter(|values| !values.is_empty())
            .ok_or(GeoJsonValidationError::Coordinates)?;
        values
            .iter()
            .try_for_each(Self::validate_coordinate_level_one)
    }

    fn validate_geo_json(&self) -> Result<(), GeoJsonValidationError> {
        let object = self.as_object().ok_or(GeoJsonValidationError::Document)?;
        let value_type = object
            .get(str_constants::GEO_JSON_TYPE)
            .and_then(Self::as_str)
            .ok_or(GeoJsonValidationError::Document)?;
        match value_type {
            str_constants::GEO_JSON_FEATURE => object
                .get(str_constants::GEO_JSON_GEOMETRY)
                .filter(|geometry| !geometry.is_null())
                .map_or(Ok(()), Self::validate_geo_json),
            str_constants::GEO_JSON_FEATURE_COLLECTION => object
                .get(str_constants::GEO_JSON_FEATURES)
                .and_then(Self::as_array)
                .ok_or(GeoJsonValidationError::Document)?
                .iter()
                .try_for_each(Self::validate_geo_json),
            str_constants::GEO_JSON_GEOMETRY_COLLECTION => object
                .get(str_constants::GEO_JSON_GEOMETRIES)
                .and_then(Self::as_array)
                .ok_or(GeoJsonValidationError::Document)?
                .iter()
                .try_for_each(Self::validate_geo_json),
            str_constants::GEO_JSON_POINT => object
                .get(str_constants::GEO_JSON_COORDINATES)
                .ok_or(GeoJsonValidationError::Coordinates)?
                .validate_position(),
            str_constants::GEO_JSON_LINE_STRING | str_constants::GEO_JSON_MULTI_POINT => object
                .get(str_constants::GEO_JSON_COORDINATES)
                .ok_or(GeoJsonValidationError::Coordinates)?
                .validate_coordinate_level_one(),
            str_constants::GEO_JSON_MULTI_LINE_STRING | str_constants::GEO_JSON_POLYGON => object
                .get(str_constants::GEO_JSON_COORDINATES)
                .ok_or(GeoJsonValidationError::Coordinates)?
                .validate_coordinate_level_two(),
            str_constants::GEO_JSON_MULTI_POLYGON => object
                .get(str_constants::GEO_JSON_COORDINATES)
                .ok_or(GeoJsonValidationError::Coordinates)?
                .validate_coordinate_level_three(),
            _ => Err(GeoJsonValidationError::UnsupportedGeometry),
        }
    }

    fn validate_position(&self) -> Result<(), GeoJsonValidationError> {
        let values = self
            .as_array()
            .filter(|values| values.len() >= 2usize)
            .ok_or(GeoJsonValidationError::Coordinates)?;
        let longitude_valid = values
            .first()
            .and_then(Self::as_f64)
            .is_some_and(|coordinate| {
                coordinate.is_finite() && (-180.0f64..=180.0f64).contains(&coordinate)
            });
        let latitude_valid = values
            .get(1usize)
            .and_then(Self::as_f64)
            .is_some_and(|coordinate| {
                coordinate.is_finite() && (-90.0f64..=90.0f64).contains(&coordinate)
            });
        if longitude_valid && latitude_valid {
            Ok(())
        } else {
            Err(GeoJsonValidationError::Coordinates)
        }
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
