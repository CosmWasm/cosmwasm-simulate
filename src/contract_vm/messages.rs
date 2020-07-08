use std::fs::File;
use std::io::Read;

#[derive(Debug)]
pub struct Message {
    pub contract_addr: String,
    pub wasm_file: String,
    pub call_type: String,
    pub sender: String,
    pub message: String,
}

impl Message {
    pub fn default() -> Self {
        return Message {
            contract_addr: String::new(),
            wasm_file: String::new(),
            call_type: String::new(),
            sender: String::new(),
            message: String::new(),
        };
    }

    pub fn new(call_type: String, sender: String, message: String, wasm_file: String, contract_addr: String) -> Message {
        return Message {
            wasm_file: wasm_file,
            contract_addr: contract_addr,
            call_type: call_type,
            sender: sender,
            message: message,
        };
    }

    pub fn build_msg(path: String) -> Vec<Message> {
        println!("load messages from: {}", path);
        let mut msgs = Vec::new();
        let data = match load_data_from_file(path.as_str()) {
            Err(e) => {
                println!("can not load target message file: {} => {}",path.as_str(),e.as_str());
                return msgs
            },
            Ok(code) => code,
        };

        let translated: serde_json::Value = match serde_json::from_slice(data.as_slice()) {
            Ok(trs) => trs,
            Err(_e) => return msgs,
        };

        let mapping = match translated.as_object() {
            None => return msgs,
            Some(kvs) => kvs,
        };

        for iter in mapping.iter() {
            if iter.0 == "messages" {
                let array: &Vec<serde_json::Value> = match iter.1.as_array() {
                    None => continue,
                    Some(a) => a,
                };
                for item in array {

                    let mapping = match item.as_object() {
                        None => return msgs,
                        Some(kvs) => kvs,
                    };
                    let mut call_type= String::new();
                    let mut sender= String::new();
                    let mut message= String::new();
                    let mut wasm_file= String::new();
                    let mut contract_addr= String::new();

                    for iter in mapping.iter() {
                        if iter.0 == "message" {
                            message = iter.1.to_string();
                        }
                        if iter.0 == "type" {
                            call_type = iter.1.to_string();
                            call_type.retain(|c| c != '"');
                        }
                        if iter.0 == "sender" {
                            sender = iter.1.to_string();
                            sender.retain(|c| c != '"');
                        }
                        if iter.0 == "wasm_file" {
                            wasm_file = iter.1.to_string();
                            wasm_file.retain(|c| c != '"');
                        }
                        if iter.0 == "contract_addr" {
                            contract_addr = iter.1.to_string();
                            contract_addr.retain(|c| c != '"');
                        }
                    }

                    if call_type == "init" {
                        if wasm_file.is_empty() {
                            panic!("A wasm file is expected for {}", message)
                        }
                        if contract_addr.is_empty() {
                            panic!("A contract address is expected for {}", message)
                        }

                    } else if call_type == "handle" {
                        if sender.is_empty() {
                            panic!("A sender address is expected for {}", message)
                        }
                        if contract_addr.is_empty() {
                            panic!("A contract address is expected for {}", message)
                        }
                    } else if call_type == "query" {
                        if contract_addr.is_empty() {
                            panic!("A contract address is expected for {}", message)
                        }
                    } else {
                        panic!("Incorrect type: {}!. Only [query | init | handle] is expected.", call_type)
                    }

                    msgs.push(Message::new(call_type, sender, message, wasm_file, contract_addr));
                }
            }

        }
        return msgs;
    }
}

pub fn load_data_from_file(path: &str) -> Result<Vec<u8>, String> {
    let mut file = match File::open(path) {
        Err(e) => return Err(format!("failed to open file , error: {}", e).to_string()),
        Ok(f) => f,
    };
    let mut data = Vec::<u8>::new();
    let _size = match file.read_to_end(&mut data) {
        Err(e) => return Err(format!("failed to read wasm , error: {}", e).to_string()),
        Ok(sz) => sz,
    };
    Ok(data)
}