extern crate reqwest;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

use std::io::Read;
use serde_json::Error;

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
#[serde(rename_all = "snake_case")]
struct Bitcoin {
    EUR: Eur
}

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
struct Eur {
    last: f64
}

fn get_bitcoin() -> Result<Bitcoin, Error> {
    let mut resp = reqwest::get("https://blockchain.info/ticker").unwrap();
    assert!(resp.status().is_success());
    let mut content = String::new();
    let _res = resp.read_to_string(&mut content);
    let b: Bitcoin = serde_json::from_str(content.as_str())?;
    Ok(b)
}

fn main() {
    let b: Bitcoin = get_bitcoin().unwrap();
    let mut color: String = format!("%{{F#BEB6AE}}");
    println!(" {}{:?}%{{F-}} ", color, b.EUR.last);
}
