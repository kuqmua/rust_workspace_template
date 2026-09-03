#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
#[derive(proc_macro_frontend_contract_derive_contract_struct_api::ContractStructApi, proc_macro_newtype_from_inner::FromInner)]
#[contract_struct_api(new)]
struct InvalidContract(u8);

fn main() {
    let _arguments = std::env::args_os();
}
