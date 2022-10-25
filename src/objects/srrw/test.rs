use crate::{SRRW, test::decode_hex, Parseable};

#[test]
fn test_srrw_round_trip() {
    let input = decode_hex(include_str!("srrw.hex"));
    let result = SRRW::parse(&input);
    let (_, value) = result.unwrap();
    let mut output = vec![];
    assert!(value.write(&mut output).is_ok());
    assert_eq!(input, output);
}