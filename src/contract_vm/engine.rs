extern crate cosmwasm_vm;
extern crate cosmwasm_std;
extern crate serde_json;

use std::fmt::Write;
use wasmer_runtime_core::{
    backend::Compiler,
    codegen::{MiddlewareChain, StreamingCompiler},
    module::Module,
};
use wasmer_middleware_common::metering;

use wasmer_singlepass_backend::ModuleCodeGenerator as SinglePassMCG;
use self::cosmwasm_vm::{Instance};
use self::cosmwasm_vm::testing::{MockQuerier};
use self::cosmwasm_std::{Uint128, Binary};
use crate::contract_vm::{mock, analyzer};

static DEFAULT_GAS_LIMIT: u64 = 500_000;
static COMPILE_GAS_LIMIT: u64 = 10_000_000_000;


pub struct ContractInstance {
    pub module: Module,
    pub instance: Instance<mock::MockStorage, mock::MockApi, MockQuerier>,
    pub wasm_file: String,
    pub env: cosmwasm_std::Env,
    pub analyzer: analyzer::Analyzer,
}

fn compiler() -> Box<dyn Compiler> {
    let c: StreamingCompiler<SinglePassMCG, _, _, _, _> = StreamingCompiler::new(move || {
        let mut chain = MiddlewareChain::new();
        //compile without opCode check
        //chain.push(DeterministicMiddleware::new());
        chain.push(metering::Metering::new(COMPILE_GAS_LIMIT));
        chain
    });
    Box::new(c)
}

impl ContractInstance
{
    pub fn new_instance(wasm_file: &str) -> Result<Self, String> {
        let deps = mock::new_mock(20, &[], "fake_contract_addr");
        let wasm = match analyzer::load_data_from_file(wasm_file) {
            Err(e) => return Err(e),
            Ok(code) => code,
        };
        println!("Compiling code");
        let md = wasmer_runtime_core::compile_with(wasm.as_slice(), compiler().as_ref()).unwrap();
        let inst = cosmwasm_vm::Instance::from_code(wasm.as_slice(), deps, DEFAULT_GAS_LIMIT).unwrap();
        return Ok(ContractInstance::make_instance(md, inst, wasm_file.to_string()));
    }

    fn make_instance(md: Module, inst: cosmwasm_vm::Instance<mock::MockStorage, mock::MockApi, MockQuerier>, file: String) -> ContractInstance {
        return ContractInstance {
            module: md,
            instance: inst,
            wasm_file: file,
            env: ContractInstance::build_mock_env(),
            analyzer: analyzer::Analyzer::default(),
        };
    }

    fn build_mock_env() -> cosmwasm_std::Env {
        return cosmwasm_std::Env {
            block: cosmwasm_std::BlockInfo {
                height: 0,
                time: 0,
                chain_id: "okchain".to_string(),
            },
            message: cosmwasm_std::MessageInfo {
                sender: cosmwasm_std::CanonicalAddr {
                    0: cosmwasm_std::Binary::from_base64("b2tjaGFpbl9rYW1pZA==").unwrap()
                },
                sent_funds: vec![cosmwasm_std::Coin {
                    denom: "okt".to_string(),
                    amount: Uint128(100000000),
                }],
            },
            contract: Default::default(),
        };
    }

    pub fn show_module_info(&self) {
        println!("showing wasm module info for [{}]", self.wasm_file);
        println!("backend : [{}]", self.module.info().backend);

        println!("=============================== module info exported func name ===============================");
        for exdesc in self.module.exports() {
            println!("exported func name [{}]", exdesc.name);
        }
        println!("=============================== module info exported func name ===============================");
        for desc in self.module.imports() {
            println!("import descriptor name:[{}->{}]", desc.namespace, desc.name);
        }
    }

    fn dump_result(key: &str, value: &[u8]) {
        let mut value_str = match std::str::from_utf8(value) {
            Ok(result) => result.to_string(),
            _ => "".to_string()
        };

        if value_str.is_empty() {
            for a in value.iter() {
                write!(value_str, "{:02x}", a).expect("Not written");
            }
        }

        println!("{} = {}", key, value_str);
    }
    pub fn call(&mut self, _sender: String, func_type: String, param: String) -> String {
        println!("***************************call started***************************");
        println!("executing func [{}] , params is {}", func_type, param);
        let gas_init = self.instance.get_gas();
        if func_type == "\"init\"" || func_type == "init" {
            let init_result =
                cosmwasm_vm::call_init::<_, _, _, cosmwasm_std::Never>(&mut self.instance, &self.env, param.as_bytes());
            let msg = match init_result {
                Ok(data) => match data {
                    Ok(resp) => resp,
                    Err(err) => {
                        println!("Error {}", err);
                        return "ERROR      :execute init failed".to_string();
                    }
                },
                Err(err) => {
                    println!("Error {}", err);
                    return "ERROR      :execute init failed".to_string();
                }
            };
            let data: Binary = match msg.data {
                None => Binary::from("".as_bytes()),
                Some(d) => d
            };
            ContractInstance::dump_result("init msg.data:", data.0.as_slice());
        } else if func_type == "handle" || func_type == "\"handle\"" {
            let handle_result = cosmwasm_vm::call_handle::<_, _, _, cosmwasm_std::Never>(&mut self.instance, &self.env, param.as_bytes());
            let msg = match handle_result {
                Ok(data) => match data {
                    Ok(resp) => resp,
                    Err(err) => {
                        println!("Error {}", err);
                        return "ERROR      :execute query failed".to_string();
                    }
                },
                Err(err) => {
                    println!("Error {}", err);
                    return "ERROR      :execute query failed".to_string();
                }
            };

            let data: Binary = match msg.data {
                None => Binary::from("".as_bytes()),
                Some(d) => d
            };
            ContractInstance::dump_result("handle msg.data:", data.0.as_slice());
        } else if func_type == "query" || func_type == "\"query\"" {
            let query_result = cosmwasm_vm::call_query::<_, _, _>(&mut self.instance, param.as_bytes());
            let msg = match query_result {
                Ok(data) => match data {
                    Ok(resp) => resp,
                    Err(err) => {
                        println!("Error {}", err);
                        return "ERROR      :execute query failed".to_string();
                    }
                },
                Err(err) => {
                    println!("Error {}", err);
                    return "ERROR      :execute query failed".to_string();
                }
            };

            ContractInstance::dump_result("query msg.data:", msg.0.as_slice());
        } else {
            println!("wrong dispatcher call {}", func_type);
        }
        let gas_used = gas_init - self.instance.get_gas();
        println!("Gas used   : {}", gas_used);
        println!("***************************call finished***************************");
        return "Execute Success".to_string();
    }
}

