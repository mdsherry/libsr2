use crate::{SRGZONEINDEX, test::decode_hex, Parseable};

#[test]
fn test_srgzoneindex_round_trip() {
    let input = decode_hex(include_str!("srgzoneindex.hex"));
    let result = SRGZONEINDEX::parse(&input);
    let (_, value) = result.unwrap();
    let mut output = vec![];
    assert!(value.write(&mut output).is_ok());
    assert_eq!(input, output);
}