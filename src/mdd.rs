#[derive(Debug)]
pub struct Containers<'a> {
    pub containers: Vec<Container<'a>>,
}

#[derive(Debug)]
pub struct Container<'a> {
    pub header: Header,
    pub fields: Vec<Field<'a>>,
}

#[derive(Debug)]
pub struct Header {
    pub version: u8,
    pub total_field: u8,
    pub depth: i8,
    pub key: i32,
    pub schema_version: u16,
    pub ext_version: u16,
}

#[derive(Debug)]
pub struct Field<'a> {
    // pub data: Vec<u8>,
    pub data: &'a [u8],
    pub field_type: FieldType,
    pub value: Option<Box<dyn Value>>,
    pub is_multi: bool,
    pub is_container: bool,
}

pub trait Value: std::fmt::Debug {
    fn to_string(&self) -> String;
    fn to_int32(&self) -> i32;
    fn to_uint32(&self) -> u32;
    fn to_bool(&self) -> bool;
}

#[derive(Debug)]
pub enum FieldType {
    Unknown,
    String,
    Int32,
    UInt32,
    Bool,
}

impl<'a> Field<'a> {
    pub fn raw(data: &'a [u8]) -> Self {
        Field {
            data,
            field_type: FieldType::Unknown,
            value: None,
            is_multi: false,
            is_container: false,
        }
    }
}
