#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
struct DelegateInput;

async fn delegate(delegate_input: DelegateInput) -> Result<(), DelegateError> {
    drop(delegate_input);
    Ok(())
}

#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
#[derive(Debug)]
struct DelegateError;

#[proc_macro_frontend_contract::route_openapi(delegate = delegate, tag = "fixture")]
async fn invalid_delegate(delegate_input: DelegateInput) -> Result<(), DelegateError> {
    delegate(delegate_input).await
}

fn main() {
    let _arguments = std::env::args_os();
}
