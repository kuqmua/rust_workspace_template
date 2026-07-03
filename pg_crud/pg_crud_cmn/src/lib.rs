pub const DEFAULT_PAGINATION_LIMIT: i64 = 5;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq)]
pub enum Oprtr {
    And,
    AndNot,
    #[default]
    Or,
    OrNot,
}

pub trait AllEnumVrtsArrDfltSomeOneEl: Sized {
    fn all_vrts_dflt_some_one_el() -> Self;
}

pub trait AllEnumVrtsArrDfltSomeOneElMaxPageSize: Sized {
    fn all_vrts_dflt_some_one_el_max_page_size() -> Self;
}

pub trait DfltSomeOneEl: Sized {
    fn dflt_some_one_el() -> Self;
}

pub trait DfltSomeOneElMaxPageSize: Sized {
    fn dflt_some_one_el_max_page_size() -> Self;
}

impl DfltSomeOneEl for Oprtr {
    fn dflt_some_one_el() -> Self {
        Self::default()
    }
}

impl core::fmt::Display for Oprtr {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match *self {
            Self::And => core::write!(f, "And"),
            Self::AndNot => core::write!(f, "AndNot"),
            Self::Or => core::write!(f, "Or"),
            Self::OrNot => core::write!(f, "OrNot"),
        }
    }
}

