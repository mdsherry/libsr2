use crate::{SRRANCH, test::decode_hex, Parseable};

#[test]
fn test_srranch_round_trip() {
    let input = decode_hex(include_str!("srranch.hex"));
    let result = SRRANCH::parse(&input);
    let (_, value) = result.unwrap();
    let mut output = vec![];
    assert!(value.write(&mut output).is_ok());
    assert_eq!(input, output);
}