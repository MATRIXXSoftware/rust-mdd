use crate::cmdc::CmdcCodec;
use crate::cmdc::Containers;
use core::str::from_utf8;
use std::error::Error;

impl CmdcCodec {
    pub fn decode_struct<'a>(&self, data: &'a [u8]) -> Result<Containers<'a>, Box<dyn Error>> {
        self.decode_containers(data)
    }

    pub fn decode_string<'a>(&self, data: &'a [u8]) -> Result<&'a str, Box<dyn Error>> {
        if data.is_empty() {
            return Ok("");
        }
        if data[0] != b'(' {
            return Err("Invalid string value".into());
        }

        for (idx, &c) in data.iter().enumerate().skip(1) {
            if c == b':' {
                let temp = &data[1..idx];
                let len = Self::bytes_to_int(temp)? as usize;
                if idx + 1 + len > data.len() {
                    return Err(format!("Invalid string length, {} is too long", len).into());
                }
                if data[idx + 1 + len] != b')' {
                    return Err(format!("Invalid string length, {} is too short", len).into());
                }
                let str = from_utf8(&data[idx + 1..idx + 1 + len])?;
                return Ok(str);
            }
        }

        Err("Invalid string value".into())
    }

    pub fn decode_int8(&self, data: &[u8]) -> Result<i8, Box<dyn Error>> {
        let s = from_utf8(data)?;
        Ok(s.parse::<i8>()?)
    }

    pub fn decode_int16(&self, data: &[u8]) -> Result<i16, Box<dyn Error>> {
        let s = from_utf8(data)?;
        Ok(s.parse::<i16>()?)
    }

    pub fn decode_int32(&self, data: &[u8]) -> Result<i32, Box<dyn Error>> {
        let s = from_utf8(data)?;
        Ok(s.parse::<i32>()?)
    }

    pub fn decode_int64(&self, data: &[u8]) -> Result<i64, Box<dyn Error>> {
        let s = from_utf8(data)?;
        Ok(s.parse::<i64>()?)
    }

    pub fn decode_uint8(&self, data: &[u8]) -> Result<u8, Box<dyn Error>> {
        let s = from_utf8(data)?;
        Ok(s.parse::<u8>()?)
    }

    pub fn decode_uint16(&self, data: &[u8]) -> Result<u16, Box<dyn Error>> {
        let s = from_utf8(data)?;
        Ok(s.parse::<u16>()?)
    }

    pub fn decode_uint32(&self, data: &[u8]) -> Result<u32, Box<dyn Error>> {
        let s = from_utf8(data)?;
        Ok(s.parse::<u32>()?)
    }

    pub fn decode_uint64(&self, data: &[u8]) -> Result<u64, Box<dyn Error>> {
        let s = from_utf8(data)?;
        Ok(s.parse::<u64>()?)
    }
}

#[cfg(test)]
mod tests {
    // use super::*;
    use crate::cmdc::CMDC_CODEC;

    #[test]
    fn test_decode_valid_string() {
        let data = b"(5:three)";
        let s = CMDC_CODEC.decode_string(data).unwrap();
        assert_eq!(s, "three");
    }

    #[test]
    fn test_decode_invalid_string_1() {
        let data = b"(15:three)";
        let err = CMDC_CODEC.decode_string(data).err().unwrap().to_string();
        assert_eq!(err, "Invalid string length, 15 is too long");
    }

    #[test]
    fn test_decode_invalid_string_2() {
        let data = b"(3:three)";
        let err = CMDC_CODEC.decode_string(data).err().unwrap().to_string();
        assert_eq!(err, "Invalid string length, 3 is too short");
    }
}
