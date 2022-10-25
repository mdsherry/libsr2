use crate::{SRAD2, test::decode_hex, Parseable};

#[test]
fn test_srad2_round_trip() {
    let input = decode_hex(include_str!("srad2.hex"));
    let result = SRAD2::parse(&input);
    let (_, value) = result.unwrap();
    let mut output = vec![];
    assert!(value.write(&mut output).is_ok());
    assert_eq!(input, output);
}