impl Oprtr {
    #[must_use]
    pub const fn is_negative(self) -> OprtrNegativeState {
        match self {
            Self::And | Self::Or => OprtrNegativeState::False,
            Self::AndNot | Self::OrNot => OprtrNegativeState::True,
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum OprtrNegativeState {
    False,
    True,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum PgJsonLenGreaterThanVrt {
    EqNotLenGreaterThan,
    LenGreaterThan,
    NotLenGreaterThan,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum PgTypeGreaterThanVrt {
    EqNotGreaterThan,
    GreaterThan,
    NotGreaterThan,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Order {
    Asc,
    Desc,
}

impl DfltSomeOneEl for Order {
    fn dflt_some_one_el() -> Self {
        Self::Asc
    }
}

impl core::fmt::Display for Order {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match *self {
            Self::Asc => core::write!(f, "asc"),
            Self::Desc => core::write!(f, "desc"),
        }
    }
}

pub trait PgType {}

pub trait PgJson {}

pub trait PgTypePk {}

pub trait PgTypeNotPk {}

pub trait PgJsonObjVecElId {}

pub trait PgTypeTestCases {}

pub trait PgJsonTestCases {}

pub trait PgTypeWhFlt<'query> {}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum QpEr {
    CheckedAdd,
    WriteIntoBuffer,
}

impl core::fmt::Display for QpEr {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match *self {
            Self::CheckedAdd => core::write!(f, "CheckedAdd"),
            Self::WriteIntoBuffer => core::write!(f, "WriteIntoBuffer"),
        }
    }
}

impl core::error::Error for QpEr {}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct QueryParameterIndex;

impl From<QueryParameterIndex> for u64 {
    fn from(_value: QueryParameterIndex) -> Self {
        Self::from(true)
    }
}

impl TryFrom<u64> for QueryParameterIndex {
    type Error = QpEr;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if value == u64::MIN {
            return Err(QpEr::CheckedAdd);
        }
        Ok(Self)
    }
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq)]
pub struct QueryParameterCounter;

impl QueryParameterCounter {
    #[must_use]
    pub const fn increment(&mut self) -> QueryParameterIncrementResult {
        QueryParameterIncrementResult::Incremented(QueryParameterIndex)
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum QueryParameterIncrementResult {
    CheckedAdd,
    Incremented(QueryParameterIndex),
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct PgnLimit;

impl From<PgnLimit> for i64 {
    fn from(_value: PgnLimit) -> Self {
        DEFAULT_PAGINATION_LIMIT
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct PgnOffset;

impl From<PgnOffset> for i64 {
    fn from(_value: PgnOffset) -> Self {
        Self::from(false)
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct PgnBase {
    limit: PgnLimit,
    offset: PgnOffset,
}

impl PgnBase {
    #[must_use]
    pub const fn new_unchecked(limit: PgnLimit, offset: PgnOffset) -> Self {
        Self { limit, offset }
    }
}

impl Default for PgnBase {
    fn default() -> Self {
        Self::new_unchecked(PgnLimit, PgnOffset)
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct NotEmptyUnqVec<T>(Vec<T>);

impl<T> From<NotEmptyUnqVec<T>> for Vec<T> {
    fn from(value: NotEmptyUnqVec<T>) -> Self {
        value.0
    }
}

impl<T: PartialEq> TryFrom<Vec<T>> for NotEmptyUnqVec<T> {
    type Error = NotEmptyUnqVecTryNewEr<T>;

    fn try_from(mut value: Vec<T>) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(NotEmptyUnqVecTryNewEr::Empty);
        }
        match first_duplicate_index(value.as_slice()) {
            DuplicateIndexSearchResult::Found(duplicate_index) => {
                Err(NotEmptyUnqVecTryNewEr::NotUnq(value.swap_remove(duplicate_index.0.get())))
            }
            DuplicateIndexSearchResult::NotFound => Ok(Self(value)),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum NotEmptyUnqVecTryNewEr<T> {
    Empty,
    NotUnq(T),
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct JsonFieldRights;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct NonPkPgTypeRdIds;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum EqOprtr {
    Eq,
    NotEq,
}

pub trait PgTypeEqOprtr {}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct UnsignedPartOfI32;

impl TryFrom<i32> for UnsignedPartOfI32 {
    type Error = UnsignedPartOfI32TryFromI32Er;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value < i32::from(false) {
            return Err(UnsignedPartOfI32TryFromI32Er::Negative);
        }
        Ok(Self)
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum UnsignedPartOfI32TryFromI32Er {
    Negative,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct NotZeroUnsignedPartOfI32(UnsignedPartOfI32);

impl TryFrom<i32> for NotZeroUnsignedPartOfI32 {
    type Error = NotZeroUnsignedPartOfI32TryFromI32Er;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value == i32::from(false) {
            return Err(NotZeroUnsignedPartOfI32TryFromI32Er::Zero);
        }
        UnsignedPartOfI32::try_from(value)
            .map(Self)
            .map_err(|source_error| match source_error {
                UnsignedPartOfI32TryFromI32Er::Negative => {
                    NotZeroUnsignedPartOfI32TryFromI32Er::Negative
                }
            })
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum NotZeroUnsignedPartOfI32TryFromI32Er {
    Negative,
    Zero,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum SingleOrMultiple<T> {
    Multiple(NotEmptyUnqVec<T>),
    Single(T),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct DuplicateSearchValues<T>(Vec<T>);

impl<T> From<Vec<T>> for DuplicateSearchValues<T> {
    fn from(value: Vec<T>) -> Self {
        Self(value)
    }
}

impl<T> From<DuplicateFreeValues<T>> for Vec<T> {
    fn from(value: DuplicateFreeValues<T>) -> Self {
        value.0
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct DuplicateFreeValues<T>(Vec<T>);

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct DuplicateValue<T>(T);

impl<T> From<DuplicateValue<T>> for Option<T> {
    fn from(value: DuplicateValue<T>) -> Self {
        Some(value.0)
    }
}

impl<T> DuplicateValue<T> {
    #[must_use]
    pub fn into_inner(self) -> T {
        self.0
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct DuplicateIndex(core::num::NonZeroUsize);

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum DuplicateIndexSearchResult {
    Found(DuplicateIndex),
    NotFound,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum DuplicateSearchResult<T> {
    Duplicate(DuplicateValue<T>),
    Unique(DuplicateFreeValues<T>),
}

fn first_duplicate_index<T>(values: &[T]) -> DuplicateIndexSearchResult
where
    T: PartialEq,
{
    let found_duplicate_index = values
        .iter()
        .enumerate()
        .find_map(|(index, current_value)| {
            values
                .iter()
                .take(index)
                .any(|previous_value| previous_value == current_value)
                .then(|| {
                    core::num::NonZeroUsize::new(index)
                        .map(DuplicateIndex)
                        .map(DuplicateIndexSearchResult::Found)
                })
                .flatten()
        });
    found_duplicate_index
        .map_or(DuplicateIndexSearchResult::NotFound, |duplicate_index| duplicate_index)
}

#[must_use]
pub fn take_fst_dup<T>(values: DuplicateSearchValues<T>) -> DuplicateSearchResult<T>
where
    T: PartialEq,
{
    let mut inner_values = values.0;
    match first_duplicate_index(inner_values.as_slice()) {
        DuplicateIndexSearchResult::Found(duplicate_index) => DuplicateSearchResult::Duplicate(
            DuplicateValue(inner_values.swap_remove(duplicate_index.0.get())),
        ),
        DuplicateIndexSearchResult::NotFound => {
            DuplicateSearchResult::Unique(DuplicateFreeValues(inner_values))
        }
    }
}
