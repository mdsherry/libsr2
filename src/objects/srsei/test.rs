use crate::{SRSEI, test::decode_hex, Parseable};

#[test]
fn test_srsei_round_trip() {
    let input = decode_hex(include_str!("srsei.hex"));
    let result = SRSEI::parse(&input);
    let (_, value) = result.unwrap();
    let mut output = vec![];
    assert!(value.write(&mut output).is_ok());
    assert_eq!(input, output);
}