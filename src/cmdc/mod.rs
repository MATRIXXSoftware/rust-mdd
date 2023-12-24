pub mod decode;
pub mod encode;

use crate::codec::Codec;
use crate::mdd::Containers;
use std::error::Error;

pub struct CmdcCodec {}

impl Codec for CmdcCodec {
    fn decode(&self, data: &[u8]) -> Result<Containers, Box<dyn Error>> {
        self.decode_containers(data)
    }

    fn encode(&self, containers: &Containers) -> Result<Vec<u8>, Box<dyn Error>> {
        self.encode_containers(containers)
    }
}
