use crate::codec::Codec;
use core::clone::Clone;
use std::error::Error;

#[derive(Debug, Clone)]
pub struct Containers<'a> {
    pub containers: Vec<Container<'a>>,
}

#[derive(Debug, Clone)]
pub struct Container<'a> {
    pub header: Header,
    pub fields: Vec<Field<'a>>,
}

#[derive(Debug, Clone)]
pub struct Header {
    pub version: u8,
    pub total_field: u8,
    pub depth: i8,
    pub key: i32,
    pub schema_version: u16,
    pub ext_version: u16,
}

#[derive(Debug, Clone)]
pub struct Field<'a> {
    // pub data: Vec<u8>,
    pub data: &'a [u8],
    pub field_type: FieldType,
    pub value: Option<Value<'a>>,
    // pub codec: Option<Box<dyn Codec>>,
    pub codec: Option<&'static dyn Codec>,
    pub is_multi: bool,
    pub is_container: bool,
    pub is_null: bool,
}

#[derive(Debug, Clone)]
pub enum Value<'a> {
    Struct(Containers<'a>),
    String(String),
    Int8(i8),
    Int16(i16),
    Int32(i32),
    Int64(i64),
    UInt8(u8),
    UInt16(u16),
    UInt32(u32),
    UInt64(u64),
    Bool(bool),
    Decimal(bigdecimal::BigDecimal),
}

#[derive(Debug, Clone)]
pub enum FieldType {
    Unknown,
    Struct,
    String,
    Int8,
    Int16,
    Int32,
    Int64,
    UInt8,
    UInt16,
    UInt32,
    UInt64,
    Bool,
    Decimal,
}

impl<'a> Field<'a> {
    pub fn raw(data: &'a [u8]) -> Self {
        Field {
            data,
            field_type: FieldType::Unknown,
            value: None,
            codec: None,
            is_multi: false,
            is_container: false,
            is_null: false,
        }
    }

    // pub fn get_value(&mut self) -> &Option<Value<'a>> {
    //     if self.value.is_none() {
    //         let value = self.codec.as_ref().and_then(|codec| {
    //             let value = codec.decode_field(self).unwrap();
    //             Some(value)
    //         });
    //         self.value = value;
    //     }
    //     &self.value
    // }
    //
    pub fn decode_value(&mut self) -> Result<Option<&Value<'a>>, Box<dyn Error>> {
        if self.is_null {
            return Ok(None);
        }
        if self.value.is_none() {
            let codec = match self.codec.as_ref() {
                Some(codec) => codec,
                None => return Err("No codec".into()),
            };

            let value = codec.decode_field(self)?;
            self.value = Some(value);
        }
        Ok(self.value.as_ref())
    }

    pub fn get_value(&self) -> Result<Option<&Value<'a>>, Box<dyn Error>> {
        if self.is_null {
            return Ok(None);
        }
        match &self.value {
            Some(ref v) => Ok(Some(v)),
            None => Err("Field not decoded yet".into()),
        }
    }

    pub fn value(&self) -> Option<&Value<'a>> {
        if self.is_null {
            return None;
        }
        self.value.as_ref()
    }
}

// impl<'a> Clone for Field<'a> {
//     fn clone(&self) -> Self {
//         Field {
//             data: self.data,
//             field_type: self.field_type.clone(),
//             value: self.value.clone(),
//             codec: self.codec,
//             is_multi: self.is_multi,
//             is_container: self.is_container,
//         }
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_string_field() {
        let field_data = b"(6:foobar)";
        let field = Field {
            data: field_data,
            field_type: FieldType::String,
            value: Some(Value::String("foobar".to_string())),
            codec: None,
            is_multi: false,
            is_container: false,
            is_null: false,
        };
        match field.value {
            Some(Value::String(v)) => assert_eq!(v, "foobar"),
            _ => panic!("Not a string"),
        }
    }

    #[test]
    fn test_get_int32_field() {
        let field_data = b"-20";
        let field = Field {
            data: field_data,
            field_type: FieldType::Int32,
            value: Some(Value::Int32(-20)),
            codec: None,
            is_multi: false,
            is_container: false,
            is_null: false,
        };
        match field.value {
            Some(Value::Int32(v)) => assert_eq!(v, -20),
            _ => panic!("Not a int32"),
        }
    }

    #[test]
    fn test_get_struct_field() {
        let field_data = b"<1,18,0,-6,5222,2>[1,20,(5:three),400000]";
        let field = Field {
            data: field_data,
            field_type: FieldType::Struct,
            value: Some(Value::Struct(Containers {
                containers: vec![Container {
                    header: Header {
                        version: 1,
                        total_field: 18,
                        depth: 0,
                        key: -6,
                        schema_version: 5222,
                        ext_version: 2,
                    },
                    fields: vec![
                        Field::raw("1".as_bytes()),
                        Field::raw("20".as_bytes()),
                        Field::raw("(5:three)".as_bytes()),
                        Field::raw("400000".as_bytes()),
                    ],
                }],
            })),
            codec: None,
            is_multi: false,
            is_container: false,
            is_null: false,
        };
        match field.value {
            Some(Value::Struct(v)) => {
                assert_eq!(v.containers.len(), 1);
                assert_eq!(v.containers[0].fields.len(), 4);
                assert_eq!(v.containers[0].fields[0].data, b"1");
                assert_eq!(v.containers[0].fields[1].data, b"20");
                assert_eq!(v.containers[0].fields[2].data, b"(5:three)");
                assert_eq!(v.containers[0].fields[3].data, b"400000");
            }
            _ => panic!("Not a struct"),
        }
    }
}
