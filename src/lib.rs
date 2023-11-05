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

#[cfg(test)]
mod tests {
    use crate::hex_encode;

    #[test]
    fn test_hex_encode() {
        assert_eq!(hex_encode(&[0x55, 0xAA, 0x05, 0xA0]), "55aa05a0");
    }
}
