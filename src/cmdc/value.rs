use crate::cmdc::CmdcCodec;
use crate::cmdc::Containers;
use crate::codec::Codec;
use core::str::from_utf8;
use std::error::Error;

impl CmdcCodec {
    pub fn decode_struct<'a>(&self, data: &'a [u8]) -> Result<Containers<'a>, Box<dyn Error>> {
        Ok(self.decode(data)?)
    }

    pub fn encode_struct(&self, containers: &Containers) -> Result<Vec<u8>, Box<dyn Error>> {
        Ok(self.encode(containers)?)
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

    pub fn encode_string(&self, s: &str) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut data = Vec::new();
        data.push(b'(');
        data.extend_from_slice(&s.len().to_string().into_bytes());
        data.push(b':');
        data.extend_from_slice(s.as_bytes());
        data.push(b')');
        Ok(data)
    }

    pub fn decode_int8(&self, data: &[u8]) -> Result<i8, Box<dyn Error>> {
        let s = from_utf8(data)?;
        Ok(s.parse::<i8>()?)
    }

    pub fn encode_int8(&self, v: i8) -> Result<Vec<u8>, Box<dyn Error>> {
        Ok(v.to_string().into_bytes())
    }

    pub fn decode_int16(&self, data: &[u8]) -> Result<i16, Box<dyn Error>> {
        let s = from_utf8(data)?;
        Ok(s.parse::<i16>()?)
    }

    pub fn encode_int16(&self, v: i16) -> Result<Vec<u8>, Box<dyn Error>> {
        Ok(v.to_string().into_bytes())
    }

    pub fn decode_int32(&self, data: &[u8]) -> Result<i32, Box<dyn Error>> {
        let s = from_utf8(data)?;
        Ok(s.parse::<i32>()?)
    }

    pub fn encode_int32(&self, v: i32) -> Result<Vec<u8>, Box<dyn Error>> {
        Ok(v.to_string().into_bytes())
    }

    pub fn decode_int64(&self, data: &[u8]) -> Result<i64, Box<dyn Error>> {
        let s = from_utf8(data)?;
        Ok(s.parse::<i64>()?)
    }

    pub fn encode_int64(&self, v: i64) -> Result<Vec<u8>, Box<dyn Error>> {
        Ok(v.to_string().into_bytes())
    }

    pub fn decode_uint8(&self, data: &[u8]) -> Result<u8, Box<dyn Error>> {
        let s = from_utf8(data)?;
        Ok(s.parse::<u8>()?)
    }

    pub fn encode_uint8(&self, v: u8) -> Result<Vec<u8>, Box<dyn Error>> {
        Ok(v.to_string().into_bytes())
    }

    pub fn decode_uint16(&self, data: &[u8]) -> Result<u16, Box<dyn Error>> {
        let s = from_utf8(data)?;
        Ok(s.parse::<u16>()?)
    }

    pub fn encode_uint16(&self, v: u16) -> Result<Vec<u8>, Box<dyn Error>> {
        Ok(v.to_string().into_bytes())
    }

    pub fn decode_uint32(&self, data: &[u8]) -> Result<u32, Box<dyn Error>> {
        let s = from_utf8(data)?;
        Ok(s.parse::<u32>()?)
    }

    pub fn encode_uint32(&self, v: u32) -> Result<Vec<u8>, Box<dyn Error>> {
        Ok(v.to_string().into_bytes())
    }

    pub fn decode_uint64(&self, data: &[u8]) -> Result<u64, Box<dyn Error>> {
        let s = from_utf8(data)?;
        Ok(s.parse::<u64>()?)
    }

    pub fn encode_uint64(&self, v: u64) -> Result<Vec<u8>, Box<dyn Error>> {
        Ok(v.to_string().into_bytes())
    }
}

#[cfg(test)]
mod tests {
    // use super::*;
    use crate::cmdc::CMDC_CODEC;

    #[test]
    fn test_encode_decode_struct() {
        let data = b"<1,2,0,452,5222,2>[100,2,]";
        let containers = CMDC_CODEC.decode_struct(data).unwrap();
        assert_eq!(containers.containers.len(), 1);
        let container = &containers.containers[0];
        assert_eq!(container.fields.len(), 3);
        assert_eq!(container.fields[0].data, b"100");
        assert_eq!(container.fields[1].data, b"2");
        assert_eq!(container.fields[2].data, b"");

        let encoded = CMDC_CODEC.encode_struct(&containers).unwrap();
        assert_eq!(encoded, data);
    }

    #[test]
    fn test_encode_decode_string() {
        let data = b"(5:three)";
        let s = CMDC_CODEC.decode_string(data).unwrap();
        assert_eq!(s, "three");
        let encoded = CMDC_CODEC.encode_string(s).unwrap();
        assert_eq!(encoded, b"(5:three)");
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

    #[test]
    fn test_encode_decode_int8() {
        let data = b"-125";
        let v = CMDC_CODEC.decode_int8(data).unwrap();
        assert_eq!(v, -125);
        let data = CMDC_CODEC.encode_int8(v).unwrap();
        assert_eq!(data, b"-125");
    }

    #[test]
    fn test_encode_decode_int16() {
        let data = b"1070";
        let v = CMDC_CODEC.decode_int16(data).unwrap();
        assert_eq!(v, 1070);
        let data = CMDC_CODEC.encode_int16(v).unwrap();
        assert_eq!(data, b"1070");
    }

    #[test]
    fn test_encode_decode_int32() {
        let data = b"-107";
        let v = CMDC_CODEC.decode_int32(data).unwrap();
        assert_eq!(v, -107);
        let data = CMDC_CODEC.encode_int32(v).unwrap();
        assert_eq!(data, b"-107");
    }

    #[test]
    fn test_encode_decode_int64() {
        let data = b"81345123666616";
        let v = CMDC_CODEC.decode_int64(data).unwrap();
        assert_eq!(v, 81345123666616);
        let data = CMDC_CODEC.encode_int64(v).unwrap();
        assert_eq!(data, b"81345123666616");
    }

    #[test]
    fn test_encode_decode_uint8() {
        let data = b"250";
        let v = CMDC_CODEC.decode_uint8(data).unwrap();
        assert_eq!(v, 250);
        let data = CMDC_CODEC.encode_uint8(v).unwrap();
        assert_eq!(data, b"250");
    }

    #[test]
    fn test_encode_decode_uint16() {
        let data = b"8000";
        let v = CMDC_CODEC.decode_uint16(data).unwrap();
        assert_eq!(v, 8000);
        let data = CMDC_CODEC.encode_uint16(v).unwrap();
        assert_eq!(data, b"8000");
    }

    #[test]
    fn test_encode_decode_uint32() {
        let data = b"8000000";
        let v = CMDC_CODEC.decode_uint32(data).unwrap();
        assert_eq!(v, 8000000);
        let data = CMDC_CODEC.encode_uint32(v).unwrap();
        assert_eq!(data, b"8000000");
    }

    #[test]
    fn test_encode_decode_uint64() {
        let data = b"8000000000000000000";
        let v = CMDC_CODEC.decode_uint64(data).unwrap();
        assert_eq!(v, 8000000000000000000);
        let encoded = CMDC_CODEC.encode_uint64(v).unwrap();
        assert_eq!(encoded, data);
    }
}
