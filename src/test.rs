pub fn decode_hex(s: &str) -> Vec<u8> {
    (0..s.len())
        .step_by(3)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16).unwrap())
        .collect()
}
