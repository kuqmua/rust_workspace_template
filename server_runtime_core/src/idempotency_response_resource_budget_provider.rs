pub trait IdempotencyResponseResourceBudgetProvider {
    fn idempotency_response_resource_budget(&self) -> &crate::resource_budget::ResourceBudget;
}
