
extern crate clap;

pub mod contract_vm;

use clap::{Arg, App};
use std::io;
use std::ops::Add;
use crate::contract_vm::engine::ContractInstance;


fn show_message_type(name : &str,members : &Vec<contract_vm::analyzer::Member>,engine : &contract_vm::engine::ContractInstance){
    println!("{} {{",name);
    for vcm in members {
        let st = match engine.analyzer.map_of_struct.get_key_value(vcm.member_def.as_str()){
            Some(h) => h,
            _ => {
                println!("\t{} : {}",vcm.member_name,vcm.member_def);
                continue;
            }
        };
        //todo:need show all members by recursive invocation
        println!("\t{} : {} :{{ ",vcm.member_name,vcm.member_def);
        for members in st.1 {
            println!("\t\t{} : {}",members.0,members.1);
        }
        println!("\t}}")
    }
    println!("}}");
}

fn check_is_need_flag(name : &str) -> bool{
    if name == "integer" {
        return false;
    }
    return true;
}

fn to_json_item(name : &String,data : &String,type_name : &str) -> String{
    let mut params = "\"".to_string();
    params += name.as_str();
    params += "\":";
    if check_is_need_flag(type_name){
        params += "\"";
    }
    params += data.as_str();
    if check_is_need_flag(type_name){
        params += "\"";
    }
    params += ",";
    return params;
}

fn input_with_out_handle(input_data : &mut String) -> bool{
    match io::stdin().read_line(input_data) {
        Ok(_n) => {
            if input_data.ends_with("\n") {
                input_data.remove(input_data.len() - 1);
            }
        }
        Err(error) => {
            println!("error: {}", error);
            return false;
        },
    }
    return true;
}

fn input_type(mem_name : &String, type_name : &String, engine : &contract_vm::engine::ContractInstance) -> String{
    println!("input [{}]:",mem_name);
    let st = match engine.analyzer.map_of_struct.get_key_value(type_name){
        Some(h) => h,
        _ => {
            let mut single : String = String::new();
            input_with_out_handle(&mut single);
            return to_json_item(&mem_name,&single,type_name);
        }
    };
    //todo:need show all members by recursive invocation
    let mut params = "\"".to_string();
    params += mem_name;
    params += "\":[{";
    for members in st.1 {
        println!("input \t[{} : {}]:",members.0,members.1);
        let mut single : String = String::new();
        input_with_out_handle(&mut single);
        params += to_json_item(&members.0,&single,type_name).as_str();
    }
    let (resv,_) = params.split_at(params.len() - 1);
    let mut ret = resv.to_string();
    ret += "}],";

    return ret;
}

fn input_message(name : &str, members : &Vec<contract_vm::analyzer::Member>, engine : &contract_vm::engine::ContractInstance, is_enum : &bool) -> String{
    let mut final_msg : String = "{".to_string();
    if *is_enum{
        final_msg = final_msg.add("\"");
        final_msg = final_msg.add(name);
        final_msg = final_msg.add("\":{");
    }

    for vcm in members{
        final_msg = final_msg.add(input_type(&vcm.member_name,&vcm.member_def.to_string(),engine).as_str());
    }
    if members.len() > 0 {
        let (resv,_) = final_msg.split_at(final_msg.len() - 1);
        final_msg = resv.to_string();
        final_msg = final_msg.add("}");
    }else {
        final_msg = "}".to_string();
    }
    if *is_enum{
        final_msg = final_msg.add("}");
    }

    println!("JsonMsg:{}",final_msg);
    return final_msg;
}

fn simulate_by_auto_analyze(engine : &mut ContractInstance){

    engine.analyzer.dump_all_members();
    loop {
        let mut is_enum = false;
        let mut call_type = String::new();
        let mut call_param = String::new();
        println!("Input call type(init | handle | query):");
        input_with_out_handle(&mut call_type);
        if call_type.ne("init") && call_type.ne("handle") && call_type.ne("query") {
            println!("Wrong call type[{}], must one of (init | handle | query)",call_type);
            continue;
        }
        print!("Input Call param from [ ");
        for k in engine.analyzer.map_of_member.keys(){
            print!("{} | ",k);
        }
        print!(" ]\n");
        input_with_out_handle(&mut call_param);

        let msg_type = match engine.analyzer.map_of_member.get(call_param.as_str()){
            None => {
                println!("can not find msg type {}",call_param.as_str());
                continue;
            },
            Some(v) => v
        };
        let len = msg_type.len();
        if  len > 1 {
            //only one msg
            is_enum = true;

            print!("Input Call param from [ ");
            for k in msg_type.keys(){
                print!("{} | ",k);
            }
            print!(" ]\n");
            call_param.clear();
            input_with_out_handle(&mut call_param);
        }


        let msg = match msg_type.get(call_param.as_str()){
            None => {
                println!("can not find msg type {}",call_param.as_str());
                continue;
            },
            Some(v) => v
        };
        show_message_type(call_param.as_str(),msg,&engine);

        let json_msg = input_message(call_param.as_str(),msg,&engine,&is_enum);

        let result = engine.call(call_type,json_msg);
        println!("Call return msg [{}]",result);
    }
}

fn simulate_by_json(engine : &mut ContractInstance){
    loop {
        let mut call_type = String::new();
        let mut json_msg = String::new();
        println!("Input call type(init | handle | query):");
        input_with_out_handle(&mut call_type);
        if call_type.ne("init") && call_type.ne("handle") && call_type.ne("query") {
            println!("Wrong call type[{}], must one of (init | handle | query)",call_type);
            continue;
        }
        println!("Input json string:");
        input_with_out_handle(&mut json_msg);
        let result = engine.call(call_type,json_msg);
        println!("Call return msg [{}]",result);
    }
}

fn start_simulate(wasmfile:&str) -> Result<bool,String>{
    println!("loading {}",wasmfile);
    let mut engine = match contract_vm::build_simulation(wasmfile) {
        Err(e) => return Err(e),
        Ok(instance) => instance,
    };

    engine.show_module_info();
    if engine.analyzer.auto_load_json_schema(&engine.wasm_file){
        simulate_by_auto_analyze(&mut engine);
    }else{
        simulate_by_json(&mut engine);
    }
    return Ok(true);

}

fn prepare_command_line() -> bool{
    let matches = App::new("cosmwasm-simulate")
        .version("0.1.0")
        .author("github : https://github.com/KamiD")
        .about("A simulation of cosmwasm smart contract system")
        .arg(Arg::with_name("run")
            .help("contract file that built by https://github.com/CosmWasm/rust-optimizer")
            .empty_values(false)
        )
        .get_matches();

    if let Some(file) = matches.value_of("run"){
        if !file.ends_with(".wasm") {
            println!("only support file[*.wasm], you just input a wrong file format - {:?}",file);
            return false;
        }
        match start_simulate(file) {
            Ok(t) => {
                if t{
                    println!("start_simulate success");
                }else {
                    println!("start_simulate failed")
                }
            },
            Err(e) => println!("error occurred during call start_simulate : {}",e)
        }
        return true;
    }
    return false;
}

fn main() {
    prepare_command_line();
}
