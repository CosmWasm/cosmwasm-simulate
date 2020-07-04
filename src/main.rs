extern crate clap;

pub mod contract_vm;
use clap::{Arg, App};

fn main() {
    prepare_command_line();
}

fn prepare_command_line() -> bool {
    let matches = App::new("cosmwasm-simulate")
        .version("0.1.0")
        .author("github : https://github.com/KamiD")
        .about("A simulation of cosmwasm smart contract system")
        .arg(Arg::with_name("wasm")
            .short("w")
            .long("wasm")
            .value_name("contract.wasm")
            .help("Sets a custom config file")
            .takes_value(true))
        .arg(Arg::with_name("message")
            .short("m")
            .long("message")
            .value_name("message.json")
            .help("Sets a custom config file")
            .takes_value(true))
        .arg(Arg::with_name("schema")
            .short("s")
            .long("schema")
            .value_name("message.json")
            .help("Sets a custom config file")
            .takes_value(true))
        .get_matches();

    if let Some(ref in_file) = matches.value_of("schema") {
        println!("The schema dir: {}", in_file);
    }

    let mut msgs = Vec::new();
    if let Some(message_file) = matches.value_of("message") {
        msgs = contract_vm::messages::Message::build_msg(message_file.to_string());
    }

    if let Some(file) = matches.value_of("wasm") {
        if !file.ends_with(".wasm") {
            println!("only support file[*.wasm], you just input a wrong file format - {:?}", file);
            return false;
        }
        match run(file, &msgs) {
            Ok(t) => {
                if t {
                    println!("start_simulate success");
                } else {
                    println!("start_simulate failed")
                }
            }
            Err(e) => println!("error occurred during call start_simulate : {}", e)
        }
        return true;
    }
    return false;
}


fn run(wasmfile: &str, msgs: &Vec<contract_vm::messages::Message>) -> Result<bool, String> {
    println!("load contract from: {}", wasmfile);
    let mut contract = match contract_vm::build_simulation(wasmfile) {
        Err(e) => return Err(e),
        Ok(instance) => instance,
    };

    for m in msgs {

        // todo:
        contract.call(m.sender.clone(), m.call_type.clone(), m.message.clone());
    }
    return Ok(true);
}


