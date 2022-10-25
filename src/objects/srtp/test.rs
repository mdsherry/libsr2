use crate::{SRTP, test::decode_hex, Parseable};

#[test]
fn test_srtp_round_trip() {
    let input = decode_hex(include_str!("srtp.hex"));
    let result = SRTP::parse(&input);
    let (_, value) = result.unwrap();
    let mut output = vec![];
    assert!(value.write(&mut output).is_ok());
    assert_eq!(input, output);
}