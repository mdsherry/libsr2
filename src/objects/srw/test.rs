use crate::{SRW, test::decode_hex, Parseable};

#[test]
fn test_srw_round_trip() {
    let input = decode_hex(include_str!("srw.hex"));
    let result = SRW::parse(&input);
    let (_, value) = result.unwrap();
    let mut output = vec![];
    assert!(value.write(&mut output).is_ok());
    assert_eq!(input, output);
}