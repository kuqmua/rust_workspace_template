pub trait BulkItemResourceBudgetProvider {
    fn bulk_item_resource_budget(&self) -> &crate::resource_budget::ResourceBudget;
}
