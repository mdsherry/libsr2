use crate::{SRGSUMM, test::decode_hex, Parseable};

#[test]
fn test_srgsumm_round_trip() {
    let input = decode_hex(include_str!("srgsumm.hex"));
    let result = SRGSUMM::parse(&input);
    let (_, value) = result.unwrap();
    let mut output = vec![];
    assert!(value.write(&mut output).is_ok());
    assert_eq!(input, output);
}