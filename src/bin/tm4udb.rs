use std::env::args;
use std::io::Read;
use std::net::TcpListener;
use tm4udb::protocol::{Query, Response};
use tm4udb::trivial_local_in_memory::TrivialLocalInMemory;

fn main() {
    let addr = args().nth(1).unwrap();
    let listener = TcpListener::bind(addr).unwrap();
    let mut storage = TrivialLocalInMemory::new();
    while let Some(stream) = listener.incoming().next() {
        let mut stream = stream.unwrap();
        let mut buf = Vec::new();
        stream.read_to_end(&mut buf).unwrap();
        let query: Query = serde_json::from_slice(&buf).unwrap();
        let response = match query {
            Query::Get(key) => match storage.get(&key) {
                Some(value) => Response::Found(value.to_vec()),
                None => Response::NotFound,
            },
            Query::Set(key, value) => {
                storage.set(key, value);
                Response::Ok
            }
            Query::Delete(key) => {
                storage.delete(&key);
                Response::Ok
            }
        };
        serde_json::to_writer(&mut stream, &response).unwrap();
    }
}
