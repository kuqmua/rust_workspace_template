pub trait IdempotencyResponseResourceBudgetProvider {
    fn idempotency_response_resource_budget(&self) -> &super::ResourceBudget;
}
