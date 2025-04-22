extern crate serde_json;
extern crate iced;

mod gobelocal;
mod goberemote;
mod utils;

use std::{env::set_current_dir, fs};
use serde_json::{json, Value};


fn main() {
    let s1: &String = &"2".to_string();
    let s2: &String = &"2".to_string();
    print!("{}", s1==s2)
    // let res: Result<String, std::io::Error> = fs::read_to_string("./data.json"); // read file into string
    // let s = res.unwrap();

    // let mut json_data: serde_json::Value = serde_json::from_str(&s)
    //     .expect("Can't parse json"); // convert string to json object and panic in case of error

    // println!("Data: {}", json_data);
    // println!("Bob age: {}", json_data["A#1"]);
    // json_data["A#1"] = serde_json::json!("123");

    // if let Some(o) = json_data.as_object_mut() {
    //     o.insert("SENT".to_string(), json!(3));
    // }

    // std::fs::write("data.json", serde_json::to_string_pretty(&json_data).unwrap())
    // .expect("Can't write to file");

    // // change values
    // json_data[0]["age"] = serde_json::json!(123);
    // json_data[0]["name"] = serde_json::json!("mascai");
    // println!("Data: {}", json_data);
    
    // std::fs::write("output.json", serde_json::to_string_pretty(&json_data).unwrap())
    //     .expect("Can't write to file");
}