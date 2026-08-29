#[derive(optimal_memory_layout::OptimalMemoryLayout)]
struct DelegateInput;

async fn delegate(value: DelegateInput) -> Result<(), DelegateError> {
    drop(value);
    Ok(())
}

#[derive(optimal_memory_layout::OptimalMemoryLayout)]
#[derive(Debug)]
struct DelegateError;

#[frontend_contract_macros::route_openapi(delegate = delegate, tag = "fixture")]
async fn invalid_delegate(value: DelegateInput) -> Result<(), DelegateError> {
    delegate(value).await
}

fn main() {}
