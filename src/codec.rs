use crate::mdd::Containers;
use crate::mdd::Field;
use crate::mdd::Value;
use std::error::Error;

pub trait Codec: std::fmt::Debug {
    fn decode<'a>(&self, data: &'a [u8]) -> Result<Containers<'a>, Box<dyn Error>>;
    fn encode(&self, containers: &Containers) -> Result<Vec<u8>, Box<dyn Error>>;

    fn decode_field<'a>(&self, field: &Field) -> Result<Value<'a>, Box<dyn Error>>;
    fn encode_field(&self, field: &Field) -> Result<Vec<u8>, Box<dyn Error>>;

    fn clone_box(&self) -> Box<dyn Codec>;
}
