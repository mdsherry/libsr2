use crate::{SRSE, test::decode_hex, Parseable};

#[test]
fn test_srse_round_trip() {
    let input = decode_hex(include_str!("srse.hex"));
    let result = SRSE::parse(&input);
    let (_, value) = result.unwrap();
    let mut output = vec![];
    assert!(value.write(&mut output).is_ok());
    assert_eq!(input, output);
}