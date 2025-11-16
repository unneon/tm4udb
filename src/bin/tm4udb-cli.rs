use std::env::{args, var};
use std::net::{Shutdown, TcpStream};
use std::process::exit;
use tm4udb::protocol::{Query, Response};

fn main() {
    let address = var("TM4UDB_ADDRESS").unwrap();
    let mut stream = TcpStream::connect(address).unwrap();

    let verb = args().nth(1).unwrap();
    match verb.as_str() {
        "get" => {
            let key = args().nth(2).unwrap();

            let query = Query::Get(key.into_bytes());
            serde_json::to_writer(&mut stream, &query).unwrap();
            stream.shutdown(Shutdown::Write).ok();

            let resp: Response = serde_json::from_reader(&mut stream).unwrap();
            match resp {
                Response::Found(v) => {
                    println!("{}", String::from_utf8(v).unwrap());
                }
                Response::NotFound => exit(1),
                _ => unreachable!(),
            }
        }
        "set" => {
            let key = args().nth(2).unwrap();
            let value = args().nth(3).unwrap();

            let query = Query::Set(key.into_bytes(), value.into_bytes());
            serde_json::to_writer(&mut stream, &query).unwrap();
            stream.shutdown(Shutdown::Write).ok();

            let resp: Response = serde_json::from_reader(&mut stream).unwrap();
            match resp {
                Response::Ok => {}
                _ => unreachable!(),
            }
        }
        "delete" => {
            let key = args().nth(2).unwrap();

            let query = Query::Delete(key.into_bytes());
            serde_json::to_writer(&mut stream, &query).unwrap();
            stream.shutdown(Shutdown::Write).ok();

            let resp: Response = serde_json::from_reader(&mut stream).unwrap();

            match resp {
                Response::Ok => {}
                _ => unreachable!(),
            }
        }
        _ => panic!(),
    }
}
