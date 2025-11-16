use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub enum Query {
    Get(Vec<u8>),
    Set(Vec<u8>, Vec<u8>),
    Delete(Vec<u8>),
}

#[derive(Serialize, Deserialize)]
pub enum Response {
    Ok,
    Found(Vec<u8>),
    NotFound,
}
