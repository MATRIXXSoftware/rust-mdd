use crate::mdd::Container;
use crate::mdd::Containers;
use crate::mdd::Field;
use crate::mdd::FieldType;
use crate::mdd::Header;
use std::error::Error;
use std::io::Write;

pub trait Codec {
    fn decode(&self, data: &[u8]) -> Result<Containers, Box<dyn Error>>;
    fn encode(&self, containers: &Containers) -> Result<Vec<u8>, Box<dyn Error>>;
}

pub struct CmdcCodec {}

impl Codec for CmdcCodec {
    fn decode(&self, data: &[u8]) -> Result<Containers, Box<dyn Error>> {
        let mut containers = Containers { containers: vec![] };

        let mut idx = 0;
        while idx < data.len() {
            let (container, offset) = self.decode_container(&data[idx..])?;
            idx += offset;
            containers.containers.push(container);
        }

        Ok(containers)
    }

    fn encode(&self, containers: &Containers) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut data = Vec::new();

        for container in &containers.containers {
            let container_data = self.encode_container(container)?;
            data.extend(container_data);
        }

        Ok(data)
    }
}

impl CmdcCodec {
    fn decode_container(&self, data: &[u8]) -> Result<(Container, usize), Box<dyn Error>> {
        let mut idx = 0;

        // Decode Header
        let (header, offset) = self.decode_header(data)?;
        idx += offset;

        // Decode Body
        let slice = &data[idx..];
        let (fields, offset) = self.decode_body(slice)?;
        idx += offset;

        Ok((Container { header, fields }, idx))
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
                    let v = Self::bytes_to_int(field_data)
                        .map_err(|err| format!("Invalid cMDC header, {}", err))?;

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
        let v = Self::bytes_to_int(field_data)
            .map_err(|err| format!("Invalid cMDC header, {}", err))?;
        header.ext_version = v as u16;

        Ok((header, idx))
    }

    fn decode_body(&self, data: &[u8]) -> Result<(Vec<Field>, usize), Box<dyn Error>> {
        let mut fields = vec![];

        if data.is_empty() {
            return Err("Invalid cMDC body, no body".into());
        }
        if data[0] != b'[' {
            return Err("Invalid cMDC body, first character must be '['".into());
        }

        let mut idx = 1;
        let mut mark = idx;
        let mut round_mark = 0;

        let mut square = 1;
        let mut angle = 0;
        let mut round = 0;
        let mut curly = 0;

        let mut is_multi = false;
        let mut is_container = false;
        let mut complete = false;

        while idx < data.len() {
            let c = data[idx];
            // println!("c: {}", c as char);

            if round != 0 {
                if c == b')' {
                    round -= 1;
                } else if round_mark == 0 {
                    return Err("Invalid cMDC body, mismatch string length".into());
                } else if c == b':' {
                    let field_data = &data[round_mark + 1..idx];
                    let len = Self::bytes_to_int(field_data)
                        .map_err(|err| format!("Invalid string field, {}", err))?;
                    idx += len as usize; // skip the string field
                    round_mark = 0; // reset round mark
                } else if !c.is_ascii_digit() {
                    return Err(format!(
                        "Invalid character '{}', numeric expected for string length",
                        c as char
                    )
                    .into());
                }
                idx += 1;
                continue;
            }

            match c {
                b'(' => {
                    round_mark = idx;
                    round += 1;
                }
                b'[' => square += 1,
                b']' => square -= 1,
                b'<' => {
                    is_container = true;
                    angle += 1;
                }
                b'>' => angle -= 1,
                b'{' => {
                    curly += 1;
                    is_multi = true;
                }
                b'}' => curly -= 1,
                b',' => {
                    if square == 1 && angle == 0 && curly == 0 {
                        // Extract fields
                        let field_data = &data[mark..idx];
                        // println!("field_data: {:?}", std::str::from_utf8(field_data));

                        mark = idx + 1;
                        let field = Field {
                            data: field_data.to_vec(),
                            field_type: FieldType::Unknown,
                            value: None,
                            is_multi,
                            is_container,
                        };
                        fields.push(field);
                        is_multi = false;
                        is_container = false;
                    }
                }
                _ => {}
            }

            if square == 0 {
                complete = true;
                idx += 1;
                break;
            }

            idx += 1;
        }

        if !complete {
            return Err("Invalid cMDC body, no end of body".into());
        }

        // Extract last field
        let field_data = &data[mark..idx - 1];
        // println!("field_data: {:?}", std::str::from_utf8(field_data));

        let field = Field {
            data: field_data.to_vec(),
            field_type: FieldType::Unknown,
            value: None,
            is_multi,
            is_container,
        };
        fields.push(field);

        Ok((fields, idx))
    }

