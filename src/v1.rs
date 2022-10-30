use crate::{simple_obj, Obj, primitives::{InGameTime, VecMap}, SRV3, PPrintable};

use std::fmt::Debug;

#[derive(Debug, Clone, PartialEq)]
pub struct SRGSUMM {
    pub version: String,
    pub a: i32,
    pub b: i32,
    pub c: i32,
    pub d: i32,
    pub time: InGameTime,
    pub e: f64,
    pub f: i32,
    pub g: i32,
    pub h: i32,
    pub i: i32,
    pub boo: bool,
}
simple_obj!(SRGSUMM : 4, "SRGSUMM", version, a, b, c, d, time, e, f, g, h, i, boo);
impl<T> PPrintable<T> for SRGSUMM {
    fn pprint(&self, printer: &mut crate::Printer<T>) -> std::io::Result<()> {
        printer.object("SRGSUMM", |p| {
            p.field("Version")?.value(&self.version)?;
            p.ufield("a")?.value(&self.a)?;
            p.ufield("b")?.value(&self.b)?;
            p.ufield("c")?.value(&self.c)?;
            p.ufield("d")?.value(&self.d)?;
            p.ufield("e")?.value(&self.e)?;
            p.ufield("f")?.value(&self.f)?;
            p.ufield("g")?.value(&self.g)?;
            p.ufield("h")?.value(&self.h)?;
            p.ufield("i")?.value(&self.i)?;
            p.ufield("time")?.value(&self.time)?;
            p.ufield("boo")?.value(&self.boo)?;
            Ok(())
        })
    }
}


#[derive(Debug, Clone, PartialEq)]
pub struct SRW {
    pub time: InGameTime,
    pub a: f32,
    pub time2: InGameTime,
    pub b: Vec<String>,
    pub c: Vec<String>,
    pub d: InGameTime,
    pub e: i32,
    pub f: VecMap<i32, SREO>,
    pub g: VecMap<i32, f32>,
    pub h: i32,
    pub j: VecMap<SRV3, InGameTime>,
    pub liquid_sources: VecMap<String, f32>,
    pub l: VecMap<SRV3, InGameTime>,
    pub gordos: VecMap<String, SRG>,
    pub m: VecMap<SRV3, SRRW>,
    pub gadgets: VecMap<String, SRPG>,
    pub treasure_pods: VecMap<String, SRTP>,
    pub switches: VecMap<String, i32>,
    pub puzzles: VecMap<String, bool>,
    pub phase_site: VecMap<String, bool>,
    pub srf: SRF,
    pub oases: VecMap<String, bool>,
    pub ginger_patches: Vec<String>,
    pub qsegs: VecMap<String, SRQSEG>,
    pub gordo_echo_notes: VecMap<String, Option<SRENG>>,
    pub glitch: Option<SRGLITCH>,
    
}
simple_obj!(SRW : 0x16, "SRW", time, a, time2, b, c, d, e, f, g, h, j, liquid_sources, l, gordos, m, 
    gadgets, treasure_pods, switches, puzzles, phase_site, srf, oases, ginger_patches,
    qsegs, gordo_echo_notes, glitch);

