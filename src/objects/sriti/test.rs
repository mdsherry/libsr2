use crate::{SRITI, test::decode_hex, Parseable};

#[test]
fn test_sriti_round_trip() {
    let input = decode_hex(include_str!("sriti.hex"));
    let result = SRITI::parse(&input);
    let (_, value) = result.unwrap();
    let mut output = vec![];
    assert!(value.write(&mut output).is_ok());
    assert_eq!(input, output);
}