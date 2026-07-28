#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ResourceBudgetMaximum(usize);
#[derive(Clone, Copy, Debug, Eq, PartialEq, newtype::FromInner)]
pub struct ResourceBudgetAmount(usize);

#[derive(Clone, Debug, newtype::FromInner)]
struct StdSharedAtomicUsize(std::sync::Arc<std::sync::atomic::AtomicUsize>);

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
#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
#[error("{}", str_constants::RESOURCE_BUDGET_MAXIMUM_MUST_BE_GREATER_THAN_ZERO)]
pub struct ResourceBudgetConfigError;
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
#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
pub enum ResourceBudgetReserveError {
    #[error("{}", str_constants::RESOURCE_BUDGET_EXHAUSTED)]
    Exhausted,
    #[error("{}", str_constants::RESOURCE_BUDGET_RESERVATION_OVERFLOW)]
    Overflow,
}
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
            reserved: StdSharedAtomicUsize::from(std::sync::Arc::from(
                std::sync::atomic::AtomicUsize::new(0usize),
            )),
        }
    }
    pub fn reserve(
        &self,
        amount: ResourceBudgetAmount,
    ) -> Result<ResourceBudgetReservation, ResourceBudgetReserveError> {
        let result = self.reserved.0.try_update(
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
        ResourceBudgetAmount::from(self.reserved.0.load(std::sync::atomic::Ordering::Acquire))
    }
}
impl Drop for ResourceBudgetReservation {
    fn drop(&mut self) {
        let _previous = self
            .reserved
            .0
            .fetch_sub(self.amount.0, std::sync::atomic::Ordering::AcqRel);
    }
}
