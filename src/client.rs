use crate::protocol::{Query, Response};
use std::net::TcpStream;

pub struct Client {
    stream: TcpStream,
}

impl Client {
    pub fn new(address: &str) -> Client {
        let stream = TcpStream::connect(address).unwrap();
        Client { stream }
    }

    pub fn get(&mut self, key: Vec<u8>) -> Option<Vec<u8>> {
        let query = Query::Get(key);
        let response = self.request(query);
        match response {
            Response::Found(value) => Some(value),
            Response::NotFound => None,
            _ => unreachable!(),
        }
    }

    pub fn set(&mut self, key: Vec<u8>, value: Vec<u8>) {
        let query = Query::Set(key, value);
        let response = self.request(query);
        match response {
            Response::Ok => (),
            _ => unreachable!(),
        }
    }

    pub fn delete(&mut self, key: Vec<u8>) {
        let query = Query::Delete(key);
        let response = self.request(query);
        match response {
            Response::Ok => (),
            _ => unreachable!(),
        }
    }

    fn request(&mut self, query: Query) -> Response {
        serde_json::to_writer(&mut self.stream, &query).unwrap();
        self.stream.shutdown(std::net::Shutdown::Write).unwrap();
        serde_json::from_reader(&mut self.stream).unwrap()
    }
}