//
// fn show_message_type(name: &str, members: &Vec<contract_vm::analyzer::Member>, engine: &contract_vm::engine::ContractInstance) {
//     println!("{} {{", name);
//     for vcm in members {
//         let st = match engine.analyzer.map_of_struct.get_key_value(vcm.member_def.as_str()) {
//             Some(h) => h,
//             _ => {
//                 println!("\t{} : {}", vcm.member_name, vcm.member_def);
//                 continue;
//             }
//         };
//         //todo:need show all members by recursive invocation
//         println!("\t{} : {} :{{ ", vcm.member_name, vcm.member_def);
//         for members in st.1 {
//             println!("\t\t{} : {}", members.0, members.1);
//         }
//         println!("\t}}")
//     }
//     println!("}}");
// }
//
// fn check_is_need_flag(name: &str) -> bool {
//     if name == "integer" {
//         return false;
//     }
//     return true;
// }
//
// fn to_json_item(name: &String, data: &String, type_name: &str) -> String {
//     let mut params = "\"".to_string();
//     params += name.as_str();
//     params += "\":";
//     if check_is_need_flag(type_name) {
//         params += "\"";
//     }
//     params += data.as_str();
//     if check_is_need_flag(type_name) {
//         params += "\"";
//     }
//     params += ",";
//     return params;
// }
//
// fn input_with_out_handle(input_data: &mut String) -> bool {
//     match io::stdin().read_line(input_data) {
//         Ok(_n) => {
//             if input_data.ends_with("\n") {
//                 input_data.remove(input_data.len() - 1);
//             }
//         }
//         Err(error) => {
//             println!("error: {}", error);
//             return false;
//         }
//     }
//     return true;
// }
//
// fn input_type(mem_name: &String, type_name: &String, engine: &contract_vm::engine::ContractInstance) -> String {
//     println!("input [{}]:", mem_name);
//     let st = match engine.analyzer.map_of_struct.get_key_value(type_name) {
//         Some(h) => h,
//         _ => {
//             let mut single: String = String::new();
//             input_with_out_handle(&mut single);
//             return to_json_item(&mem_name, &single, type_name);
//         }
//     };
//     //todo:need show all members by recursive invocation
//     let mut params = "\"".to_string();
//     params += mem_name;
//     params += "\":[{";
//     for members in st.1 {
//         println!("input \t[{} : {}]:", members.0, members.1);
//         let mut single: String = String::new();
//         input_with_out_handle(&mut single);
//         params += to_json_item(&members.0, &single, type_name).as_str();
//     }
//     let (resv, _) = params.split_at(params.len() - 1);
//     let mut ret = resv.to_string();
//     ret += "}],";
//
//     return ret;
// }
//
// fn input_message(name: &str, members: &Vec<contract_vm::analyzer::Member>, engine: &contract_vm::engine::ContractInstance, is_enum: &bool) -> String {
//     let mut final_msg: String = "{".to_string();
//     if *is_enum {
//         final_msg = final_msg.add("\"");
//         final_msg = final_msg.add(name);
//         final_msg = final_msg.add("\":{");
//     }
//
//     for vcm in members {
//         final_msg = final_msg.add(input_type(&vcm.member_name, &vcm.member_def.to_string(), engine).as_str());
//     }
//     if members.len() > 0 {
//         let (resv, _) = final_msg.split_at(final_msg.len() - 1);
//         final_msg = resv.to_string();
//         final_msg = final_msg.add("}");
//     } else {
//         final_msg = "}".to_string();
//     }
//     if *is_enum {
//         final_msg = final_msg.add("}");
//     }
//
//     println!("JsonMsg:{}", final_msg);
//     return final_msg;
// }
//
// fn simulate2(engine: &mut ContractInstance) {
//     let json_msg: String =
//         "{\"name\":\"Test okchain token\",\"symbol\":\"OKT\",\"decimals\":10,\"initial_balances\":[{\"address\":\"okchainaaaaaaaokch\",\"amount\":\"1006\"}]}".to_string();
//     let call_type: String = "init".to_string();
//     let result = engine.call(call_type, json_msg);
//     println!("Call return msg [{}]", result);
//
//
//     // let json_msg: String =  "{\"transfer\":{\"recipient\":\"okchainya\",\"amount\":\"2\"}}".to_string();
//     // let call_type: String = "handle".to_string();
//     // let result = engine.call(call_type, json_msg);
//     // println!("Call return msg [{}]", result);
//
//
//     let json_msg: String =  "{\"balance\":{\"address\":\"okchainaaaaaaaokch\"}}".to_string();
//     let call_type: String = "query".to_string();
//     let result = engine.call(call_type, json_msg);
//     println!("Call return msg [{}]", result);
// }
//
// fn simulate3(engine: &mut ContractInstance) {
//     let json_msg: String =
//         "{\"name\":\"Test okchain token\",\"symbol\":\"OKT\",\"decimals\":10,\"initial_balances\":[{\"address\":\"okchainaaaaaaaokch\",\"amount\":\"1006\"}]}".to_string();
//     let call_type: String = "init".to_string();
//     let result = engine.call(call_type, json_msg);
//     println!("Call return msg [{}]", result);
//
//
//     // let json_msg: String =  "{\"transfer\":{\"recipient\":\"okchainya\",\"amount\":\"2\"}}".to_string();
//     // let call_type: String = "handle".to_string();
//     // let result = engine.call(call_type, json_msg);
//     // println!("Call return msg [{}]", result);
//
//
//     let json_msg: String =  "{\"balance\":{\"address\":\"okchainaaaaaaaokch\"}}".to_string();
//     let call_type: String = "query".to_string();
//     let result = engine.call(call_type, json_msg);
//     println!("Call return msg [{}]", result);
// }
//
//
// fn simulate_by_auto_analyze(engine: &mut ContractInstance) {
//     engine.analyzer.dump_all_members();
//     loop {
//         let mut is_enum = false;
//         let mut call_type = String::new();
//         let mut call_param = String::new();
//         println!("Input call type(init | handle | query):");
//         input_with_out_handle(&mut call_type);
//         if call_type.ne("init") && call_type.ne("handle") && call_type.ne("query") {
//             println!("Wrong call type[{}], must one of (init | handle | query)", call_type);
//             continue;
//         }
//         print!("Input Call param from [ ");
//         for k in engine.analyzer.map_of_member.keys() {
//             print!("{} | ", k);
//         }
//         print!(" ]\n");
//         input_with_out_handle(&mut call_param);
//
//         let msg_type = match engine.analyzer.map_of_member.get(call_param.as_str()) {
//             None => {
//                 println!("can not find msg type {}", call_param.as_str());
//                 continue;
//             }
//             Some(v) => v
//         };
//         let len = msg_type.len();
//         if len > 1 {
//             //only one msg
//             is_enum = true;
//
//             print!("Input Call param from [ ");
//             for k in msg_type.keys() {
//                 print!("{} | ", k);
//             }
//             print!(" ]\n");
//             call_param.clear();
//             input_with_out_handle(&mut call_param);
//         }
//
//
//         let msg = match msg_type.get(call_param.as_str()) {
//             None => {
//                 println!("can not find msg type {}", call_param.as_str());
//                 continue;
//             }
//             Some(v) => v
//         };
//         show_message_type(call_param.as_str(), msg, &engine);
//
//         let json_msg = input_message(call_param.as_str(), msg, &engine, &is_enum);
//         println!("Call msg [{}]", json_msg);
//
//         let result = engine.call(call_type, json_msg);
//         println!("Call return msg [{}]", result);
//     }
// }
//
// fn simulate_by_json(engine: &mut ContractInstance) {
//     loop {
//         let mut call_type = String::new();
//         let mut json_msg = String::new();
//         println!("Input call type(init | handle | query):");
//         input_with_out_handle(&mut call_type);
//         if call_type.ne("init") && call_type.ne("handle") && call_type.ne("query") {
//             println!("Wrong call type[{}], must one of (init | handle | query)", call_type);
//             continue;
//         }
//         println!("Input json string:");
//         input_with_out_handle(&mut json_msg);
//         let result = engine.call(call_type, json_msg);
//         println!("Call return msg [{}]", result);
//     }
// }
//
// fn start_simulate(wasmfile: &str, msgs: &Vec<contract_vm::messages::Message>) -> Result<bool, String> {
//     println!("loading {}", wasmfile);
//
//     let mut engine = match contract_vm::build_simulation(wasmfile) {
//         Err(e) => return Err(e),
//         Ok(instance) => instance,
//     };
//
//     for m in msgs {
//         engine.call(m.call_type.clone(), m.message.clone());
//     }
//
//     //
//     // engine.show_module_info();
//     // // engine.analyzer.load_message(&engine.wasm_file);
//     //
//     //
//     //
//     // if engine.analyzer.auto_load_json_schema(&engine.wasm_file) {
//     //     // simulate_by_auto_analyze(&mut engine);
//     //
//     //     for m in msgs {
//     //         engine.call(m.call_type.clone(), m.message.clone());
//     //     }
//     //
//     //     // executing func ["init"] , params is {"decimals":10,"initial_balances":[{"address":"okchainaaaaaaaokch","amount":"100"}],"name":"Test okchain token","symbol":"OKT"}
//     //     simulate2(&mut engine);
//     //     // executing func [init] , params is {"name":"Test okchain token","symbol":"OKT","decimals":10,"initial_balances":[{"address":"okchainaaaaaaaokch","amount":"1006"}]}
//     // }
//     // else {
//     //     println!("failed to load schema for {}", wasmfile);
//     //     // simulate_by_json(&mut engine);
//     // }
//     return Ok(true);
// }

