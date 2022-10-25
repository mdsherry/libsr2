use crate::{SRSECRETSTYLEDISC, test::decode_hex, Parseable};

#[test]
fn test_srsecretstyledisc_round_trip() {
    let input = decode_hex(include_str!("srsecretstyledisc.hex"));
    let result = SRSECRETSTYLEDISC::parse(&input);
    let (_, value) = result.unwrap();
    let mut output = vec![];
    assert!(value.write(&mut output).is_ok());
    assert_eq!(input, output);
}