impl<T> PPrintable<T> for SRW {
    fn pprint(&self, printer: &mut crate::Printer<T>) -> std::io::Result<()> {
        printer.object(Self::NAME, |p| {
            p.ufield("time")?.value(&self.time)?;
            p.ufield("a")?.value(&self.a)?;
            p.ufield("time2")?.value(&self.time2)?;
            p.ufield("b")?.value(&self.b)?;
            p.ufield("c")?.value(&self.c)?;
            p.ufield("d")?.value(&self.d)?;
            p.ufield("e")?.value(&self.e)?;
            p.ufield("f")?.value(&self.f)?;
            p.ufield("g")?.value(&self.g)?;
            p.ufield("h")?.value(&self.h)?;
            
            p.ufield("j")?.value(&self.j)?;
            p.ufield("Liquid sources")?.value(&self.liquid_sources)?;
            p.ufield("l")?.value(&self.l)?;
            p.ufield("gordos")?.value(&self.gordos)?;
            p.ufield("m")?.value(&self.m)?;
            p.ufield("gadgets")?.value(&self.gadgets)?;
            p.ufield("treasure_pods")?.value(&self.treasure_pods)?;
            p.ufield("switches")?.value(&self.switches)?;
            p.ufield("puzzles")?.value(&self.puzzles)?;
            p.ufield("phase_site")?.value(&self.phase_site)?;
            p.ufield("srf")?.value(&self.srf)?;
            p.ufield("oases")?.value(&self.oases)?;
            p.ufield("ginger_patches")?.value(&self.ginger_patches)?;
            p.ufield("qsegs")?.value(&self.qsegs)?;
            p.ufield("gordo_echo_notes")?.value(&self.gordo_echo_notes)?;
            p.ufield("glitch")?.value(&self.glitch)?;
            Ok(())
        })
    }
}
    
#[derive(Debug, Clone, PartialEq)]
pub struct SREO {
    pub b1: bool,
    pub s1: String,
    pub s2: String,
    pub a: i32,
    pub b: f32,
    pub c: i32,
    pub d: f32,
    pub e: Vec<SRRIE>,
    pub f: Vec<SRIE>,
}

simple_obj!(SREO : 0x04, "SREO", b1, s1, s2, a, b, c, d, e, f );

