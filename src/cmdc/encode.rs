use super::CmdcCodec;
use crate::mdd::Container;
use crate::mdd::Containers;
use crate::mdd::Field;
use crate::mdd::Header;
use std::error::Error;
use std::io::Cursor;
use std::io::Write;

impl CmdcCodec {
    pub fn encode_containers(
        &self,
        containers: &Containers,
        buffer: &mut Cursor<Vec<u8>>,
    ) -> Result<(), Box<dyn Error>> {
        for container in &containers.containers {
            self.encode_container(container, buffer)?;
        }

        Ok(())
    }

    fn encode_container(
        &self,
        container: &Container,
        buffer: &mut Cursor<Vec<u8>>,
    ) -> Result<(), Box<dyn Error>> {
        // Encode Header
        self.encode_header(&container.header, buffer)?;
        // Encode Body
        self.encode_body(&container.fields, buffer)?;

        Ok(())
    }

    fn encode_header(
        &self,
        header: &Header,
        buffer: &mut Cursor<Vec<u8>>,
    ) -> Result<(), Box<dyn Error>> {
        // Predefine the capacity of the vector to avoid reallocation
        // let estimated_size = 4 + 1 + 1 + 7 + 4 + 3 + 6 + 2;
        // let mut data = Vec::with_capacity(estimated_size);

        write!(
            buffer,
            "<{},{},{},{},{},{}>",
            header.version,
            header.total_field,
            header.depth,
            header.key,
            header.schema_version,
            header.ext_version
        )?;

        Ok(())
    }

    fn encode_body(
        &self,
        fields: &[Field],
        buffer: &mut Cursor<Vec<u8>>,
    ) -> Result<(), Box<dyn Error>> {
        // Predefine the capacity of the vector to avoid reallocation
        // let mut estimated_len = fields.len() + 2;
        // for field in fields.iter() {
        //     estimated_len += field.data.len();
        // }
        // let mut data = Vec::with_capacity(estimated_len);

        buffer.write_all(b"[")?;
        for (i, field) in fields.iter().enumerate() {
            if i > 0 {
                buffer.write_all(b",")?;
            }
            buffer.write_all(field.data)?;
        }

        buffer.write_all(b"]")?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cmdc::CMDC_CODEC;

    #[test]
    fn test_encode_container() {
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

        let mut buffer = Cursor::new(Vec::new());
        CMDC_CODEC
            .encode_containers(&containers, &mut buffer)
            .unwrap();

        let encoded = buffer.into_inner();
        assert_eq!(encoded, b"<1,18,0,-6,5222,2>[1,20,(5:three),400000]");
    }
}
