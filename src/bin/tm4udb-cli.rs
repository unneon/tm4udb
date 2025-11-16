use std::env::{args, var};
use std::process::exit;
use tm4udb::client::Client;

fn main() {
    let address = var("TM4UDB_ADDRESS").unwrap();
    let mut client = Client::new(&address);

    let verb = args().nth(1).unwrap();
    match verb.as_str() {
        "get" => {
            let key = args().nth(2).unwrap();
            let value = client.get(key.into_bytes());
            if let Some(value) = value {
                println!("{}", String::from_utf8(value).unwrap());
            } else {
                exit(1);
            }
        }
        "set" => {
            let key = args().nth(2).unwrap();
            let value = args().nth(3).unwrap();
            client.set(key.into_bytes(), value.into_bytes());
        }
        "delete" => {
            let key = args().nth(2).unwrap();
            client.delete(key.into_bytes());
        }
        _ => panic!(),
    }
}