// fn analyze_schema(path: String) -> bool {
//
//     println!("loading json schema: {}", path);
//
//     let data = match load_data_from_file(path.as_str()) {
//         Err(_e) => return false,
//         Ok(code) => code,
//     };
//     let translated: serde_json::Value = match serde_json::from_slice(data.as_slice()) {
//         Ok(trs) => trs,
//         Err(_e) => return false,
//     };
//     let title_must_exist = match translated["title"].as_str() {
//         None => return false,
//         Some(title) => title,
//     };
//
//     let mapping = match translated.as_object() {
//         None => return false,
//         Some(kvs) => kvs,
//     };
//
//     self.map_of_member.insert(title_must_exist.to_string(), HashMap::new());
//     let mut current_member = match self.map_of_member.get_mut(&title_must_exist.to_string()) {
//         None => return false,
//         Some(c) => c
//     };
//     for iter in mapping.iter() {
//         if iter.0 == "definitions" {
//             Analyzer::prepare_definitions(&iter.1, &mut self.map_of_basetype, &mut self.map_of_struct);
//         } else if iter.0 == "required" {
//             let properties = match mapping.get("properties") {
//                 None => continue,
//                 Some(p) => p,
//             };
//
//             Analyzer::build_member(iter.1, properties, &title_must_exist.to_string(), &mut current_member);
//         } else if iter.0 == "anyOf" {
//             let array: &Vec<serde_json::Value> = match iter.1.as_array() {
//                 None => continue,
//                 Some(a) => a,
//             };
//             for sub_item in array {
//                 //TODO: need more security&border check
//                 let requreid = match sub_item.get("required") {
//                     None => continue,
//                     Some(r) => r
//                 };
//                 let name = match requreid[0].as_str() {
//                     None => continue,
//                     Some(n) => n
//                 };
//                 let required = match sub_item.get("properties") {
//                     None => continue,
//                     Some(p) => {
//                         match p.get(name) {
//                             None => continue,
//                             Some(nm) => nm
//                         }
//                     }
//                 };
//
//                 let properties = match required.get("properties") {
//                     None => continue,
//                     Some(pp) => pp
//                 };
//                 let target_required = match required.as_object() {
//                     None => continue,
//                     Some(target) => match target.get("required") {
//                         None => continue,
//                         Some(m) => m
//                     }
//                 };
//                 if name != "null" {
//                     Analyzer::build_member(target_required, properties, &name.to_string(), &mut current_member);
//                 }
//             }
//         }
//     }
//     return true;
// }

