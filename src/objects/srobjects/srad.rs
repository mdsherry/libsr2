use super::{Obj, SRSe, SRSed, Srrcd, Srv3};
use crate::objects::{
    macros::{simple_obj, simple_parse},
    InGameTime, ItemId, PPrintable, Parseable, Printer, SceneGroupId,
};
use nom::IResult;
use std::{fmt::Debug, io::Write};

#[derive(Debug, Clone, PartialEq)]
pub enum SRAD {
    V1(SRAD1),
    V2(SRAD2),
}
impl SRAD {
    pub fn as_v1(&self) -> Option<&SRAD1> {
        match self {
            SRAD::V1(v1) => Some(v1),
            SRAD::V2(_) => None,
        }
    }
    pub fn as_v2(&self) -> Option<&SRAD2> {
        match self {
            SRAD::V1(_) => None,
            SRAD::V2(v2) => Some(v2),
        }
    }
}
impl Default for SRAD {
    fn default() -> Self {
        SRAD::V1(Default::default())
    }
}
impl Parseable for SRAD {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        nom::branch::alt((
            nom::combinator::map(SRAD1::parse, SRAD::V1),
            nom::combinator::map(SRAD2::parse, SRAD::V2),
        ))(input)
    }

    fn write<W: Write>(&self, f: &mut W) -> std::io::Result<()> {
        match self {
            SRAD::V1(v1) => v1.write(f),
            SRAD::V2(v2) => v2.write(f),
        }
    }
}
impl PPrintable for SRAD {
    fn pprint(&self, printer: &mut Printer) {
        match self {
            SRAD::V1(v1) => v1.pprint(printer),
            SRAD::V2(v2) => v2.pprint(printer),
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SRAD1 {
    pub item_id: ItemId,
    pub count: i32,
    pub c: i32,
    pub sed: SRSed,
}
simple_obj!(SRAD1, "SRAD", item_id, count, c, sed);
impl PPrintable for SRAD1 {
    fn pprint(&self, printer: &mut Printer) {
        printer.object("SRAD::V1", |p| {
            p.field("Item ID").value(self.item_id);
            p.field("Count").value(self.count);
            p.ufield("c").value(self.c);
            p.ufield("sed").value(&self.sed);
        });
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SRAD2 {
    pub pos: Srv3,
    pub facing: Srv3,
    pub index: i32,
    pub unknown: [u8; 4],
    pub actor_type: ItemId,
    pub srsed: SRSed,
    // NaN for grown fowl, timestamp for chicks, 0 otherwise
    pub chick_timer: InGameTime,
    // NaN for chicks and roosters, timestamp for hens, 0 otherwise
    pub hen_timer: InGameTime,
    // Future time for plorts, 0 otherwise
    pub plort_expire: InGameTime,
    pub srrcd: Srrcd,
    pub has_extra_ts: bool,
    pub ts4: Option<InGameTime>,
    pub feral: bool,
    pub more_bytes: [u8; 5],
    pub zone: SceneGroupId,
    pub statuified: bool,
    pub statue_actor_id: i32,
    pub even_more_bytes: [u8; 4],
    pub ts5: InGameTime,
    pub srses: Vec<SRSe>,
}

impl PPrintable for SRAD2 {
    fn pprint(&self, printer: &mut Printer) {
        printer.object("SRAD::V2", |p| {
            p.field("Pos").value(&self.pos);
            p.field("Facing").value(&self.facing);
            p.field("Actor type").value(self.actor_type);
            p.ufield("Index").value(self.index);
            p.ufield("unknown").value(self.unknown);
            p.ufield("srsed").value(&self.srsed);
            p.ufield("Chick timer").value(self.chick_timer);
            p.ufield("Hen timer").value(self.hen_timer);
            p.ufield("Plort expires").value(self.plort_expire);
            p.ufield("Plant state").value(&self.srrcd);
            p.ufield("Has extra timestamp").value(self.has_extra_ts);
            p.field("Feral").value(self.feral);

            if let Some(ts) = self.ts4 {
                p.ufield("ts4").value(ts);
            }
            p.ufield("more_bytes").value(self.more_bytes.as_slice());
            p.ufield("zone").value(self.zone);
            p.field("Statuified").value(self.statuified);
            p.field("Statued actor ID").value(self.statue_actor_id);
            p.ufield("even_more_bytes")
                .value(self.even_more_bytes.as_slice());
            p.ufield("ts5").value(self.ts5);
            p.ufield("srses").value(&self.srses);
        })
    }
}

impl Obj for SRAD2 {
    const NAME: &'static str = "SRAD";

    fn parse_body(input: &[u8]) -> IResult<&[u8], Self> {
        simple_parse!(
            input,
            pos,
            facing,
            index,
            unknown,
            actor_type,
            srsed,
            chick_timer,
            hen_timer,
            plort_expire,
            srrcd,
            has_extra_ts
        );
        let has_extra_ts: bool = has_extra_ts;
        let (input, ts4) = if !has_extra_ts {
            (input, None)
        } else {
            let (input, ts) = Parseable::parse(input)?;
            (input, Some(ts))
        };
        simple_parse!(
            input,
            feral,
            more_bytes,
            zone,
            statuified,
            statue_actor_id,
            even_more_bytes,
            ts5,
            srses
        );
        Ok((
            input,
            Self {
                pos,
                facing,
                index,
                unknown,
                actor_type,
                srsed,
                chick_timer,
                hen_timer,
                plort_expire,
                srrcd,
                has_extra_ts,
                ts4,
                feral,
                more_bytes,
                zone,
                statuified,
                statue_actor_id,
                even_more_bytes,
                ts5,
                srses,
            },
        ))
    }

    fn write_body<W: Write>(&self, f: &mut W) -> std::io::Result<()> {
        self.pos.write(f)?;
        self.facing.write(f)?;
        self.index.write(f)?;
        self.unknown.write(f)?;
        self.actor_type.write(f)?;
        self.srsed.write(f)?;
        self.chick_timer.write(f)?;
        self.hen_timer.write(f)?;
        self.plort_expire.write(f)?;
        self.srrcd.write(f)?;
        self.has_extra_ts.write(f)?;
        if let Some(ts) = self.ts4 {
            ts.write(f)?;
        }
        self.feral.write(f)?;
        self.more_bytes.write(f)?;
        self.zone.write(f)?;
        self.statuified.write(f)?;
        self.statue_actor_id.write(f)?;
        self.even_more_bytes.write(f)?;
        self.ts5.write(f)?;
        self.srses.write(f)?;
        Ok(())
    }
}
