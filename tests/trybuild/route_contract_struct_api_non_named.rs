#[derive(optimal_memory_layout::OptimalMemoryLayout)]
#[derive(frontend_contract::domain_types::ContractStructApi, newtype::FromInner)]
#[contract_struct_api(new)]
struct InvalidContract(u8);

fn main() {}
