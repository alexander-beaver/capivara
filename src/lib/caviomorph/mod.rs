use bincode::{config, Decode, Encode};

mod cap;

#[derive(Encode, Decode, PartialEq, Debug)]
pub struct BundledLog{
    pub keys: Vec<char>
}