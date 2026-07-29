struct DelegateInput;

async fn delegate(value: DelegateInput) -> Result<(), DelegateError> {
    drop(value);
    Ok(())
}

#[derive(Debug)]
struct DelegateError;

#[frontend_contract::route_openapi(delegate = delegate, tag = "fixture")]
async fn invalid_delegate(value: DelegateInput) -> Result<(), DelegateError> {
    delegate(value).await
}

fn main() {}
