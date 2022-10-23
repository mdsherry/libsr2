mod macros;
use crate::parsers::Parseable;
pub(crate) use macros::*;
use nom::{error::ContextError, IResult};

use std::{fmt::Debug, hash::Hash, io::Write};
pub(crate) trait Obj: Sized {
    const NAME: &'static str;
    const VERSION: i32 = 1;
    fn parse_prefix(input: &[u8]) -> IResult<&[u8], ()> {
        let (input, _) = nom::bytes::complete::tag([Self::NAME.len() as u8])(input)?;
        let (input, _) = nom::bytes::complete::tag(Self::NAME.as_bytes())(input)?;
        let (input, _) = nom::number::complete::le_i32(input)?;
        let (input, _) = nom::bytes::complete::tag([0x00, 0x30])(input)?;
        Ok((input, ()))
    }
    fn parse_suffix(input: &[u8]) -> IResult<&[u8], ()> {
        let (input, _) = nom::bytes::complete::tag([0x00, 0x40])(input)?;
        Ok((input, ()))
    }
    fn parse_body(input: &[u8]) -> IResult<&[u8], Self>;

    fn write_prefix<W: Write>(&self, f: &mut W) -> std::io::Result<()> {
        f.write_all(&[Self::NAME.len() as u8])?;
        f.write_all(Self::NAME.as_bytes())?;
        f.write_all(&Self::VERSION.to_le_bytes())?;
        f.write_all(&[0x00, 0x30])?;
        Ok(())
    }
    fn write_suffix<W: Write>(&self, f: &mut W) -> std::io::Result<()> {
        f.write_all(&[0x00, 0x40])
    }

    fn write_body<W: Write>(&self, f: &mut W) -> std::io::Result<()>;
}

impl<O> Parseable for O
where
    O: Obj + Debug,
{
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let (input, _) = Self::parse_prefix(input)?;
        let (input, rv) = Self::parse_body(input)
            .map_err(|e| e.map(|e| ContextError::add_context(input, Self::NAME, e)))?;

        let (input, _) = Self::parse_suffix(input)?;
        Ok((input, rv))
    }

    fn write<W: Write>(&self, f: &mut W) -> std::io::Result<()> {
        self.write_prefix(f)?;
        self.write_body(f)?;
        self.write_suffix(f)
    }
}

mod srv3;
pub use srv3::*;

mod srsgi;
pub use srsgi::*;

mod srrw;
pub use srrw::*;
mod sriti;
pub use sriti::*;
mod srrgi;
pub use srrgi::*;

mod srgameiconindex;
pub use srgameiconindex::*;

mod srzoneindex;
pub use srzoneindex::*;

mod srgsumm;
pub use srgsumm::*;

mod stringpair;
pub use stringpair::*;

mod srupgradecomponents;
pub use srupgradecomponents::*;

mod srg;
pub use srg::*;

mod srgamesettings;
pub use srgamesettings::*;

mod srpg;
pub use srpg::*;

mod srrcd;
pub use srrcd::*;

mod srredrone;
pub use srredrone::*;

mod srtp;
pub use srtp::*;

mod srw;
pub use srw::*;

mod srlp;
pub use srlp::*;

mod srpl;
pub use srpl::*;

mod srpu;
pub use srpu::*;

mod srdzr;
pub use srdzr::*;

mod srranch;
pub use srranch::*;

mod srped;
pub use srped::*;

mod srgame;
pub use srgame::*;

mod srse;
pub use srse::*;

mod sruci;
pub use sruci::*;

mod srsecretstyledisc;
pub use srsecretstyledisc::*;

mod sreventrecord;
pub use sreventrecord::*;

mod srevententry;
pub use srevententry::*;

mod srsed;
pub use srsed::*;

mod srad;
pub use srad::*;

mod srsei;
pub use srsei::*;

mod srapp;
pub use srapp::*;

mod srresnode;
pub use srresnode::*;

mod trackedactorlist;
pub use trackedactorlist::*;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum PlotType {
    Empty = 1,
    HenCoop = 3,
    SlimeEnclosure = 2,
    Silo = 5,
    Pond = 6,
    Incinerator = 7,
    Garden = 4,
    Unknown = 255,
}
impl From<u8> for PlotType {
    fn from(v: u8) -> Self {
        match v {
            1 => Self::Empty,
            2 => Self::SlimeEnclosure,
            3 => Self::HenCoop,
            4 => Self::Garden,
            5 => Self::Silo,
            6 => Self::Pond,
            7 => Self::Incinerator,
            _ => Self::Unknown,
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum InventoryType {
    Silo,
    FoodDispensor,
    PlortVaccuum,
    OldHenVaccuum,
    Other(String),
}

impl From<&str> for InventoryType {
    fn from(t: &str) -> Self {
        match t {
            "58d5bd4fc903e1c49aba61495aa74014" => Self::Silo,
            "7e1edc80785d7894a928f24f5aebbccd" => Self::FoodDispensor,
            "83f638af7ebb11944b6b55c915889459" => Self::PlortVaccuum,
            "e65dad0e2c627f8498d5a2b3b65f6215" => Self::OldHenVaccuum,
            _ => Self::Other(t.into()),
        }
    }
}
