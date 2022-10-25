use crate::{SRLP, test::decode_hex, Parseable};

#[test]
fn test_srlp_round_trip() {
    let input = decode_hex(include_str!("srlp.hex"));
    let result = SRLP::parse(&input);
    let (_, value) = result.unwrap();
    let mut output = vec![];
    assert!(value.write(&mut output).is_ok());
    assert_eq!(input, output);
}