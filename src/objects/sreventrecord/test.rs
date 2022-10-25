use crate::{SREVENTRECORD, test::decode_hex, Parseable};

#[test]
fn test_sreventrecord_round_trip() {
    let input = decode_hex(include_str!("sreventrecord.hex"));
    let result = SREVENTRECORD::parse(&input);
    let (_, value) = result.unwrap();
    let mut output = vec![];
    assert!(value.write(&mut output).is_ok());
    assert_eq!(input, output);
}