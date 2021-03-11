extern crate serde;
extern crate serde_json;
use std::collections::HashMap;
use std::error::Error;
use std::io::Read;
use tokio::fs::File;
use tokio::io::{self, AsyncReadExt};

fn read_config(p: &str) -> Result<Box<HashMap<String, String>>, Box<dyn Error>> {
    let mut file = std::fs::File::open(p).unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    let config: HashMap<String, String> = serde_json::from_str(&data.as_str())?;
    Ok(Box::new(config))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config = read_config("./rlog.json").expect("Could not read config");
    println!("{:?}", config);

    // Create streams for each of the files and combine streams into a sink output to stdout
    // https://www.zfnd.org/blog/decoding-bitcoin-messages-with-tokio-codecs/
    // https://dev.to/jtenner/creating-a-tokio-codec-1f0l
    // https://docs.rs/tokio-codec/0.1.1/tokio_codec/struct.Framed.html
    // https://docs.rs/tokio/1.3.0/tokio/fs/struct.File.html#examples-13

    for (name, log_path) in config.iter() {
        println!("{} / {}", name, log_path);

        let file = File::open(log_path).await?;
    }

    Ok(())
}
