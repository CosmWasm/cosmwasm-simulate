use std::fmt::Write;

pub fn logger_storage_event_insert(key: &[u8], value: &[u8]){
    let mut key_str = match std::str::from_utf8(&key){
        Ok(result) => result.to_string(),
        _ => "".to_string()
    };
    if key_str.is_empty() {
        for a in key.iter() {
            write!(key_str, "{:02x}", a).expect("Not written");
        }
    }

    let mut val_str = match std::str::from_utf8(&value){
        Ok(result) => result.to_string(),
        _ => "".to_string()
    };
    if val_str.is_empty() {
        for a in key.iter() {
            write!(val_str, "{:02x}", a).expect("Not written");
        }
    }

    println!("DB Changed : [Insert]\nKey        : [{}]\nValue      : [{}]",key_str,val_str);
}

pub fn logger_storage_event_remove(key: &[u8]){
    let mut key_str = match std::str::from_utf8(&key){
        Ok(result) => result.to_string(),
        _ => "".to_string()
    };
    if key_str.is_empty() {
        for a in key.iter() {
            write!(key_str, "{:02x}", a).expect("Not written");
        }
    }

    println!("DB Changed : [Remove]\nKey        : [{}]",key_str);
}