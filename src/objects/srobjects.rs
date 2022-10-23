mod srv3;
#[cfg(test)]
mod test;
pub use srv3::*;
mod srsgi;
pub use srsgi::*;

use std::io::Write;

use nom::{error::ContextError, IResult};

use super::{macros::simple_obj, Parseable};
use std::fmt::Debug;

fn parse_obj_prefix<'a>(name: &str, input: &'a [u8]) -> IResult<&'a [u8], ()> {
    let (input, _) = nom::bytes::complete::tag([name.len() as u8])(input)?;
    let (input, _) = nom::bytes::complete::tag(name.as_bytes())(input)?;
    let (input, _) = nom::number::complete::le_i32(input)?;
    let (input, _) = nom::bytes::complete::tag([0x00, 0x30])(input)?;
    Ok((input, ()))
}

fn parse_obj_suffix(input: &[u8]) -> IResult<&[u8], ()> {
    let (input, _) = nom::bytes::complete::tag([0x00, 0x40])(input)?;
    Ok((input, ()))
}

trait Obj: Sized {
    const NAME: &'static str;
    const VERSION: i32 = 1;
    fn parse_prefix(input: &[u8]) -> IResult<&[u8], ()> {
        parse_obj_prefix(Self::NAME, input)
    }
    fn parse_suffix(input: &[u8]) -> IResult<&[u8], ()> {
        parse_obj_suffix(input)
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
