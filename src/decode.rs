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

        containers.containers.push(self.decode_container(data)?);

        Ok(containers)
    }

    fn encode(&self, _containers: &Containers) -> Result<Vec<u8>, Box<dyn Error>> {
        todo!()
    }
}

impl CmdcCodec {
    fn decode_container(&self, data: &[u8]) -> Result<Container, Box<dyn Error>> {
        // Decode Header
        let (header, offset) = self.decode_header(data)?;

        // Decode Body
        let slice = &data[offset..];
        let (fields, _offset) = self.decode_body(slice)?;

        Ok(Container { header, fields })
    }

    fn decode_header(&self, data: &[u8]) -> Result<(Header, usize), Box<dyn Error>> {
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
        let mut complete = false;
        while idx < data.len() {
            match data[idx] {
                b'>' => {
                    complete = true;
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
                        _ => return Err(format!("Invalid cMDC header, 6 fields expected").into()),
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

        if complete == false {
            return Err("Invalid cMDC header, missing '>'".into());
        }

        if field_number != 5 {
            return Err("Invalid cMDC header, 6 fields expected".into());
        }

        let field_data = &data[mark..idx - 1];
        header.ext_version = Self::bytes_to_int(field_data)? as u16;

        Ok((header, idx))
    }

    fn bytes_to_int(data: &[u8]) -> Result<i32, Box<dyn Error>> {
        let str_data = std::str::from_utf8(data)?;
        match str_data.parse::<i32>() {
            Ok(v) => Ok(v),
            Err(_) => {
                Err(format!("Invalid cMDC header field '{}', numeric expected", str_data).into())
            }
        }
    }

    fn decode_body(&self, _data: &[u8]) -> Result<(Vec<Field>, usize), Box<dyn Error>> {
        Ok((vec![], 0))
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

    #[test]
    fn test_invalid_header1() {
        let codec = CmdcCodec {};
        let data = b"<1,18,0,-6,5222,2,1>";
        let err = codec.decode(data).unwrap_err();
        assert_eq!(err.to_string(), "Invalid cMDC header, 6 fields expected");
    }

    #[test]
    fn test_invalid_header2() {
        let codec = CmdcCodec {};
        let data = b"<1,18,0,-6,5222[1,20,300,4]";
        let err = codec.decode(data).unwrap_err();
        assert_eq!(
            err.to_string(),
            "Invalid cMDC character '[' in header, numeric expected"
        );
    }

    #[test]
    fn test_invalid_header3() {
        let codec = CmdcCodec {};
        let data = b"<1,18,0,-6,5222,2";
        let err = codec.decode(data).unwrap_err();
        assert_eq!(err.to_string(), "Invalid cMDC header, missing '>'")
    }

    #[test]
    fn test_invalid_header4() {
        let codec = CmdcCodec {};
        let data = b"1,18,0,-6,5222,2>[]";
        let err = codec.decode(data).unwrap_err();
        assert_eq!(
            err.to_string(),
            "Invalid cMDC header, first character must be '<'"
        );
    }

    #[test]
    fn test_invalid_header5() {
        let codec = CmdcCodec {};
        let data = b"<1,18,0,1-6,5222,2>[]";
        let err = codec.decode(data).unwrap_err();
        assert_eq!(
            err.to_string(),
            "Invalid cMDC header field '1-6', numeric expected"
        );
    }
}