    fn bytes_to_int(data: &[u8]) -> Result<i32, Box<dyn Error>> {
        let str_data = std::str::from_utf8(data)?;
        str_data
            .parse::<i32>()
            .map_err(|_| format!("Invalid digit found in '{}'", str_data).into())
    }

    fn encode_container(&self, container: &Container) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut data = Vec::new();

        // Encode Header
        let header_data = self.encode_header(&container.header)?;
        data.extend(header_data);

        // Encode Body
        let body_data = self.encode_body(&container.fields)?;
        data.extend(body_data);

        Ok(data)
    }

    fn encode_header(&self, header: &Header) -> Result<Vec<u8>, Box<dyn Error>> {
        // Predefine the capacity of the vector to avoid reallocation
        let estimated_size = 4 + 1 + 1 + 7 + 4 + 3 + 6 + 2;
        let mut data = Vec::with_capacity(estimated_size);
        // let mut data = Vec::new();

        write!(
            data,
            "<{},{},{},{},{},{}>",
            header.version,
            header.total_field,
            header.depth,
            header.key,
            header.schema_version,
            header.ext_version
        )?;

        Ok(data)
    }

    fn encode_body(&self, fields: &[Field]) -> Result<Vec<u8>, Box<dyn Error>> {
        // Predefine the capacity of the vector to avoid reallocation
        let mut estimated_len = fields.len() + 2;
        for field in fields.iter() {
            estimated_len += field.data.len();
        }
        let mut data = Vec::with_capacity(estimated_len);
        // let mut data = Vec::new();

        data.push(b'[');
        for (i, field) in fields.iter().enumerate() {
            if i > 0 {
                data.push(b',');
            }
            // Use reference instead of clone for best performrance, but might be risky
            data.extend(&field.data);
            // data.extend(field.data.clone());
        }
        data.push(b']');

        Ok(data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_single_container1() {
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

                assert_eq!(container.fields.len(), 4);
                assert_eq!(container.fields[0].data, b"1");
                assert_eq!(container.fields[1].data, b"20");
                assert_eq!(container.fields[2].data, b"300");
                assert_eq!(container.fields[3].data, b"4");
            }
            Err(err) => {
                panic!("decode error: {}", err);
            }
        }
    }

    #[test]
    fn test_decode_single_container2() {
        let codec = CmdcCodec {};
        let data = b"<1,18,0,-6,5222,2>[,(6:value2),3,2021-09-07T08:00:25.000001Z,2021-10-31,09:13:02.667997Z,88,5.5,]";

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

                assert_eq!(container.fields.len(), 9);
                assert_eq!(container.fields[0].data, b"");
                assert_eq!(container.fields[1].data, b"(6:value2)");
                assert_eq!(container.fields[2].data, b"3");
                assert_eq!(container.fields[3].data, b"2021-09-07T08:00:25.000001Z");
                assert_eq!(container.fields[4].data, b"2021-10-31");
                assert_eq!(container.fields[5].data, b"09:13:02.667997Z");
                assert_eq!(container.fields[6].data, b"88");
                assert_eq!(container.fields[7].data, b"5.5");
                assert_eq!(container.fields[8].data, b"");
            }
            Err(err) => {
                panic!("decode error: {}", err);
            }
        }
    }

    #[test]
    fn test_decode_multi_containers() {
        let codec = CmdcCodec {};
        let data = b"<1,18,0,-6,5222,2>[1,20,300,4]<1,5,0,-7,5222,2>[,2,(3:def),4]";

        let result = codec.decode(data);
        match result {
            Ok(containers) => {
                assert_eq!(containers.containers.len(), 2);
                let container0 = &containers.containers[0];
                assert_eq!(container0.header.version, 1);
                assert_eq!(container0.header.total_field, 18);
                assert_eq!(container0.header.depth, 0);
                assert_eq!(container0.header.key, -6);
                assert_eq!(container0.header.schema_version, 5222);
                assert_eq!(container0.header.ext_version, 2);

                assert_eq!(container0.fields.len(), 4);
                assert_eq!(container0.fields[0].data, b"1");
                assert_eq!(container0.fields[1].data, b"20");
                assert_eq!(container0.fields[2].data, b"300");
                assert_eq!(container0.fields[3].data, b"4");

                let container1 = &containers.containers[1];
                assert_eq!(container1.header.version, 1);
                assert_eq!(container1.header.total_field, 5);
                assert_eq!(container1.header.depth, 0);
                assert_eq!(container1.header.key, -7);
                assert_eq!(container1.header.schema_version, 5222);
                assert_eq!(container1.header.ext_version, 2);

                assert_eq!(container1.fields.len(), 4);
                assert_eq!(container1.fields[0].data, b"");
                assert_eq!(container1.fields[1].data, b"2");
                assert_eq!(container1.fields[2].data, b"(3:def)");
                assert_eq!(container1.fields[3].data, b"4");
            }
            Err(err) => {
                panic!("decode error: {}", err);
            }
        }
    }

    #[test]
    fn test_decode_nested_containers() {
        let codec = CmdcCodec {};
        let data = b"<1,18,0,-6,5222,2>[1,20,<1,2,0,452,5222,2>[100],4]";
        let containers = codec.decode(data).unwrap();
        let container = &containers.containers[0];

        assert_eq!(container.fields.len(), 4);
        assert_eq!(container.fields[0].data, b"1");
        assert_eq!(container.fields[1].data, b"20");
        assert_eq!(container.fields[2].data, b"<1,2,0,452,5222,2>[100]");
        assert_eq!(container.fields[3].data, b"4");

        assert_eq!(container.fields[0].is_container, false);
        assert_eq!(container.fields[1].is_container, false);
        assert_eq!(container.fields[2].is_container, true);
        assert_eq!(container.fields[3].is_container, false);
    }

    #[test]
    fn test_list_integer_value() {
        let codec = CmdcCodec {};
        let data = b"<1,18,0,-6,5222,2>[0,{1,2,3},,,300,{4,5}]";
        let containers = codec.decode(data).unwrap();
        let container = &containers.containers[0];

        assert_eq!(container.fields.len(), 6);
        assert_eq!(container.fields[0].data, b"0");
        assert_eq!(container.fields[1].data, b"{1,2,3}");
        assert_eq!(container.fields[2].data, b"");
        assert_eq!(container.fields[3].data, b"");
        assert_eq!(container.fields[4].data, b"300");
        assert_eq!(container.fields[5].data, b"{4,5}");

        assert_eq!(container.fields[0].is_multi, false);
        assert_eq!(container.fields[1].is_multi, true);
        assert_eq!(container.fields[2].is_multi, false);
        assert_eq!(container.fields[3].is_multi, false);
        assert_eq!(container.fields[4].is_multi, false);
        assert_eq!(container.fields[5].is_multi, true);
    }

    #[test]
    fn test_decode_empty_body() {
        let codec = CmdcCodec {};
        let data = b"<1,18,0,-6,5222,2>[]";
        let containers = codec.decode(data).unwrap();

        assert!(containers.containers.len() == 1);
        let container = &containers.containers[0];
        assert_eq!(container.fields[0].data, b"");
    }

    #[test]
    fn test_zero_len_string_field() {
        let codec = CmdcCodec {};
        let data = b"<1,18,0,-6,5222,2>[1,(0:),3,4]";
        let containers = codec.decode(data).unwrap();

        let container = &containers.containers[0];
        assert_eq!(container.fields[0].data, b"1");
        assert_eq!(container.fields[1].data, b"(0:)");
        assert_eq!(container.fields[2].data, b"3");
        assert_eq!(container.fields[3].data, b"4");
    }

    #[test]
    fn test_empty_string_field() {
        let codec = CmdcCodec {};
        let data = b"<1,18,0,-6,5222,2>[1,(),3,4]";
        let containers = codec.decode(data).unwrap();

        let container = &containers.containers[0];
        assert_eq!(container.fields[0].data, b"1");
        assert_eq!(container.fields[1].data, b"()");
        assert_eq!(container.fields[2].data, b"3");
        assert_eq!(container.fields[3].data, b"4");
    }

    #[test]
    fn test_unicode_string_field() {
        let codec = CmdcCodec {};
        let data = "<1,18,0,-6,5222,2>[1,(6:富爸),3,4]".as_bytes();
        let containers = codec.decode(data).unwrap();

        let container = &containers.containers[0];
        assert_eq!(container.fields[0].data, b"1");
        assert_eq!(container.fields[1].data, "(6:富爸)".as_bytes());
        assert_eq!(container.fields[2].data, b"3");
        assert_eq!(container.fields[3].data, b"4");
    }

    #[test]
    fn test_decode_field_with_reserved_char() {
        let codec = CmdcCodec {};
        let data = b"<1,18,0,-6,5222,2>[1,2,(10:v[<ue(obar),4,,6]";
        let containers = codec.decode(data).unwrap();
        let container = &containers.containers[0];

        assert_eq!(container.fields.len(), 6);
        assert_eq!(container.fields[0].data, b"1");
        assert_eq!(container.fields[1].data, b"2");
        assert_eq!(container.fields[2].data, b"(10:v[<ue(obar)");
        assert_eq!(container.fields[3].data, b"4");
        assert_eq!(container.fields[4].data, b"");
        assert_eq!(container.fields[5].data, b"6");
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
            "Invalid cMDC header, Invalid digit found in '1-6'"
        );
    }

    #[test]
    fn test_invalid_body1() {
        let codec = CmdcCodec {};
        let data = b"<1,18,0,-6,5222,2>[1,20,300,4";
        let err = codec.decode(data).unwrap_err();
        assert_eq!(err.to_string(), "Invalid cMDC body, no end of body");
    }

    #[test]
    fn test_invalid_body2() {
        let codec = CmdcCodec {};
        let data = b"<1,18,0,-6,5222,2>";
        let err = codec.decode(data).unwrap_err();
        assert_eq!(err.to_string(), "Invalid cMDC body, no body");
    }

    #[test]
    fn test_invalid_body3() {
        let codec = CmdcCodec {};
        let data = b"<1,18,0,-6,5222,2>1,2,3]";
        let err = codec.decode(data).unwrap_err();
        assert_eq!(
            err.to_string(),
            "Invalid cMDC body, first character must be '['"
        );
    }

    #[test]
    fn test_invalid_body4() {
        let codec = CmdcCodec {};
        let data = b"<1,18,0,-6,5222,2>[1,(abc:foo),3,4]";
        let err = codec.decode(data).unwrap_err();
        assert_eq!(
            err.to_string(),
            "Invalid character 'a', numeric expected for string length"
        );
    }

    #[test]
    fn test_invalid_body5() {
        let codec = CmdcCodec {};
        let data = b"<1,18,0,-6,5222,2>[1,(5:foo),3,4]";
        let err = codec.decode(data).unwrap_err();
        assert_eq!(err.to_string(), "Invalid cMDC body, mismatch string length");
    }

    #[test]
    fn test_invalid_body6() {
        let codec = CmdcCodec {};
        let data = b"<1,18,0,-6,5222,2>[1,(5:foobar),3,4]";
        let err = codec.decode(data).unwrap_err();
        assert_eq!(err.to_string(), "Invalid cMDC body, mismatch string length");
    }

    #[test]
    fn test_invalid_body7() {
        let codec = CmdcCodec {};
        let data = b"<1,18,0,-6,5222,2>[1,(5:fooba:),3,4]";
        let err = codec.decode(data).unwrap_err();
        assert_eq!(err.to_string(), "Invalid cMDC body, mismatch string length");
    }

    #[test]
    fn test_encode_container() {
        let codec = CmdcCodec {};

        let containers = Containers {
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
                    Field {
                        data: "1".as_bytes().to_vec(),
                        field_type: FieldType::Unknown,
                        value: None,
                        is_multi: false,
                        is_container: false,
                    },
                    Field {
                        data: "20".as_bytes().to_vec(),
                        field_type: FieldType::Unknown,
                        value: None,
                        is_multi: false,
                        is_container: false,
                    },
                    Field {
                        data: "(5:three)".as_bytes().to_vec(),
                        field_type: FieldType::Unknown,
                        value: None,
                        is_multi: false,
                        is_container: false,
                    },
                    Field {
                        data: "400000".as_bytes().to_vec(),
                        field_type: FieldType::Unknown,
                        value: None,
                        is_multi: false,
                        is_container: false,
                    },
                ],
            }],
        };

        let encoded = codec.encode(&containers).unwrap();
        assert_eq!(encoded, b"<1,18,0,-6,5222,2>[1,20,(5:three),400000]");
    }
}
