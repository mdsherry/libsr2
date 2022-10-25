use crate::{SRV3, test::decode_hex, Parseable};

#[test]
fn test_srv3_round_trip() {
    let input = decode_hex(include_str!("srv3.hex"));
    let result = SRV3::parse(&input);
    let (_, value) = result.unwrap();
    let mut output = vec![];
    assert!(value.write(&mut output).is_ok());
    assert_eq!(input, output);
}