//
// fn main() {
//     let matches = App::new("My Super Program")
//         .version("1.0")
//         .author("Kevin K. <kbknapp@gmail.com>")
//         .about("Does awesome things")
//         .arg(Arg::with_name("wasm")
//             .short("w")
//             .long("wasm")
//             .value_name("contract.wasm")
//             .help("Sets a custom config file")
//             .takes_value(true))
//         .arg(Arg::with_name("message")
//             .short("m")
//             .long("message")
//             .value_name("message.json")
//             .help("Sets a custom config file")
//             .takes_value(true))
//         .arg(Arg::with_name("schema")
//             .short("s")
//             .long("schema")
//             .value_name("message.json")
//             .help("Sets a custom config file")
//             .takes_value(true))
//         .arg(Arg::with_name("v")
//             .short("v")
//             .multiple(true)
//             .help("Sets the level of verbosity"))
//         .get_matches();
//
//     // Gets a value for config if supplied by user, or defaults to "default.conf"
//     let config = matches.value_of("wasm")
//     println!("Value for config: {}", config);
//
//     // Calling .unwrap() is safe here because "INPUT" is required (if "INPUT" wasn't
//     // required we could have used an 'if let' to conditionally get the value)
//     println!("Using input file: {}", matches.value_of("INPUT").unwrap());
//
//     // Vary the output based on how many times the user used the "verbose" flag
//     // (i.e. 'myprog -v -v -v' or 'myprog -vvv' vs 'myprog -v'
//     match matches.occurrences_of("v") {
//         0 => println!("No verbose info"),
//         1 => println!("Some verbose info"),
//         2 => println!("Tons of verbose info"),
//         3 | _ => println!("Don't be crazy"),
//     }
//
//     // You can handle information about subcommands by requesting their matches by name
//     // (as below), requesting just the name used, or both at the same time
//     if let Some(matches) = matches.subcommand_matches("test") {
//         if matches.is_present("debug") {
//             println!("Printing debug info...");
//         } else {
//             println!("Printing normally...");
//         }
//     }
//
//     // more program logic goes here...
// }