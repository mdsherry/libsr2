use crate::{SRSED, test::decode_hex, Parseable};

#[test]
fn test_srsed_round_trip() {
    let input = decode_hex(include_str!("srsed.hex"));
    let result = SRSED::parse(&input);
    let (_, value) = result.unwrap();
    let mut output = vec![];
    assert!(value.write(&mut output).is_ok());
    assert_eq!(input, output);
}