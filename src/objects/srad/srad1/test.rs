use crate::{SRAD1, test::decode_hex, Parseable};

#[test]
fn test_srad1_round_trip() {
    let input = decode_hex(include_str!("srad1.hex"));
    let result = SRAD1::parse(&input);
    let (_, value) = result.unwrap();
    let mut output = vec![];
    assert!(value.write(&mut output).is_ok());
    assert_eq!(input, output);
}