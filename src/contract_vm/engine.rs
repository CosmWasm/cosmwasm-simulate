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
use self::cosmwasm_std::{Binary, HumanAddr, Empty};
use crate::contract_vm::mock::{install, mock_env_addr, handler_resp};
use crate::contract_vm::querier::MyMockQuerier;
use crate::contract_vm::storage::MyMockStorage;
use crate::contract_vm::analyzer;
use self::cosmwasm_vm::testing::MockApi;


static COMPILE_GAS_LIMIT: u64 = 10_000_000_000;

pub struct ContractInstance {
    pub module: Module,
    pub instance: Instance<MyMockStorage, MockApi, MyMockQuerier>,
    pub wasm_file: String,
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

impl<'a> ContractInstance {
    pub fn new_instance(wasm_file: &str, contract_addr: &str) -> Result<Self, String> {
        let wasm = match analyzer::load_data_from_file(wasm_file) {
            Err(e) => return Err(e),
            Ok(code) => code,
        };
        println!("Compiling [{}]...", wasm_file);
        let md = wasmer_runtime_core::compile_with(wasm.as_slice(), compiler().as_ref()).unwrap();
        let inst = install(HumanAddr(contract_addr.clone().to_string()),
                                    contract_addr.clone().to_string(), wasm);

        // let inst = cosmwasm_vm::Instance::from_code(wasm.as_slice(), deps, DEFAULT_GAS_LIMIT).unwrap();
        return Ok(ContractInstance::make_instance(md, inst, wasm_file.to_string()));
    }

    fn make_instance(md: Module, inst: cosmwasm_vm::Instance<MyMockStorage, MockApi, MyMockQuerier>, file: String) -> ContractInstance {
        return ContractInstance {
            module: md,
            instance: inst,
            wasm_file: file,
            analyzer: analyzer::Analyzer::default(),
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
    pub fn call(&mut self, sender: String, contract_addr: String, func_type: String, param: String) -> String {
        println!("***************************[{}] call started***************************", func_type);
        println!("sender<{}>, contract addr<{}>, message<{}>", sender, contract_addr, param);

        let gas_init = self.instance.get_gas_left();
        if func_type == "init" {
            let env = mock_env_addr(&self.instance.api, &HumanAddr(sender), &HumanAddr(contract_addr), &[]);
            let init_result =
                cosmwasm_vm::call_init::<_, _, _, Empty>(&mut self.instance, &env, param.as_bytes());
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

            for log in msg.log {
                print!("{} => {}",log.key,log.value)
            }
        } else if func_type == "handle" {
            let env = mock_env_addr(&self.instance.api, &HumanAddr(sender), &HumanAddr(contract_addr.clone()), &[]);
            let handle_result = cosmwasm_vm::call_handle::<_, _, _, Empty>(&mut self.instance, &env, param.as_bytes()).expect("VM error");
            let res = match handle_result {
                Ok(data) => handler_resp(data, HumanAddr(contract_addr)),
                Err(err) => {
                    println!("Error {}", err);
                    return "ERROR      :execute query failed".to_string();
                }
            };

            let msg = match res {
                Ok(data) => data,
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
        } else if func_type == "query" {
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
        let gas_used = gas_init - self.instance.get_gas_left();
        println!("Gas used   : {}", gas_used);
        println!("***************************[{}] call finished***************************", func_type);
        return "Execute Success".to_string();
    }
}

