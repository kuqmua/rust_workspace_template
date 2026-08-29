#[derive(optimal_memory_layout::OptimalMemoryLayout)]
#[derive(frontend_contract_macros::ContractStructApi, newtype::FromInner)]
#[contract_struct_api(new)]
struct InvalidContract(u8);

fn main() {}
