#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ResourceBudgetMaximum(usize);
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ResourceBudgetAmount(usize);
impl From<usize> for ResourceBudgetAmount {
    fn from(value: usize) -> Self {
        Self(value)
    }
}
#[derive(Debug)]
struct StdAtomicUsize(std::sync::atomic::AtomicUsize);
#[derive(Clone, Debug)]
struct StdSharedAtomicUsize(std::sync::Arc<StdAtomicUsize>);
impl TryFrom<usize> for ResourceBudgetMaximum {
    type Error = ResourceBudgetConfigError;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        if value == 0usize {
            Err(ResourceBudgetConfigError)
        } else {
            Ok(Self(value))
        }
    }
}
impl From<std::num::NonZeroUsize> for ResourceBudgetMaximum {
    fn from(value: std::num::NonZeroUsize) -> Self {
        Self(value.get())
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ResourceBudgetConfigError;
impl std::fmt::Display for ResourceBudgetConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(str_constants::text::RESOURCE_BUDGET_MAXIMUM_MUST_BE_GREATER_THAN_ZERO)
    }
}
impl std::error::Error for ResourceBudgetConfigError {}
#[derive(Clone, Debug)]
pub struct ResourceBudget {
    maximum: ResourceBudgetMaximum,
    reserved: StdSharedAtomicUsize,
}
pub trait GetBulkItemResourceBudget {
    fn get_bulk_item_resource_budget(&self) -> &ResourceBudget;
}
pub trait GetIdempotencyResponseResourceBudget {
    fn get_idempotency_response_resource_budget(&self) -> &ResourceBudget;
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ResourceBudgetReserveError {
    Exhausted,
    Overflow,
}
impl std::fmt::Display for ResourceBudgetReserveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Exhausted => f.write_str(str_constants::text::RESOURCE_BUDGET_EXHAUSTED),
            Self::Overflow => {
                f.write_str(str_constants::text::RESOURCE_BUDGET_RESERVATION_OVERFLOW)
            }
        }
    }
}
impl std::error::Error for ResourceBudgetReserveError {}
#[derive(Debug)]
#[must_use]
pub struct ResourceBudgetReservation {
    amount: ResourceBudgetAmount,
    reserved: StdSharedAtomicUsize,
}
impl ResourceBudget {
    #[must_use]
    pub fn new(maximum: ResourceBudgetMaximum) -> Self {
        Self {
            maximum,
            reserved: StdSharedAtomicUsize(std::sync::Arc::from(StdAtomicUsize(
                std::sync::atomic::AtomicUsize::new(0usize),
            ))),
        }
    }
    pub fn reserve(
        &self,
        amount: ResourceBudgetAmount,
    ) -> Result<ResourceBudgetReservation, ResourceBudgetReserveError> {
        let result = self.reserved.0.0.try_update(
            std::sync::atomic::Ordering::AcqRel,
            std::sync::atomic::Ordering::Acquire,
            |current| {
                current
                    .checked_add(amount.0)
                    .filter(|next| *next <= self.maximum.0)
            },
        );
        match result {
            Ok(_previous) => Ok(ResourceBudgetReservation {
                amount,
                reserved: self.reserved.clone(),
            }),
            Err(current) if current.checked_add(amount.0).is_none() => {
                Err(ResourceBudgetReserveError::Overflow)
            }
            Err(_current) => Err(ResourceBudgetReserveError::Exhausted),
        }
    }
    #[must_use]
    pub fn reserved(&self) -> ResourceBudgetAmount {
        ResourceBudgetAmount::from(self.reserved.0.0.load(std::sync::atomic::Ordering::Acquire))
    }
}
impl Drop for ResourceBudgetReservation {
    fn drop(&mut self) {
        let _previous = self
            .reserved
            .0
            .0
            .fetch_sub(self.amount.0, std::sync::atomic::Ordering::AcqRel);
    }
}
