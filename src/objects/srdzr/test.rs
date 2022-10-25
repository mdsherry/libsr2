use crate::{SRDZR, test::decode_hex, Parseable};

#[test]
fn test_srdzr_round_trip() {
    let input = decode_hex(include_str!("srdzr.hex"));
    let result = SRDZR::parse(&input);
    let (_, value) = result.unwrap();
    let mut output = vec![];
    assert!(value.write(&mut output).is_ok());
    assert_eq!(input, output);
}