use crate::{StringPair, test::decode_hex, Parseable, };

#[test]
fn test_stringpair_round_trip() {
    let input = decode_hex(include_str!("stringpair.hex"));
    let result = StringPair::parse(&input);
    let (_, value) = result.unwrap();
    let mut output = vec![];
    assert!(value.write(&mut output).is_ok());
    assert_eq!(input, output);
}