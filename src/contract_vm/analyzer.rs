//analyzer for json schema file

use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
// pub mod messages;

//Todo: analyze more detail from json schema file
pub struct StructType {
    pub member_name: String,
    pub member_type: String,
}

pub struct Member {
    pub member_name: String,
    pub member_def: String,
}

pub struct Analyzer {
    pub map_of_basetype: HashMap<String, String>,
    pub map_of_struct: HashMap<String, HashMap<String, String>>,
    pub map_of_member: HashMap<String, HashMap<String, Vec<Member>>>,
}

impl Analyzer {
    pub fn default() -> Self {
        return Analyzer {
            map_of_basetype: HashMap::new(),
            map_of_struct: HashMap::new(),
            map_of_member: HashMap::new(),
        };
    }

    pub fn build_member(required: &serde_json::Value, properties: &serde_json::Value,
                        mem_name: &String, mapper: &mut HashMap<String, Vec<Member>>) -> bool {
        let req_arr = match required.as_array() {
            None => return false,
            Some(arr) => arr
        };
        mapper.insert(mem_name.clone(), Vec::new());
        let vec_mem = match mapper.get_mut(mem_name) {
            None => return false,
            Some(vecm) => vecm
        };

        for req in req_arr {
            let req_str = match req.as_str() {
                None => continue,
                Some(s) => s
            };
            let proper = match properties.get(req_str) {
                None => continue,
                Some(ps) => ps
            };
            let type_name = match proper.get("type") {
                None => match proper.get("$ref") {
                    None => continue,
                    Some(rf) => rf
                },
                Some(s) => s
            };
            let name = match type_name.as_str() {
                None => continue,
                Some(s) => s
            };
            let mut member: Member = Member { member_name: req_str.to_string(), member_def: "".to_string() };
            if name == "array" {
                let item = match proper.get("items") {
                    None => continue,
                    Some(it) => match it.get("$ref") {
                        None => continue,
                        Some(rf) => match rf.as_str() {
                            None => continue,
                            Some(s) => s
                        }
                    }
                };

                //struct
                let seg = match item.rfind('/') {
                    None => 0,
                    Some(idx) => idx,
                };
                let (_, short_name) = item.split_at(seg + 1);
                member.member_def = short_name.to_string();
            } else if name.starts_with("#/definitions") {
                //struct
                let seg = match name.rfind('/') {
                    None => 0,
                    Some(idx) => idx,
                };
                let (_, short_name) = name.split_at(seg + 1);
                member.member_def = short_name.to_string();
            } else {
                //base type
                member.member_def = name.to_string();
            }
            vec_mem.insert(vec_mem.len(), member);
        }
        return true;
    }

    pub fn dump_all_definitions(&self) {
        println!("Base Type :");
        for b in &self.map_of_basetype {
            println!("{} => {}", b.0, b.1);
        }
        println!("Struct Type :");
        for s in &self.map_of_struct {
            println!("{} {{", s.0);
            for member in s.1 {
                println!("\t{} : {}", member.0, member.1);
            }
            println!("}}");
        }
    }

    pub fn dump_all_members(&self) {
        println!("=================================================");
        println!("dump_all_members");

        for b in &self.map_of_member {
            println!("{} {{", b.0);
            for vcm in b.1 {
                println!("{} {{", vcm.0);
                for vc in vcm.1 {
                    println!("\t{} : {}", vc.member_name, vc.member_def);
                }
            }
            println!("}}")
        }

        println!("dump_all_members done");

        println!("=================================================");
    }

    pub fn prepare_definitions(def: &serde_json::Value, base_type: &mut HashMap<String, String>, struct_type: &mut HashMap<String, HashMap<String, String>>) -> bool {
        let mut vec_struct: HashMap<String, String> = HashMap::new();
        let def_arr = match def.as_object() {
            None => return false,
            Some(da) => da
        };

        for d in def_arr {
            let type_def = match d.1.get("type") {
                None => continue,
                Some(t) => t,
            };
            if type_def == "object" {
                //struct
                let prop = match d.1.get("properties") {
                    None => continue,
                    Some(p) => p
                };

                let prop_map = match prop.as_object() {
                    None => continue,
                    Some(pm) => pm
                };
                for p in prop_map {
                    let def = match p.1.get("$ref") {
                        None => continue,
                        Some(s) => s
                    };
                    let def_str = match def.as_str() {
                        None => continue,
                        Some(s) => s
                    };
                    let seg = match def_str.rfind('/') {
                        None => 0,
                        Some(idx) => idx,
                    };
                    let (_, short_name) = def_str.split_at(seg + 1);

                    vec_struct.insert(p.0.to_string(), short_name.to_string());
                }
                struct_type.insert(d.0.to_string(), vec_struct.clone());
            } else {
                //base type
                let def = match type_def.as_str() {
                    None => continue,
                    Some(s) => s
                };
                base_type.insert("".to_string() + d.0, def.to_string());
            }
        }
        return true;
    }

    fn analyze_msg(&mut self, path: String) -> bool {
        let data = match load_data_from_file(path.as_str()) {
            Err(_e) => return false,
            Ok(code) => code,
        };

        let translated: serde_json::Value = match serde_json::from_slice(data.as_slice()) {
            Ok(trs) => trs,
            Err(_e) => return false,
        };

        let mapping = match translated.as_object() {
            None => return false,
            Some(kvs) => kvs,
        };

        for iter in mapping.iter() {
            println!("Auto loading json schema from {}", iter.0);
            let mut msgs = Vec::new();
           if iter.0 == "messages" {
                let array: &Vec<serde_json::Value> = match iter.1.as_array() {
                    None => continue,
                    Some(a) => a,
                };
               for item in array {

                   // let mut msg = match messages::Message::new_instance(item) {
                   //     Ok(instance) => instance,
                   // };

                   let mapping = match item.as_object() {
                       None => return false,
                       Some(kvs) => kvs,
                   };

                   for iter in mapping.iter() {
                       println!("[{}][{}]", iter.0, iter.1);
                       if iter.0 == "message" {
                           msgs.push(iter.1);
                           // msgs.push(self);
                       }
                   }
                }
            }

            for v in msgs {
                println!("[{}]", v);
            }
        }
        return true;
    }

    //load jsonschema file, translate from json string to func:params...
    pub fn try_load_json_schema(&mut self, dir: String) -> bool {
        let all_json_file = std::fs::read_dir(dir).unwrap();

        // all_json_file.all()

        for file in all_json_file {

            // self.analyze_schema(file.unwrap().path().display().to_string());
            self.analyze_msg(file.unwrap().path().display().to_string());
        }
        return true;
    }

    pub fn auto_load_json_schema(&mut self, file_path: &String) -> bool {
        let seg = match file_path.rfind('/') {
            None => return false,
            Some(idx) => idx,
        };
        let (parent_path, _) = file_path.split_at(seg);
        println!("Auto loading json schema from {}/schema", parent_path);

        return self.try_load_json_schema(parent_path.to_string() + "/schema");
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