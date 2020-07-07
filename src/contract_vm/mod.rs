pub mod engine;
pub mod analyzer;
pub mod watcher;
pub mod messages;
pub mod mock;
pub mod storage;
pub mod querier;

// pub fn build_simulation(wasmfile: &str) -> Result<ContractInstance, String> {
//     let wasmer = engine::ContractInstance::new_instance(wasmfile);
//     return wasmer;
// }
