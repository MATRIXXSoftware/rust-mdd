use super::CmdcCodec;
use crate::mdd::Container;
use crate::mdd::Containers;
use crate::mdd::Field;
use crate::mdd::Header;
use std::error::Error;
use std::io::Write;

impl CmdcCodec {
    pub fn encode_containers<W: Write>(
        &self,
        buffer: &mut W,
        containers: &Containers,
    ) -> Result<(), Box<dyn Error>> {
        for container in &containers.containers {
            self.encode_container(buffer, container)?;
        }

        Ok(())
    }

    pub fn get_containers_len(&self, containers: &Containers) -> usize {
        let mut len = 0;
        for container in &containers.containers {
            len += self.get_container_len(container);
        }
        len
    }

    fn encode_container<W: Write>(
        &self,
        buffer: &mut W,
        container: &Container,
    ) -> Result<(), Box<dyn Error>> {
        self.encode_header(buffer, &container.header)?;
        self.encode_body(buffer, &container.fields)?;

        Ok(())
    }

    #[inline]
    fn get_container_len(&self, container: &Container) -> usize {
        let mut len = self.get_header_len(&container.header);
        len += self.get_body_len(&container.fields);

        len
    }

    fn encode_header<W: Write>(
        &self,
        buffer: &mut W,
        header: &Header,
    ) -> Result<(), Box<dyn Error>> {
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

    #[inline]
    fn get_header_len(&self, _header: &Header) -> usize {
        return 4 + 1 + 1 + 7 + 4 + 3 + 6 + 2;
    }

    fn encode_body<W: Write>(
        &self,
        buffer: &mut W,
        fields: &[Field],
    ) -> Result<(), Box<dyn Error>> {
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

    #[inline]
    fn get_body_len(&self, fields: &[Field]) -> usize {
        let mut len = 2;
        for field in fields.iter() {
            len += field.data.len() + 1;
        }
        len
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cmdc::CMDC_CODEC;
    use std::io::BufWriter;

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

        let mut buffer = BufWriter::new(Vec::new());
        CMDC_CODEC
            .encode_containers(&mut buffer, &containers)
            .unwrap();

        let encoded = buffer.into_inner().unwrap();
        assert_eq!(encoded, b"<1,18,0,-6,5222,2>[1,20,(5:three),400000]");
    }
}
