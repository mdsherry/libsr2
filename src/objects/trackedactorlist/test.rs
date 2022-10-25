use crate::{TRACKEDACTORLIST, test::decode_hex, Parseable};

#[test]
fn test_trackedactorlist_round_trip() {
    let input = decode_hex(include_str!("trackedactorlist.hex"));
    let result = TRACKEDACTORLIST::parse(&input);
    let (_, value) = result.unwrap();
    let mut output = vec![];
    assert!(value.write(&mut output).is_ok());
    assert_eq!(input, output);
}