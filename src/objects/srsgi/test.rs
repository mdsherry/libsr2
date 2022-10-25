use crate::{SRSGI, test::decode_hex, Parseable};

#[test]
fn test_srsgi_round_trip() {
    let input = decode_hex(include_str!("srsgi.hex"));
    let result = SRSGI::parse(&input);
    let (_, value) = result.unwrap();
    let mut output = vec![];
    assert!(value.write(&mut output).is_ok());
    assert_eq!(input, output);
}