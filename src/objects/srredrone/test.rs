use crate::{SRREDRONE, test::decode_hex, Parseable};

#[test]
fn test_srredrone_round_trip() {
    let input = decode_hex(include_str!("srredrone.hex"));
    let result = SRREDRONE::parse(&input);
    let (_, value) = result.unwrap();
    let mut output = vec![];
    assert!(value.write(&mut output).is_ok());
    assert_eq!(input, output);
}