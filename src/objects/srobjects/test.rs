use crate::objects::{
    srobjects::{
        SRGameIconIndex, SRGameSettings, SRGsumm, SRPg, SRReDrone, SRUpgradeComponents,
        SRZoneIndex, Sriti, Srrcd, Srrgi, Srrw, Srsgi, SRG,
    },
    Parseable,
};

use super::Srv3;

fn decode_hex(s: &str) -> Vec<u8> {
    (0..s.len())
        .step_by(3)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16).unwrap())
        .collect()
}

#[test]
fn test_srv3() {
    let input = decode_hex(include_str!("sample_srv3.hex"));
    let result = Srv3::parse(&input);
    assert_eq!(
        Ok((
            [].as_ref(),
            Srv3 {
                coords: [609.46, 30.829998, 475.56]
            }
        )),
        result
    );
    let (_, srv3) = result.unwrap();
    let mut output = vec![];
    assert!(srv3.write(&mut output).is_ok());
    assert_eq!(input, output);
}

#[test]
fn test_srrw() {
    let input = decode_hex(include_str!("sample_srrw.hex"));
    let result = Srrw::parse(&input);
    
    let (_, value) = result.unwrap();
    let mut output = vec![];
    assert!(value.write(&mut output).is_ok());
    assert_eq!(input, output);
}

#[test]
fn test_sriti() {
    let input = decode_hex(include_str!("sample_sriti.hex"));
    let result = Sriti::parse(&input);
    // assert_eq!(Ok(([].as_ref(), Srrw { a: [0x00, 0x00, 0x00, 0xf0], b: 6.57062, c: 0. })), result);
    let (_, value) = result.unwrap();
    let mut output = vec![];
    assert!(value.write(&mut output).is_ok());
    assert_eq!(input, output);
}

#[test]
fn test_srrgi() {
    let input = decode_hex(include_str!("sample_srrgi.hex"));
    let result = Srrgi::parse(&input);
    // assert_eq!(Ok(([].as_ref(), Srrw { a: [0x00, 0x00, 0x00, 0xf0], b: 6.57062, c: 0. })), result);
    let (_, value) = result.unwrap();
    let mut output = vec![];
    assert!(value.write(&mut output).is_ok());
    assert_eq!(input, output);
}

#[test]
fn test_srsgi() {
    let input = decode_hex(include_str!("sample_srsgi.hex"));
    let result = Srsgi::parse(&input);
    // assert_eq!(Ok(([].as_ref(), Srrw { a: [0x00, 0x00, 0x00, 0xf0], b: 6.57062, c: 0. })), result);
    let (_, value) = result.unwrap();
    let mut output = vec![];
    assert!(value.write(&mut output).is_ok());
    assert_eq!(input, output);
}

#[test]
fn test_srgameiconindex() {
    let input = decode_hex(include_str!("sample_srgameiconindex.hex"));
    let result = SRGameIconIndex::parse(&input);
    // assert_eq!(Ok(([].as_ref(), Srrw { a: [0x00, 0x00, 0x00, 0xf0], b: 6.57062, c: 0. })), result);
    let (_, value) = result.unwrap();
    let mut output = vec![];
    assert!(value.write(&mut output).is_ok());
    assert_eq!(input, output);
}

#[test]
fn test_srgzoneindex() {
    let input = decode_hex(include_str!("sample_srgzoneindex.hex"));
    let result = SRZoneIndex::parse(&input);
    // assert_eq!(Ok(([].as_ref(), Srrw { a: [0x00, 0x00, 0x00, 0xf0], b: 6.57062, c: 0. })), result);
    let (_, value) = result.unwrap();
    let mut output = vec![];
    assert!(value.write(&mut output).is_ok());
    assert_eq!(input, output);
}

#[test]
fn test_srgsumm() {
    let input = decode_hex(include_str!("sample_srgsumm.hex"));
    let result = SRGsumm::parse(&input);
    // assert_eq!(Ok(([].as_ref(), Srrw { a: [0x00, 0x00, 0x00, 0xf0], b: 6.57062, c: 0. })), result);
    let (_, value) = result.unwrap();
    let mut output = vec![];
    assert!(value.write(&mut output).is_ok());
    assert_eq!(input, output);
}

#[test]
fn test_srgamesettings() {
    let input = decode_hex(include_str!("sample_srgamesettings.hex"));
    let result = SRGameSettings::parse(&input);
    // assert_eq!(Ok(([].as_ref(), Srrw { a: [0x00, 0x00, 0x00, 0xf0], b: 6.57062, c: 0. })), result);
    let (_, value) = result.unwrap();
    let mut output = vec![];
    assert!(value.write(&mut output).is_ok());
    assert_eq!(input, output);
}

#[test]
fn test_srrcd() {
    let input = decode_hex(include_str!("sample_srrcd.hex"));
    let result = Srrcd::parse(&input);
    // assert_eq!(Ok(([].as_ref(), Srrw { a: [0x00, 0x00, 0x00, 0xf0], b: 6.57062, c: 0. })), result);
    let (_, value) = result.unwrap();
    let mut output = vec![];
    assert!(value.write(&mut output).is_ok());
    assert_eq!(input, output);
}

#[test]
fn test_srg() {
    let input = decode_hex(include_str!("sample_srg.hex"));
    let result = SRG::parse(&input);
    // assert_eq!(Ok(([].as_ref(), Srrw { a: [0x00, 0x00, 0x00, 0xf0], b: 6.57062, c: 0. })), result);
    let (_, value) = result.unwrap();
    let mut output = vec![];
    assert!(value.write(&mut output).is_ok());
    assert_eq!(input, output);
}

#[test]
fn test_srpg() {
    let input = decode_hex(include_str!("sample_srpg.hex"));
    let result = SRPg::parse(&input);
    // assert_eq!(Ok(([].as_ref(), Srrw { a: [0x00, 0x00, 0x00, 0xf0], b: 6.57062, c: 0. })), result);
    let (_, value) = result.unwrap();
    let mut output = vec![];
    assert!(value.write(&mut output).is_ok());
    assert_eq!(input, output);
}

#[test]
fn test_srredrone() {
    let input = decode_hex(include_str!("sample_srredrone.hex"));
    let result = SRReDrone::parse(&input);
    // assert_eq!(Ok(([].as_ref(), Srrw { a: [0x00, 0x00, 0x00, 0xf0], b: 6.57062, c: 0. })), result);
    let (_, value) = result.unwrap();
    let mut output = vec![];
    assert!(value.write(&mut output).is_ok());
    assert_eq!(input, output);
}

#[test]
fn test_srupgradecomponents() {
    let input = decode_hex(include_str!("sample_srupgradecomponents.hex"));
    let result = SRUpgradeComponents::parse(&input);
    // assert_eq!(Ok(([].as_ref(), Srrw { a: [0x00, 0x00, 0x00, 0xf0], b: 6.57062, c: 0. })), result);
    let (_, value) = result.unwrap();
    let mut output = vec![];
    assert!(value.write(&mut output).is_ok());
    assert_eq!(input, output);
}
