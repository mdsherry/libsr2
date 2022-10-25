use crate::{SRPL, test::decode_hex, Parseable};

#[test]
fn test_srpl_round_trip() {
    let input = decode_hex(include_str!("srpl.hex"));
    let result = SRPL::parse(&input);
    let (_, value) = result.unwrap();
    let mut output = vec![];
    assert!(value.write(&mut output).is_ok());
    assert_eq!(input, output);
}