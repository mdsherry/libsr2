use crate::{SRPU, test::decode_hex, Parseable};

#[test]
fn test_srpu_round_trip() {
    let input = decode_hex(include_str!("srpu.hex"));
    let result = SRPU::parse(&input);
    let (_, value) = result.unwrap();
    let mut output = vec![];
    assert!(value.write(&mut output).is_ok());
    assert_eq!(input, output);
}