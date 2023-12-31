use crate::error::Error;
use crate::mdd::Containers;
use crate::mdd::Field;
use crate::mdd::Value;

pub trait Codec: std::fmt::Debug {
    fn decode<'a>(&self, data: &'a [u8]) -> Result<Containers<'a>, Error>;
    fn encode(&self, containers: &Containers) -> Result<Vec<u8>, Error>;

    fn decode_field<'a>(&self, field: &Field<'a>) -> Result<Value<'a>, Error>;
    fn encode_field(&self, field: &Field) -> Result<Vec<u8>, Error>;
}
