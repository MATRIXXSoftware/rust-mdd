pub mod decode;
pub mod encode;
pub mod value;

use crate::codec::Codec;
use crate::mdd::Containers;
use crate::mdd::Field;
use crate::mdd::FieldType;
use crate::mdd::Value;
use std::error::Error;

static CMDC_CODEC: CmdcCodec = CmdcCodec {};

#[derive(Debug, Clone)]
pub struct CmdcCodec {}

impl Codec for CmdcCodec {
    fn decode<'a>(&self, data: &'a [u8]) -> Result<Containers<'a>, Box<dyn Error>> {
        self.decode_containers(data)
    }

    fn encode(&self, containers: &Containers) -> Result<Vec<u8>, Box<dyn Error>> {
        self.encode_containers(containers)
    }

    fn decode_field<'a>(&self, field: &Field) -> Result<Value<'a>, Box<dyn Error>> {
        match field.field_type {
            FieldType::String => Ok(Value::String(self.decode_string(field.data)?.to_string())),
            FieldType::Int8 => Ok(Value::Int8(self.decode_int8(field.data)?)),
            FieldType::Int16 => Ok(Value::Int16(self.decode_int16(field.data)?)),
            FieldType::Int32 => Ok(Value::Int32(self.decode_int32(field.data)?)),
            FieldType::Int64 => Ok(Value::Int64(self.decode_int64(field.data)?)),
            FieldType::UInt8 => Ok(Value::UInt8(self.decode_uint8(field.data)?)),
            FieldType::UInt16 => Ok(Value::UInt16(self.decode_uint16(field.data)?)),
            FieldType::UInt32 => Ok(Value::UInt32(self.decode_uint32(field.data)?)),
            FieldType::UInt64 => Ok(Value::UInt64(self.decode_uint64(field.data)?)),
            _ => todo!(),
        }
    }

    fn encode_field(&self, _field: &Field) -> Result<Vec<u8>, Box<dyn Error>> {
        todo!()
    }

    // fn clone_box(&self) -> Box<dyn Codec> {
    //     Box::new(self.clone())
    // }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mdd::FieldType;

    #[test]
    fn test_decode_example() {
        let codec = CmdcCodec {};
        let data = b"<1,18,0,-6,5222,2>[1,-20,(5:three),4,]";
        let mut containers = codec.decode_containers(data).unwrap();
        assert_eq!(containers.containers.len(), 1);

        let container = &mut containers.containers[0];
        assert_eq!(container.fields[0].data, b"1");
        assert_eq!(container.fields[1].data, b"-20");
        assert_eq!(container.fields[2].data, b"(5:three)");
        assert_eq!(container.fields[3].data, b"4");
        assert_eq!(container.fields[4].data, b"");

        container.fields[0].field_type = FieldType::UInt8;
        container.fields[1].field_type = FieldType::Int32;
        container.fields[2].field_type = FieldType::String;
        container.fields[3].field_type = FieldType::UInt32;
        container.fields[4].field_type = FieldType::Int8;

        // field 1 is uint8 1
        match container.fields[0].decode_value() {
            Ok(Some(Value::UInt8(v))) => assert_eq!(*v, 1),
            _ => panic!("Not a UInt8"),
        }
        // field 2 is int32 -20
        match container.fields[1].decode_value() {
            Ok(Some(Value::Int32(v))) => assert_eq!(*v, -20),
            _ => panic!("Not a Int32"),
        }
        // field 3 is string 'three'
        match container.fields[2].decode_value() {
            Ok(Some(Value::String(v))) => assert_eq!(v, "three"),
            _ => panic!("Not a String"),
        }
        // field 4 is uint32 4
        match container.fields[3].decode_value() {
            Ok(Some(Value::UInt32(v))) => assert_eq!(*v, 4),
            _ => panic!("Not a UInt32"),
        }
        // field 5 is null
        assert_eq!(container.fields[4].decode_value().unwrap().is_none(), true);
    }
}
