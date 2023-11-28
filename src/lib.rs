use std::io::{Error, ErrorKind};

const HEX_TABLE: [char; 16] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f'];

pub fn hex_encode(data: &[u8]) -> String {
    let mut s = String::new();
    for b in data {
        let b1 = (*b >> 4) as usize;
        let b2 = (*b & 0x0F) as usize;
        s.push(HEX_TABLE[b1]);
        s.push(HEX_TABLE[b2]);
    }
    s
}

pub fn hex_decode(data: &String) -> Result<Vec<u8>, Error> {
    let mut result = Vec::new();
    let l = data.len();
    if l == 0 {
        return Ok(result);
    }
    if (l & 1) != 0 {
        return Err(Error::new(ErrorKind::InvalidInput, "invalid source string length"));
    }
    let chars: Vec<char> = data.chars().collect();
    for i in (0..l).step_by(2) {
        let mut b = convert(chars[i])? << 4;
        b |= convert(chars[i+1])?;
        result.push(b);
    }
    Ok(result)
}

fn convert(data: char) -> Result<u8, Error> {
    if data >= '0' && data <= '9' {
        return Ok(data as u8 - '0' as u8)
    }
    if data >= 'a' && data <= 'f' {
        return Ok(data as u8 - 'a' as u8 + 10)
    }
    if data >= 'A' && data <= 'F' {
        return Ok(data as u8 - 'A' as u8 + 10)
    }
    Err(Error::new(ErrorKind::InvalidInput, "non-hex symbol"))
}

#[cfg(test)]
mod tests {
    use std::io::Error;
    use crate::{hex_encode, hex_decode};

    #[test]
    fn test_hex_encode() {
        assert_eq!(hex_encode(&[0x55, 0xAA, 0x05, 0xA0]), "55aa05a0");
    }

    #[test]
    fn test_hex_decode() -> Result<(), Error> {
        assert_eq!(hex_decode(&"55aa05a0".to_string())?, &[0x55, 0xAA, 0x05, 0xA0]);
        assert_eq!(hex_decode(&"55AA05A0".to_string())?, &[0x55, 0xAA, 0x05, 0xA0]);
        assert!(hex_decode(&"55AA05Aw".to_string()).is_err());
        assert!(hex_decode(&"55AA05A".to_string()).is_err());
        Ok(())
    }
}
