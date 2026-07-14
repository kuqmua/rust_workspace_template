#[derive(Debug, Clone, PartialEq, Eq, optml::Optml, newtype::Newtype)]
#[newtype(display)]
pub struct SqlxPostgresQueryBindEr(String);
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    thiserror::Error,
    utoipa::ToSchema,
)]
pub enum PgCrudStringWrapperTryFromStringEr {
    #[error("string wrapper length {len} exceeds maximum {max}")]
    TooLong { len: usize, max: usize },
}
impl to_err_string::ToErrString for PgCrudStringWrapperTryFromStringEr {
    fn to_err_string(&self) -> to_err_string::ToErrStringValue {
        to_err_string::ToErrStringValue::try_from(self.to_string())
            .unwrap_or_else(to_err_string::ToErrStringValue::from)
    }
}
impl From<PgCrudStringWrapperTryFromStringEr> for SqlxPostgresQueryBindEr {
    fn from(value: PgCrudStringWrapperTryFromStringEr) -> Self {
        Self(value.to_string())
    }
}
impl SqlxPostgresQueryBindEr {
    #[must_use]
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for SqlxPostgresQueryBindEr {
    type Error = PgCrudStringWrapperTryFromStringEr;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > crate::PG_CRUD_STRING_WRAPPER_MAX_LEN {
            return Err(Self::Error::TooLong {
                len: value.len(),
                max: crate::PG_CRUD_STRING_WRAPPER_MAX_LEN,
            });
        }
        Ok(Self(value))
    }
}
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    thiserror::Error,
    location::Location,
    optml::Optml,
)]
#[location_to_schema]
pub enum QpEr {
    CheckedAdd {
        loc: loc_lib::loc::Loc,
    },
    StringWrapperTryFromString {
        loc: loc_lib::loc::Loc,
        #[eo_to_err_string_serde]
        er: PgCrudStringWrapperTryFromStringEr,
    },
    WriteIntoBuffer {
        loc: loc_lib::loc::Loc,
    },
}
impl From<PgCrudStringWrapperTryFromStringEr> for QpEr {
    fn from(er: PgCrudStringWrapperTryFromStringEr) -> Self {
        Self::StringWrapperTryFromString {
            loc: loc_macros::loc!(),
            er,
        }
    }
}
