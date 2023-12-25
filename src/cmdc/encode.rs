use super::CmdcCodec;
use crate::mdd::Container;
use crate::mdd::Containers;
use crate::mdd::Field;
use crate::mdd::Header;
use std::error::Error;
use std::io::Write;

impl CmdcCodec {
    pub fn encode_containers(&self, containers: &Containers) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut data = Vec::new();

        for container in &containers.containers {
            let container_data = self.encode_container(container)?;
            data.extend(container_data);
        }

        Ok(data)
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
            data.extend(field.data);
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
                    Field::raw("1".as_bytes()),
                    Field::raw("20".as_bytes()),
                    Field::raw("(5:three)".as_bytes()),
                    Field::raw("400000".as_bytes()),
                ],
            }],
        };

        let encoded = codec.encode_containers(&containers).unwrap();
        assert_eq!(encoded, b"<1,18,0,-6,5222,2>[1,20,(5:three),400000]");
    }
}
