extern crate clap;

pub mod contract_vm;
use clap::{Arg, App};
use std::collections::HashMap;

fn main() {
    let matches = App::new("cosmwasm-simulate")
        .version("0.1.0")
        .author("github : https://github.com/KamiD")
        .about("A simulation of cosmwasm smart contract system")
        .arg(Arg::with_name("message")
            .short("m")
            .long("message")
            .value_name("message.json")
            .help("Sets a custom config file")
            .takes_value(true))
        .get_matches();

    if let Some(message_file) = matches.value_of("message") {
        let msgs = contract_vm::messages::Message::build_msg(message_file.to_string());

        let mut contract_map = load_contract(&msgs);

        for m in msgs {
            if let Some(contract) = contract_map.get_mut(&m.contract_addr) {
                contract.call(
                    m.sender.clone(),
                    m.contract_addr.clone(),
                    m.call_type.clone(),
                    m.message.clone());
            }
        }
    }
}

fn load_contract(msgs: &Vec<contract_vm::messages::Message>) -> HashMap<String, contract_vm::engine::ContractInstance> {
    let mut contract_map = HashMap::new();
    for m in msgs {
        if m.call_type != "init" {
            continue
        }
        if m.wasm_file.is_empty() {
            continue
        }
        if m.contract_addr.is_empty() {
            continue
        }

        let contract =
            match contract_vm::engine::ContractInstance::new_instance(
                m.wasm_file.as_str(),
                m.contract_addr.as_str()) {
            Err(e) => {
                println!("failed to load [{}], error: {}", m.wasm_file.to_string(), e);
                return contract_map
            },
            Ok(instance) => {
                println!("successfully loaded [{}]", m.wasm_file.to_string());
                instance
            },
        };
        contract_map.insert(m.contract_addr.clone(), contract);
    }
    return contract_map;
}

