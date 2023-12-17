use crate::mdd::Container;
use crate::mdd::Containers;
use std::error::Error;

pub trait Codec {
    fn decode(&self, data: &[u8]) -> Result<Container, Box<dyn Error>>;
    fn encode(&self, containers: &Containers) -> Result<Vec<u8>, Box<dyn Error>>;
}

struct CmdcCodec {}

impl Codec for CmdcCodec {
    fn decode(&self, _data: &[u8]) -> Result<Container, Box<dyn Error>> {
        todo!()
    }

    fn encode(&self, _containers: &Containers) -> Result<Vec<u8>, Box<dyn Error>> {
        todo!()
    }
}
