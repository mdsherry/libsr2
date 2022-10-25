use crate::{SRAPP, test::decode_hex, Parseable};

#[test]
fn test_srapp_round_trip() {
    let input = decode_hex(include_str!("srapp.hex"));
    let result = SRAPP::parse(&input);
    let (_, value) = result.unwrap();
    let mut output = vec![];
    assert!(value.write(&mut output).is_ok());
    assert_eq!(input, output);
}