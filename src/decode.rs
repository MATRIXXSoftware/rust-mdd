use crate::mdd::Container;
use crate::mdd::Containers;
use crate::mdd::Field;
use crate::mdd::Header;
use std::error::Error;

pub trait Codec {
    fn decode(&self, data: &[u8]) -> Result<Containers, Box<dyn Error>>;
    fn encode(&self, containers: &Containers) -> Result<Vec<u8>, Box<dyn Error>>;
}

struct CmdcCodec {}

impl Codec for CmdcCodec {
    fn decode(&self, data: &[u8]) -> Result<Containers, Box<dyn Error>> {
        let mut containers = Containers { containers: vec![] };

        containers.containers.push(Container {
            header: self.encode_header(data)?,
            fields: self.encode_body()?,
        });

        Ok(containers)
    }

    fn encode(&self, _containers: &Containers) -> Result<Vec<u8>, Box<dyn Error>> {
        todo!()
    }
}

impl CmdcCodec {
    fn encode_header(&self, data: &[u8]) -> Result<Header, Box<dyn Error>> {
        let mut header = Header {
            version: 0,
            total_field: 0,
            depth: 0,
            key: 0,
            schema_version: 0,
            ext_version: 0,
        };

        if data.is_empty() {
            return Err("Invalid cMDC header, no header".into());
        }
        if data[0] != b'<' {
            return Err("Invalid cMDC header, first character must be '<'".into());
        }

        let mut field_number = 0;
        let mut idx = 1;
        let mut mark = idx;
        while idx < data.len() {
            match data[idx] {
                b'>' => {
                    idx += 1;
                    break;
                }
                b',' => {
                    let field_data = &data[mark..idx];
                    let v = Self::bytes_to_int(field_data)?;

                    match field_number {
                        0 => header.version = v as u8,
                        1 => header.total_field = v as u8,
                        2 => header.depth = v as i8,
                        3 => header.key = v as i32,
                        4 => header.schema_version = v as u16,
                        _ => return Err(format!("Invalid cMDC header, 6 fields exxpected").into()),
                    }
                    field_number += 1;

                    mark = idx + 1;
                    idx += 1;
                    continue;
                }
                c if c.is_ascii_digit() || c == b'-' => {}
                c => {
                    return Err(format!(
                        "Invalid cMDC character '{}' in header, numeric expected",
                        c as char
                    )
                    .into())
                }
            }
            idx += 1;
        }

        if field_number != 5 {
            return Err("Invalid cMDC header, 6 fields expected".into());
        }

        let field_data = &data[mark..idx - 1];
        header.ext_version = Self::bytes_to_int(field_data)? as u16;

        Ok(header)
    }

    fn bytes_to_int(data: &[u8]) -> Result<i32, Box<dyn Error>> {
        let str_data = std::str::from_utf8(data)?;
        Ok(str_data.parse::<i32>()?)
    }

    fn encode_body(&self) -> Result<Vec<Field>, Box<dyn Error>> {
        Ok(vec![])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode() {
        let codec = CmdcCodec {};
        let data = b"<1,18,0,-6,5222,2>[1,20,300,4]";

        let result = codec.decode(data);
        match result {
            Ok(containers) => {
                let container = &containers.containers[0];
                assert_eq!(container.header.version, 1);
                assert_eq!(container.header.total_field, 18);
                assert_eq!(container.header.depth, 0);
                assert_eq!(container.header.key, -6);
                assert_eq!(container.header.schema_version, 5222);
                assert_eq!(container.header.ext_version, 2);
                // assert_eq!(container.fields.len(), 1);
                // assert_eq!(container.fields[0].data, b"[1,20,300,4]");
                // assert_eq!(container.fields[0].field_type, FieldType::String);
                // assert_eq!(container.fields[0].is_multi, false);
                // assert_eq!(container.fields[0].is_continue, false);
            }
            Err(err) => {
                panic!("decode error: {}", err);
            }
        }
    }
}
