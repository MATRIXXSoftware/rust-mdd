use crate::mdd::Containers;
use std::error::Error;

pub trait Codec {
    fn decode(&self, data: &[u8]) -> Result<Containers, Box<dyn Error>>;
    fn encode(&self, containers: &Containers) -> Result<Vec<u8>, Box<dyn Error>>;
}
