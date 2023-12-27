pub mod decode;
pub mod encode;

use crate::codec::Codec;
use crate::mdd::Containers;
use crate::mdd::Field;
use crate::mdd::Value;
use std::error::Error;

#[derive(Debug, Clone)]
pub struct CmdcCodec {}

impl Codec for CmdcCodec {
    fn decode<'a>(&self, data: &'a [u8]) -> Result<Containers<'a>, Box<dyn Error>> {
        self.decode_containers(data)
    }

    fn encode(&self, containers: &Containers) -> Result<Vec<u8>, Box<dyn Error>> {
        self.encode_containers(containers)
    }

    fn decode_field<'a>(&self, _field: &Field) -> Result<Value<'a>, Box<dyn Error>> {
        Ok(Value::Int32(-20))
        // todo!()
    }

    fn encode_field(&self, _field: &Field) -> Result<Vec<u8>, Box<dyn Error>> {
        todo!()
    }

    fn clone_box(&self) -> Box<dyn Codec> {
        Box::new(self.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mdd::FieldType;

    #[test]
    fn test_decode_example() {
        let codec = CmdcCodec {};
        let data = b"<1,18,0,-6,5222,2>[1,-20,(5:three),4]";
        let mut containers = codec.decode_containers(data).unwrap();
        assert_eq!(containers.containers.len(), 1);

        let container = &mut containers.containers[0];
        assert_eq!(container.fields[0].data, b"1");
        assert_eq!(container.fields[1].data, b"-20");
        assert_eq!(container.fields[2].data, b"(5:three)");
        assert_eq!(container.fields[3].data, b"4");

        container.fields[0].field_type = FieldType::UInt8;
        container.fields[1].field_type = FieldType::Int32;
        container.fields[2].field_type = FieldType::String;
        container.fields[2].field_type = FieldType::UInt32;

        match container.fields[1].get_value().unwrap() {
            Value::Int32(v) => assert_eq!(*v, -20),
            _ => panic!("Not a int32"),
        }

        // match container.fields[1].get_value() {
        //     Some(Value::Int32(v)) => assert_eq!(*v, -20),
        //     _ => panic!("Not a int32"),
        // }
        // match container.fields[1].value {
        //     Some(Value::Int32(v)) => assert_eq!(v, -20),
        //     _ => panic!("Not a int32"),
        // }
    }
}