impl<T> PPrintable<T> for SREO {
    fn pprint(&self, printer: &mut crate::Printer<T>) -> std::io::Result<()> {
        printer.object(Self::NAME, |p| {
            p.ufield("b1")?.value(&self.b1)?;
            p.ufield("s1")?.value(&self.s1)?;
            p.ufield("s2")?.value(&self.s2)?;
            p.ufield("a")?.value(&self.a)?;
            p.ufield("b")?.value(&self.b)?;
            p.ufield("c")?.value(&self.c)?;
            p.ufield("d")?.value(&self.d)?;
            p.ufield("e")?.value(&self.e)?;
            p.ufield("f")?.value(&self.f)?;
            Ok(())
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SRRIE {
    pub a: i32,
    pub b: f32,
    pub c: i32,
    pub d: i32,
}

simple_obj!(SRRIE : 0x03, "SRRIE", a, b, c, d);

impl<T> PPrintable<T> for SRRIE {
    fn pprint(&self, printer: &mut crate::Printer<T>) -> std::io::Result<()> {
        printer.object(Self::NAME, |p| {
            p.ufield("a")?.value(&self.a)?;
            p.ufield("b")?.value(&self.b)?;
            p.ufield("c")?.value(&self.c)?;
            p.ufield("d")?.value(&self.d)?;
            Ok(())
        })
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct SRIE {
    pub a: i32,
    pub b: f32,
    pub c: i32,
}
simple_obj!(SRIE : 0x03, "SRIE", a, b, c);

impl<T> PPrintable<T> for SRIE {
    fn pprint(&self, printer: &mut crate::Printer<T>) -> std::io::Result<()> {
        printer.object(Self::NAME, |p| {
            p.ufield("a")?.value(&self.a)?;
            p.ufield("b")?.value(&self.b)?;
            p.ufield("c")?.value(&self.c)?;
            Ok(())
        })
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SRG {
    // Gordos
    pub fed: i32,
    pub unknown: i32
}
simple_obj!(SRG : 1, "SRG", fed, unknown);

impl<T> PPrintable<T> for SRG {
    fn pprint(&self, printer: &mut crate::Printer<T>) -> std::io::Result<()> {
        printer.object(Self::NAME, |p| {
            p.ufield("fed")?.value(&self.fed)?;
            p.ufield("unknown")?.value(&self.unknown)?;
           
            Ok(())
        })
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct SRRW {
    pub a: InGameTime,
    pub unknown: i32
}
simple_obj!(SRRW : 3, "SRRW", a, unknown);

impl<T> PPrintable<T> for SRRW {
    fn pprint(&self, printer: &mut crate::Printer<T>) -> std::io::Result<()> {
        printer.object(Self::NAME, |p| {
            p.ufield("a")?.value(&self.a)?;
            p.ufield("unknown")?.value(&self.unknown)?;

            Ok(())
        })
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct SRPG {
    pub a: i32,
    pub b: f32,
    pub c1: (bool, Vec<SRAD>),    
    pub c2: i32,
    pub c3: i32,
    pub c4: i32,
    pub c5: InGameTime,
    pub c: [u8; 0x24],
    pub d: Option<SRDRGD>
}
simple_obj!(SRPG : 8, "SRPG", a, b, c1, c2, c3, c4, c5, c, d);

impl<T> PPrintable<T> for SRPG {
    fn pprint(&self, printer: &mut crate::Printer<T>) -> std::io::Result<()> {
        printer.object(Self::NAME, |p| {
            p.ufield("a")?.value(&self.a)?;
            p.ufield("b")?.value(&self.b)?;
            p.ufield("c1")?.value(&self.c1)?;
            p.ufield("c2")?.value(&self.c2)?;
            p.ufield("c3")?.value(&self.c3)?;
            p.ufield("c4")?.value(&self.c4)?;
            p.ufield("c5")?.value(&self.c5)?;
            p.ufield("c")?.value(&self.c)?;
            p.ufield("d")?.value(&self.d)?;

            Ok(())
        })
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct SRDRGD {
    pub b1: bool,
    pub a: SRDRONE,
    pub b2: bool,
    pub b: SRDRST,
    pub c: Vec<SRDRONEPROG>
}
simple_obj!(SRDRGD : 1, "SRDRGD", b1, a, b2, b, c);

impl<T> PPrintable<T> for SRDRGD {
    fn pprint(&self, printer: &mut crate::Printer<T>) -> std::io::Result<()> {
        printer.object(Self::NAME, |p| {
            p.ufield("b1")?.value(&self.b1)?;
            p.ufield("a")?.value(&self.a)?;
            p.ufield("b2")?.value(&self.b2)?;
            p.ufield("b")?.value(&self.b)?;
            p.ufield("c")?.value(&self.c)?;

            Ok(())
        })
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct SRDRONE {
    pub b1: bool,
    pub b: SRV3,
    pub c1: bool,
    pub c: SRV3,
    pub d1: bool,
    pub d: SRAD,
    pub e: i32,
    pub f: bool,
}
simple_obj!(SRDRONE : 8, "SRDRONE", b1, b, c1, c, d1, d, e, f);

impl<T> PPrintable<T> for SRDRONE {
    fn pprint(&self, printer: &mut crate::Printer<T>) -> std::io::Result<()> {
        printer.object(Self::NAME, |p| {
            p.ufield("b1")?.value(&self.b1)?;
            p.ufield("b")?.value(&self.b)?;
            p.ufield("c1")?.value(&self.c1)?;
            p.ufield("c")?.value(&self.c)?;
            p.ufield("d1")?.value(&self.d1)?;
            p.ufield("d")?.value(&self.d)?;
            p.ufield("e")?.value(&self.e)?;
            p.ufield("f")?.value(&self.f)?;

            Ok(())
        })
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct SRAD {
    pub a: i32,
    pub b: i32,
    pub c: SRSED,
}
simple_obj!(SRAD : 2, "SRAD", a, b, c);

impl<T> PPrintable<T> for SRAD {
    fn pprint(&self, printer: &mut crate::Printer<T>) -> std::io::Result<()> {
        printer.object(Self::NAME, |p| {
            p.ufield("a")?.value(&self.a)?;
            p.ufield("b")?.value(&self.b)?;
            p.ufield("c")?.value(&self.c)?;

            Ok(())
        })
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct SRSED {
    pub a: Vec<(i32, f32)>,
}
simple_obj!(SRSED : 2, "SRSED", a);

impl<T> PPrintable<T> for SRSED {
    fn pprint(&self, printer: &mut crate::Printer<T>) -> std::io::Result<()> {
        printer.object(Self::NAME, |p| {
            p.ufield("a")?.value(&self.a)?;

            Ok(())
        })
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct SRDRST {
    pub b1: bool,
    pub a: SRDRSTB
}
simple_obj!(SRDRST : 1, "SRDRST", b1, a);

impl<T> PPrintable<T> for SRDRST {
    fn pprint(&self, printer: &mut crate::Printer<T>) -> std::io::Result<()> {
        printer.object(Self::NAME, |p| {
            p.ufield("b1")?.value(&self.b1)?;
            p.ufield("a")?.value(&self.a)?;

            Ok(())
        })
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct SRDRSTB {
    pub a: f64,
}
simple_obj!(SRDRSTB : 1, "SRDRSTB", a);

impl<T> PPrintable<T> for SRDRSTB {
    fn pprint(&self, printer: &mut crate::Printer<T>) -> std::io::Result<()> {
        printer.object(Self::NAME, |p| {
            p.ufield("a")?.value(&self.a)?;

            Ok(())
        })
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SRDRONEPROG {
    a: String,
    b: String,
    c: String,
}
simple_obj!(SRDRONEPROG : 1, "SRDRONEPROG", a, b, c);


impl<T> PPrintable<T> for SRDRONEPROG {
    fn pprint(&self, printer: &mut crate::Printer<T>) -> std::io::Result<()> {
        printer.object(Self::NAME, |p| {
            p.ufield("a")?.value(&self.a)?;
            p.ufield("b")?.value(&self.b)?;
            p.ufield("c")?.value(&self.c)?;

            Ok(())
        })
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SRTP {
    pub a: i32,
    pub b: i32,
}
simple_obj!(SRTP : 1, "SRTP", a, b);

impl<T> PPrintable<T> for SRTP {
    fn pprint(&self, printer: &mut crate::Printer<T>) -> std::io::Result<()> {
        printer.object(Self::NAME, |p| {
            p.ufield("a")?.value(&self.a)?;
            p.ufield("b")?.value(&self.b)?;

            Ok(())
        })
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct SRQSEG {
    pub a: bool,
    pub b: i32,
}
simple_obj!(SRQSEG : 2, "SRQSEG", a, b);

impl<T> PPrintable<T> for SRQSEG {
    fn pprint(&self, printer: &mut crate::Printer<T>) -> std::io::Result<()> {
        printer.object(Self::NAME, |p| {
            p.ufield("a")?.value(&self.a)?;
            p.ufield("b")?.value(&self.b)?;

            Ok(())
        })
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct SRF {
    pub a: f64,
    pub c: bool,
    pub b: f64,
}
simple_obj!(SRF : 1, "SRF", a, c, b);

impl<T> PPrintable<T> for SRF {
    fn pprint(&self, printer: &mut crate::Printer<T>) -> std::io::Result<()> {
        printer.object(Self::NAME, |p| {
            p.ufield("a")?.value(&self.a)?;
            p.ufield("b")?.value(&self.b)?;
            p.ufield("c")?.value(&self.c)?;

            Ok(())
        })
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct SRENG {
    pub a: i32,
}
simple_obj!(SRENG : 1, "SRENG", a);

impl<T> PPrintable<T> for SRENG {
    fn pprint(&self, printer: &mut crate::Printer<T>) -> std::io::Result<()> {
        printer.object(Self::NAME, |p| {
            p.ufield("a")?.value(&self.a)?;

            Ok(())
        })
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct SRGLITCH {
    pub a: VecMap<String, Option<SRGLITCH_TPD>>,
    pub b: VecMap<String, Option<SRGLITCH_TS>>,
    pub n: i32,
    pub cell_slimulation: VecMap<String, Option<SRGLITCH_ID>>,
    pub imposters: VecMap<String, Option<SRGLITCH_IP>>,
    pub glitches: VecMap<String, Option<SRGLITCH_ST>>,
}
simple_obj!(SRGLITCH : 2, "SRGLITCH", a, b, n, cell_slimulation, imposters, glitches);

impl<T> PPrintable<T> for SRGLITCH {
    fn pprint(&self, printer: &mut crate::Printer<T>) -> std::io::Result<()> {
        printer.object(Self::NAME, |p| {
            p.ufield("a")?.value(&self.a)?;
            p.ufield("b")?.value(&self.b)?;
            p.ufield("n")?.value(&self.n)?;
            p.ufield("cell_slimulation")?.value(&self.cell_slimulation)?;
            p.ufield("imposters")?.value(&self.imposters)?;
            p.ufield("glitches")?.value(&self.glitches)?;

            Ok(())
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SRGLITCH_TPD {
    pub a: bool,
}
simple_obj!(SRGLITCH_TPD : 1, "SRGLITCH_TPD", a);

impl<T> PPrintable<T> for SRGLITCH_TPD {
    fn pprint(&self, printer: &mut crate::Printer<T>) -> std::io::Result<()> {
        printer.object(Self::NAME, |p| {
            p.ufield("a")?.value(&self.a)?;

            Ok(())
        })
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct SRGLITCH_TS {
    pub a: f64,
}
simple_obj!(SRGLITCH_TS : 1, "SRGLITCH_TS", a);

impl<T> PPrintable<T> for SRGLITCH_TS {
    fn pprint(&self, printer: &mut crate::Printer<T>) -> std::io::Result<()> {
        printer.object(Self::NAME, |p| {
            p.ufield("a")?.value(&self.a)?;

            Ok(())
        })
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct SRGLITCH_ID {
    pub a: Option<f64>,
}
simple_obj!(SRGLITCH_ID : 1, "SRGLITCH_ID", a);

impl<T> PPrintable<T> for SRGLITCH_ID {
    fn pprint(&self, printer: &mut crate::Printer<T>) -> std::io::Result<()> {
        printer.object(Self::NAME, |p| {
            p.ufield("a")?.value(&self.a)?;

            Ok(())
        })
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct SRGLITCH_IP {
    pub a: bool,
    pub b: f64,
}
simple_obj!(SRGLITCH_IP : 1, "SRGLITCH_IP", a, b);

impl<T> PPrintable<T> for SRGLITCH_IP {
    fn pprint(&self, printer: &mut crate::Printer<T>) -> std::io::Result<()> {
        printer.object(Self::NAME, |p| {
            p.ufield("a")?.value(&self.a)?;
            p.ufield("b")?.value(&self.b)?;

            Ok(())
        })
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct SRGLITCH_ST {
    pub a: f64,
}
simple_obj!(SRGLITCH_ST : 1, "SRGLITCH_ST", a);

impl<T> PPrintable<T> for SRGLITCH_ST {
    fn pprint(&self, printer: &mut crate::Printer<T>) -> std::io::Result<()> {
        printer.object(Self::NAME, |p| {
            p.ufield("a")?.value(&self.a)?;

            Ok(())
        })
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct SRGAME1 {
    pub name_base: String,
    pub save_name: String,
    pub summary: SRGSUMM,
    pub world: SRW,
    pub player: SRPL,
    pub ranch: SRRANCH,
    pub unknown: [u8; 2],
    pub actors: Vec<SRAD2>,
    pub unknown2: [u8; 2],
    pub srped: SRPED,
    pub srga: SRGA,
    pub srhd: SRHD,
    pub srapp: SRAPP,
    pub srinstr: SRINSTR,
}

simple_obj!(SRGAME1 : 12, "SRGAME", name_base, save_name, summary, world, player, ranch, unknown, actors, unknown2, srped, srga, srhd, srapp, srinstr);

impl<T> PPrintable<T> for SRGAME1 {
    fn pprint(&self, printer: &mut crate::Printer<T>) -> std::io::Result<()> {
        printer.object(Self::NAME, |p| {
            p.field("Name base")?.value(&self.name_base)?;
            p.field("Save name")?.value(&self.save_name)?;
            p.field("Summary")?.value(&self.summary)?;
            p.field("World")?.value(&self.world)?;
            p.field("Player")?.value(&self.player)?;
            p.field("Ranch")?.value(&self.ranch)?;
            p.field("Actors")?.value(&self.actors)?;
            p.field("srped")?.value(&self.srped)?;
            p.ufield("srga")?.value(&self.srga)?;
            p.ufield("srhd")?.value(&self.srhd)?;
            p.ufield("srapp")?.value(&self.srapp)?;
            p.ufield("srinstr")?.value(&self.srinstr)?;
            Ok(())
        })
    }
}


#[derive(Debug, Clone, PartialEq)]
pub struct SRPL {
    pub hp: i32,
    pub energy: i32,
    pub a: i32,
    pub b: i32,
    pub c: i32,
    pub d: i32,
    pub build: String,
    pub e: i32,
    pub f: i32,
    pub pos: SRV3,
    pub facing: SRV3,
    pub g: Vec<i32>,
    pub inv: VecMap<i32, Vec<SRAD>>,
    pub mail: Vec<SRMAIL>,
    pub h: Vec<i32>,
    pub i: VecMap<i32, (bool, InGameTime)>,
    pub j: VecMap<i32, i32>,
    pub k: VecMap<i32, InGameTime>,
    pub l: Vec<i32>,
    pub m: Vec<i32>,
    pub n: VecMap<i32, (bool, f64)>,
    pub o: VecMap<i32, i32>,
    pub p: VecMap<i32, i32>,
    pub q: i32,
    pub r: Vec<i32>,
    pub s: bool,
    pub t: bool,
    pub srdzr: SRDZR,
}
simple_obj!(SRPL : 0x0e, "SRPL", hp, energy, a, b, c, d, build, e, f, pos, facing, g, inv, mail, h, i, j, k, l, m, n, o, p, q, r, s, t, srdzr);

impl<T> PPrintable<T> for SRPL {
    fn pprint(&self, printer: &mut crate::Printer<T>) -> std::io::Result<()> {
        printer.object(Self::NAME, |p| {
            p.ufield("a")?.value(&self.a)?;
            p.ufield("b")?.value(&self.b)?;
            p.ufield("c")?.value(&self.c)?;
            p.ufield("d")?.value(&self.d)?;
            p.ufield("build")?.value(&self.build)?;
            p.ufield("e")?.value(&self.e)?;
            p.ufield("f")?.value(&self.f)?;
            p.ufield("pos")?.value(&self.pos)?;
            p.ufield("facing")?.value(&self.facing)?;
            p.ufield("g")?.value(&self.g)?;
            p.ufield("inv")?.value(&self.inv)?;
            p.ufield("mail")?.value(&self.mail)?;
            p.ufield("h")?.value(&self.h)?;
            p.ufield("i")?.value(&self.i)?;
            p.ufield("j")?.value(&self.j)?;
            p.ufield("k")?.value(&self.k)?;
            p.ufield("l")?.value(&self.l)?;
            p.ufield("m")?.value(&self.m)?;
            p.ufield("n")?.value(&self.n)?;
            p.ufield("o")?.value(&self.o)?;
            p.ufield("p")?.value(&self.p)?;
            p.ufield("q")?.value(&self.q)?;
            p.ufield("r")?.value(&self.r)?;
            p.ufield("s")?.value(&self.s)?;
            p.ufield("t")?.value(&self.t)?;
            p.ufield("srdzr")?.value(&self.srdzr)?;


            Ok(())
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SRMAIL {
    pub a: i32,
    pub b: String,
    pub c: bool,
}
simple_obj!(SRMAIL : 1, "SRMAIL", a, b, c);

impl<T> PPrintable<T> for SRMAIL {
    fn pprint(&self, printer: &mut crate::Printer<T>) -> std::io::Result<()> {
        printer.object(Self::NAME, |p| {
            p.ufield("a")?.value(&self.a)?;
            p.ufield("b")?.value(&self.b)?;
            p.ufield("c")?.value(&self.c)?;

            Ok(())
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SRDZR {
    pub a: VecMap<i32, i32>,
    pub b: VecMap<String, Option<SRDZRSETTINGS>>,
}
simple_obj!(SRDZR : 1, "SRDZR", a, b);

impl<T> PPrintable<T> for SRDZR {
    fn pprint(&self, printer: &mut crate::Printer<T>) -> std::io::Result<()> {
        printer.object(Self::NAME, |p| {
            p.ufield("a")?.value(&self.a)?;
            p.ufield("b")?.value(&self.b)?;

            Ok(())
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SRDZRSETTINGS {
    pub a: i32
}
simple_obj!(SRDZRSETTINGS : 1, "SRDZRSETTINGS", a);

impl<T> PPrintable<T> for SRDZRSETTINGS {
    fn pprint(&self, printer: &mut crate::Printer<T>) -> std::io::Result<()> {
        printer.object(Self::NAME, |p| {
            p.ufield("a")?.value(&self.a)?;

            Ok(())
        })
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct SRRANCH {
    pub plots: Vec<SRLP>,
    pub doors: VecMap<String, i32>,
    pub a: VecMap<i32, i32>,
    pub b: VecMap<String, f64>,
}
simple_obj!(SRRANCH : 7, "SRRANCH", plots, doors, a, b);

impl<T> PPrintable<T> for SRRANCH {
    fn pprint(&self, printer: &mut crate::Printer<T>) -> std::io::Result<()> {
        printer.object(Self::NAME, |p| {
            p.ufield("plots")?.value(&self.plots)?;
            p.ufield("doors")?.value(&self.doors)?;
            p.ufield("a")?.value(&self.a)?;
            p.ufield("b")?.value(&self.b)?;

            Ok(())
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SRLP {
    pub a: [u8; 0x28],
    pub name: String,
    pub upgrades: Vec<i32>,
    pub b: VecMap<i32, Vec<SRAD>>,
    pub e: Vec<i32>,
    pub f: i32,
}
simple_obj!(SRLP : 8, "SRLP", a, name, upgrades, b, e, f);

impl<T> PPrintable<T> for SRLP {
    fn pprint(&self, printer: &mut crate::Printer<T>) -> std::io::Result<()> {
        printer.object(Self::NAME, |p| {
            p.ufield("a")?.value(&self.a)?;
            p.ufield("name")?.value(&self.name)?;
            p.ufield("upgrades")?.value(&self.upgrades)?;
            p.ufield("b")?.value(&self.b)?;
            p.ufield("e")?.value(&self.e)?;
            p.ufield("f")?.value(&self.f)?;

            Ok(())
        })
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct SRAD2 {
    pub pos: SRV3,
    pub facing: SRV3,
    pub index: i32,
    pub unknown: [u8; 4],
    pub actor_type: i32,
    pub srsed: SRSED,
    // NaN for grown fowl, timestamp for chicks, 0 otherwise
    pub chick_timer: InGameTime,
    // NaN for chicks and roosters, timestamp for hens, 0 otherwise
    pub hen_timer: InGameTime,
    // Future time for plorts, 0 otherwise
    pub plort_expire: InGameTime,
    pub srrcd: SRRCD,
    pub ts: Option<InGameTime>,
    pub more_bytes: [u8; 10],
    
}
simple_obj!(SRAD2 : 9, "SRAD", pos, facing, index, unknown, actor_type, srsed, chick_timer, hen_timer, plort_expire, srrcd, ts, more_bytes);

impl<T> PPrintable<T> for SRAD2 {
    fn pprint(&self, printer: &mut crate::Printer<T>) -> std::io::Result<()> {
        printer.object(Self::NAME, |p| {
            p.ufield("pos")?.value(&self.pos)?;
            p.ufield("facing")?.value(&self.facing)?;
            p.ufield("index")?.value(&self.index)?;
            p.ufield("unknown")?.value(&self.unknown)?;
            p.ufield("actor_type")?.value(&self.actor_type)?;
            p.ufield("srsed")?.value(&self.srsed)?;
            p.ufield("chick_timer")?.value(&self.chick_timer)?;
            p.ufield("hen_timer")?.value(&self.hen_timer)?;
            p.ufield("plort_expire")?.value(&self.plort_expire)?;
            p.ufield("srrcd")?.value(&self.srrcd)?;
            p.ufield("ts")?.value(&self.ts)?;
            p.ufield("more_bytes")?.value(&self.more_bytes)?;


            Ok(())
        })
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
pub struct SRRCD {
    pub condition: i32,
    pub time: InGameTime,
}
simple_obj!(SRRCD, "SRRCD", condition, time);

impl<T> PPrintable<T> for SRRCD {
    fn pprint(&self, printer: &mut crate::Printer<T>) -> std::io::Result<()> {
        printer.object(Self::NAME, |p| {
            p.ufield("condition")?.value(&self.condition)?;
            p.ufield("time")?.value(&self.time)?;

            Ok(())
        })
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct SRSE {
    pub unknown: [u8; 4],
    pub ts: InGameTime,
}
simple_obj!(SRSE, "SRSE", unknown, ts);

impl<T> PPrintable<T> for SRSE {
    fn pprint(&self, printer: &mut crate::Printer<T>) -> std::io::Result<()> {
        printer.object(Self::NAME, |p| {
            p.ufield("unknown")?.value(&self.unknown)?;
            p.ufield("ts")?.value(&self.ts)?;

            Ok(())
        })
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
pub struct SRPED {
    pub a: i32, // Probably an array?
    pub index: Vec<String>,
    pub index2: Vec<String>,
    pub b: i32,
}
simple_obj!(SRPED : 3, "SRPED", a, index, index2, b);

impl<T> PPrintable<T> for SRPED {
    fn pprint(&self, printer: &mut crate::Printer<T>) -> std::io::Result<()> {
        printer.object(Self::NAME, |p| {
            p.ufield("a")?.value(&self.a)?;
            p.ufield("index")?.value(&self.index)?;
            p.ufield("index2")?.value(&self.index2)?;
            p.ufield("b")?.value(&self.b)?;

            Ok(())
        })
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct SRGA {
    pub a: i32,
    pub b: VecMap<i32, f64>,
    pub c: VecMap<i32, i32>,
    pub d: VecMap<i32, VecMap<i32, i32>>,
}
simple_obj!(SRGA : 3, "SRGA", a, b, c, d);

impl<T> PPrintable<T> for SRGA {
    fn pprint(&self, printer: &mut crate::Printer<T>) -> std::io::Result<()> {
        printer.object(Self::NAME, |p| {
            p.ufield("a")?.value(&self.a)?;
            p.ufield("b")?.value(&self.b)?;
            p.ufield("c")?.value(&self.c)?;
            p.ufield("d")?.value(&self.d)?;

            Ok(())
        })
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
pub struct SRHD {
    pub unknown: [u8; 8],
}
simple_obj!(SRHD : 2, "SRHD", unknown);

impl<T> PPrintable<T> for SRHD {
    fn pprint(&self, printer: &mut crate::Printer<T>) -> std::io::Result<()> {
        printer.object(Self::NAME, |p| {
            p.ufield("unknown")?.value(&self.unknown)?;

            Ok(())
        })
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
pub struct SRAPP {
    pub item_map: VecMap<i32, (i32, i32)>,
    pub unknown: VecMap<i32, i32>,
}
simple_obj!(SRAPP, "SRAPP", item_map, unknown);

impl<T> PPrintable<T> for SRAPP {
    fn pprint(&self, printer: &mut crate::Printer<T>) -> std::io::Result<()> {
        printer.object(Self::NAME, |p| {
            p.ufield("item_map")?.value(&self.item_map)?;
            p.ufield("unknown")?.value(&self.unknown)?;

            Ok(())
        })
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
pub struct SRINSTR {
    pub unknown: [u8; 8],
}
simple_obj!(SRINSTR, "SRINSTR", unknown);

impl<T> PPrintable<T> for SRINSTR {
    fn pprint(&self, printer: &mut crate::Printer<T>) -> std::io::Result<()> {
        printer.object(Self::NAME, |p| {
            p.ufield("unknown")?.value(&self.unknown)?;

            Ok(())
        })
    }
}