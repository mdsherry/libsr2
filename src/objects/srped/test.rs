use crate::{SRPED, test::decode_hex, Parseable};

#[test]
fn test_srped_round_trip() {
    let input = decode_hex(include_str!("srped.hex"));
    let result = SRPED::parse(&input);
    let (_, value) = result.unwrap();
    let mut output = vec![];
    assert!(value.write(&mut output).is_ok());
    assert_eq!(input, output);
}