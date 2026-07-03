#[derive(Debug, Clone, Copy, Default, Eq, PartialEq)]
pub enum EncodeFormat {
    #[default]
    Base64,
    Escape,
    Hex,
}

impl core::fmt::Display for EncodeFormat {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match *self {
            Self::Base64 => core::write!(f, "base64"),
            Self::Escape => core::write!(f, "escape"),
            Self::Hex => core::write!(f, "hex"),
        }
    }
}

impl pg_crud_cmn::DfltSomeOneEl for EncodeFormat {
    fn dflt_some_one_el() -> Self {
        Self::default()
    }
}

#[derive(Debug, Clone)]
pub struct RgxRgx(regex::Regex);

impl TryFrom<String> for RgxRgx {
    type Error = regex::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        regex::Regex::new(value.as_str()).map(Self)
    }
}

impl From<RgxRgx> for String {
    fn from(value: RgxRgx) -> Self {
        value.0.as_str().to_owned()
    }
}

impl PartialEq for RgxRgx {
    fn eq(&self, other: &Self) -> bool {
        self.0.as_str() == other.0.as_str()
    }
}

impl Eq for RgxRgx {}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq)]
pub enum RgxCase {
    Insensitive,
    #[default]
    Sensitive,
}

impl pg_crud_cmn::DfltSomeOneEl for RgxCase {
    fn dflt_some_one_el() -> Self {
        Self::default()
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PgJsonNotEmptyUnqVec<T>(pg_crud_cmn::NotEmptyUnqVec<T>);

impl<T> From<PgJsonNotEmptyUnqVec<T>> for Vec<T> {
    fn from(value: PgJsonNotEmptyUnqVec<T>) -> Self {
        value.0.into()
    }
}

impl<T: PartialEq> TryFrom<Vec<T>> for PgJsonNotEmptyUnqVec<T> {
    type Error = pg_crud_cmn::NotEmptyUnqVecTryNewEr<T>;

    fn try_from(value: Vec<T>) -> Result<Self, Self::Error> {
        pg_crud_cmn::NotEmptyUnqVec::try_from(value).map(Self)
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PgTypeNotEmptyUnqVec<T>(pg_crud_cmn::NotEmptyUnqVec<T>);

impl<T> From<PgTypeNotEmptyUnqVec<T>> for Vec<T> {
    fn from(value: PgTypeNotEmptyUnqVec<T>) -> Self {
        value.0.into()
    }
}

impl<T: PartialEq> TryFrom<Vec<T>> for PgTypeNotEmptyUnqVec<T> {
    type Error = pg_crud_cmn::NotEmptyUnqVecTryNewEr<T>;

    fn try_from(value: Vec<T>) -> Result<Self, Self::Error> {
        pg_crud_cmn::NotEmptyUnqVec::try_from(value).map(Self)
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct BoundedVec<T, const LENGTH: usize>(Vec<T>);

impl<T, const LENGTH: usize> TryFrom<Vec<T>> for BoundedVec<T, LENGTH> {
    type Error = BoundedVecTryNewEr;

    fn try_from(value: Vec<T>) -> Result<Self, Self::Error> {
        if value.len() != LENGTH {
            return Err(BoundedVecTryNewEr::LenIsNotCorrect);
        }
        Ok(Self(value))
    }
}

impl<T, const LENGTH: usize> From<BoundedVec<T, LENGTH>> for Vec<T> {
    fn from(value: BoundedVec<T, LENGTH>) -> Self {
        value.0
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum BoundedVecTryNewEr {
    LenIsNotCorrect,
}
