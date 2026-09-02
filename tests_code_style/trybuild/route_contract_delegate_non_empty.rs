#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
struct DelegateInput;

async fn delegate(value: DelegateInput) -> Result<(), DelegateError> {
    drop(value);
    Ok(())
}

#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
#[derive(Debug)]
struct DelegateError;

#[proc_macro_frontend_contract::route_openapi(delegate = delegate, tag = "fixture")]
async fn invalid_delegate(value: DelegateInput) -> Result<(), DelegateError> {
    delegate(value).await
}

fn main() {}
