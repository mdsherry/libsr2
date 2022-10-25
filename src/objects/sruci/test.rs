use crate::{SRUCI, test::decode_hex, Parseable};

#[test]
fn test_sruci_round_trip() {
    let input = decode_hex(include_str!("sruci.hex"));
    let result = SRUCI::parse(&input);
    let (_, value) = result.unwrap();
    let mut output = vec![];
    assert!(value.write(&mut output).is_ok());
    assert_eq!(input, output);
}