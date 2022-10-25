use crate::{SRRESNODE, test::decode_hex, Parseable};

#[test]
fn test_srresnode_round_trip() {
    let input = decode_hex(include_str!("srresnode.hex"));
    let result = SRRESNODE::parse(&input);
    let (_, value) = result.unwrap();
    let mut output = vec![];
    assert!(value.write(&mut output).is_ok());
    assert_eq!(input, output);
}