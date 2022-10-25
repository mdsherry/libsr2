use crate::{SRUPGRADECOMPONENTS, test::decode_hex, Parseable};

#[test]
fn test_srupgradecomponents_round_trip() {
    let input = decode_hex(include_str!("srupgradecomponents.hex"));
    let result = SRUPGRADECOMPONENTS::parse(&input);
    let (_, value) = result.unwrap();
    let mut output = vec![];
    assert!(value.write(&mut output).is_ok());
    assert_eq!(input, output);
}