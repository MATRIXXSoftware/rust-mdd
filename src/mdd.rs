#[derive(Debug)]
pub struct Containers {
    pub containers: Vec<Container>,
}

#[derive(Debug)]
pub struct Container {
    pub header: Header,
    pub fields: Vec<Field>,
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
pub struct Field {
    pub data: Vec<u8>,
    pub field_type: FieldType,
    // pub value: Box<dyn std::any::Any>,
    // pub value: Box<dyn Value>, // TODO
    pub is_multi: bool,
    pub is_container: bool,
}

pub trait Value {
    fn to_string(&self) -> String;
    fn to_int32(&self) -> i32;
    fn to_uint32(&self) -> u32;
    fn to_bool(&self) -> bool;
}

#[derive(Debug)]
pub enum FieldType {
    String,
    Int32,
    UInt32,
    Bool,
}
