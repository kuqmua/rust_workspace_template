#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    newtype::IntoInnerFrom,
)]
pub struct ListTotal(i64);

impl TryFrom<i64> for ListTotal {
    type Error = super::ListTotalError;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        if value < constants_i64::ZERO {
            Err(super::ListTotalError)
        } else {
            Ok(Self(value))
        }
    }
}

impl From<u32> for ListTotal {
    fn from(value: u32) -> Self {
        Self(i64::from(value))
    }
}
