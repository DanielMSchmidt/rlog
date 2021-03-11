extern crate serde;
extern crate serde_json;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::Read;

fn read_config(p: &str) -> Result<Box<HashMap<String, String>>, Box<dyn Error>> {
    let mut file = File::open(p).unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    let config: HashMap<String, String> = serde_json::from_str(&data.as_str())?;
    Ok(Box::new(config))
}

fn main() {
    let config = read_config("./rlog.json").expect("Could not read config");
    println!("{:?}", config)
}
