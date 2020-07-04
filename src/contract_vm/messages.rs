use std::fs::File;
use std::io::Read;

#[derive(Debug)]
pub struct Message {
    pub call_type: String,
    pub sender: String,
    pub message: String,
}

impl Message {
    pub fn new(call_type: String, sender: String, message: String) -> Message {
        return Message {
            call_type: call_type,
            sender: sender,
            message: message,
        };
    }

    pub fn build_msg(path: String) -> Vec<Message> {
        println!("load messages from: {}", path);
        let mut msgs = Vec::new();
        let data = match load_data_from_file(path.as_str()) {
            Err(_e) => return msgs,
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

                    for iter in mapping.iter() {
                        if iter.0 == "message" {
                            message = iter.1.to_string();
                        }
                        if iter.0 == "type" {
                            call_type = iter.1.to_string();
                        }
                        if iter.0 == "sender" {
                            sender = iter.1.to_string();
                        }
                    }

                    msgs.push(Message::new(call_type, sender, message));
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