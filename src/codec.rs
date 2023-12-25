use crate::mdd::Containers;
use std::error::Error;

pub trait Codec {
    fn decode<'a>(&self, data: &'a [u8]) -> Result<Containers<'a>, Box<dyn Error>>;
    fn encode(&self, containers: &Containers) -> Result<Vec<u8>, Box<dyn Error>>;
}
