use crate::{SRRGI, test::decode_hex, Parseable};

#[test]
fn test_srrgi_round_trip() {
    let input = decode_hex(include_str!("srrgi.hex"));
    let result = SRRGI::parse(&input);
    let (_, value) = result.unwrap();
    let mut output = vec![];
    assert!(value.write(&mut output).is_ok());
    assert_eq!(input, output);
}