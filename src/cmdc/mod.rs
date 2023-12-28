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

    fn decode_field<'a>(&self, field: &Field<'a>) -> Result<Value<'a>, Box<dyn Error>> {
        match field.field_type {
            FieldType::Struct => Ok(Value::Struct(self.decode_containers(field.data)?)),
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

    fn encode_field(&self, field: &Field) -> Result<Vec<u8>, Box<dyn Error>> {
        // If field is null, return empty vec
        if field.is_null {
            return Ok(vec![]);
        }
        // If field has data, use it
        if !field.data.is_empty() {
            return Ok(field.data.to_vec());
        }
        // Encode field value
        let field_value = field.get_value()?.unwrap();

        match field.field_type {
            FieldType::Struct => Ok(self.encode_containers(field_value.as_struct().unwrap())?),
            FieldType::String => Ok(self.encode_string(field_value.as_string().unwrap())?),
            FieldType::Int8 => Ok(self.encode_int8(field_value.as_int8().unwrap())?),
            FieldType::Int16 => Ok(self.encode_int16(field_value.as_int16().unwrap())?),
            FieldType::Int32 => Ok(self.encode_int32(field_value.as_int32().unwrap())?),
            FieldType::Int64 => Ok(self.encode_int64(field_value.as_int64().unwrap())?),
            FieldType::UInt8 => Ok(self.encode_uint8(field_value.as_uint8().unwrap())?),
            FieldType::UInt16 => Ok(self.encode_uint16(field_value.as_uint16().unwrap())?),
            FieldType::UInt32 => Ok(self.encode_uint32(field_value.as_uint32().unwrap())?),
            FieldType::UInt64 => Ok(self.encode_uint64(field_value.as_uint64().unwrap())?),
            _ => todo!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mdd::FieldType;

    #[test]
    fn test_decode_example() {
        let codec = CmdcCodec {};
        let data = b"<1,18,0,-6,5222,2>[1,-20,(5:three),4,,<1,2,0,452,5222,2>[100]]";
        let mut containers = codec.decode_containers(data).unwrap();
        assert_eq!(containers.containers.len(), 1);

        let container = &mut containers.containers[0];
        assert_eq!(container.fields[0].data, b"1");
        assert_eq!(container.fields[1].data, b"-20");
        assert_eq!(container.fields[2].data, b"(5:three)");
        assert_eq!(container.fields[3].data, b"4");
        assert_eq!(container.fields[4].data, b"");
        assert_eq!(container.fields[5].data, b"<1,2,0,452,5222,2>[100]");

        container.fields[0].field_type = FieldType::UInt8;
        container.fields[1].field_type = FieldType::Int32;
        container.fields[2].field_type = FieldType::String;
        container.fields[3].field_type = FieldType::UInt32;
        container.fields[4].field_type = FieldType::Int8;
        container.fields[5].field_type = FieldType::Struct;

        // field 0 is uint8 1, use pattern match
        match container.fields[0].decode_value() {
            Ok(Some(Value::UInt8(v))) => assert_eq!(*v, 1),
            _ => panic!("Not a UInt8"),
        }

        // field 1 is int32 -20, use as_int32()
        let field1 = container.fields[1].decode_value().unwrap().unwrap();
        assert_eq!(field1.as_int32().unwrap(), -20);

        // field 2 is string 'three', use decode_value() and then value()
        container.fields[2].decode_value().unwrap();
        let field2 = container.fields[2].value().unwrap();
        assert_eq!(field2.as_string().unwrap(), "three");

        // field 3 is uint32 4
        assert_eq!(
            container.fields[3].get_value().err().unwrap().to_string(),
            "Field not decoded yet"
        );
        container.fields[3].decode_value().unwrap();
        let field3 = container.fields[3].get_value().unwrap();
        assert_eq!(field3.unwrap().as_uint32().unwrap(), 4);

        // field 4 is null
        assert_eq!(container.fields[4].decode_value().unwrap().is_none(), true);

        // field 5 as struct
        let field5 = container.fields[5].decode_value().unwrap().unwrap();
        let nested_container = field5.as_struct().unwrap();
        assert_eq!(nested_container.containers.len(), 1);
        assert_eq!(nested_container.containers[0].fields.len(), 1);
        assert_eq!(nested_container.containers[0].fields[0].data, b"100");
    }
}
