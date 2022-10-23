mod macros;
use macros::*;

use chrono::{prelude::*, Duration};
use std::{
    collections::HashMap,
    fmt::{Debug, Display},
    fs::File,
    hash::Hash,
    io::{BufWriter, Write},
    path::Path,
};

use nom::IResult;

mod srobjects;
pub use srobjects::*;

pub fn s2a<T: Copy + Sized, const N: usize>(s: &[T]) -> Option<[T; N]> {
    if s.len() != N {
        None
    } else {
        Some(std::array::from_fn(|idx| s[idx]))
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct VecMap<K, V>(pub Vec<(K, V)>);

pub trait PPrintable {
    fn pprint(&self, printer: &mut Printer);
}
impl<P: PPrintable> PPrintable for &P {
    fn pprint(&self, printer: &mut Printer) {
        P::pprint(&self, printer)
    }
}
impl<A: PPrintable, B: PPrintable> PPrintable for (A, B) {
    fn pprint(&self, printer: &mut Printer) {
        printer.print("(");
        self.0.pprint(printer);
        printer.print(", ");
        self.1.pprint(printer);
        printer.print(")");
    }
}
impl<P: PPrintable> PPrintable for Vec<P> {
    fn pprint(&self, printer: &mut Printer) {
        printer.list(self, |p, item| {
            item.pprint(p);
        })
    }
}
impl PPrintable for i32 {
    fn pprint(&self, printer: &mut Printer) {
        printer.print(&format!("{self}"));
    }
}
impl<K: PPrintable, V: PPrintable> PPrintable for VecMap<K, V> {
    fn pprint(&self, printer: &mut Printer) {
        printer.map(&self.0, |p, (k, v)| {
            k.pprint(p);
            p.print(": ");
            v.pprint(p);
        })
    }
}
impl PPrintable for ItemId {
    fn pprint(&self, printer: &mut Printer) {
        let s = if self.0 != usize::MAX {
            printer.game().item_index.identifiable_types[self.0].clone()
        } else {
            "<empty>".to_string()
        };
        printer.print(&s)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct InGameTime(pub f64);
impl Parseable for InGameTime {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let (input, inner) = f64::parse(input)?;
        Ok((input, Self(inner)))
    }

    fn write<W: Write>(&self, f: &mut W) -> std::io::Result<()> {
        self.0.write(f)
    }
}
impl PPrintable for InGameTime {
    fn pprint(&self, printer: &mut Printer) {
        const SECONDS_PER_MINUTE: f64 = 60.;
        const SECONDS_PER_HOUR: f64 = 60. * SECONDS_PER_MINUTE;
        const SECONDS_PER_DAY: f64 = 24. * SECONDS_PER_HOUR;
        let d = (self.0 / SECONDS_PER_DAY) as i32;
        let h = ((self.0 % SECONDS_PER_DAY) / SECONDS_PER_HOUR) as i32;
        let m = ((self.0 % SECONDS_PER_HOUR) / SECONDS_PER_MINUTE) as i32;
        let s = self.0 % SECONDS_PER_MINUTE;
        if self.0.is_nan() {
            printer.print(&format!("NaN: {} {:02x?}", s, s.to_le_bytes()));
        } else {
            printer.print(&format!("{d}d {h:02}:{m:02}:{s:02.2}"));
        }
    }
}
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct TimeSinceYear1(pub i64);
impl Parseable for TimeSinceYear1 {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let (input, inner) = i64::parse(input)?;
        Ok((input, Self(inner)))
    }

    fn write<W: Write>(&self, f: &mut W) -> std::io::Result<()> {
        self.0.write(f)
    }
}
impl PPrintable for TimeSinceYear1 {
    fn pprint(&self, printer: &mut Printer) {
        let epoch = chrono::Utc.ymd(1, 1, 1).and_hms(0, 0, 0);
        let ts = epoch + Duration::seconds(self.0 / 10_000_000);
        printer.print(&format!("{}", ts.format("%Y-%m-%dT%H:%M:%S%z")));
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct WindowsTime(pub i64);
impl Parseable for WindowsTime {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let (input, inner) = i64::parse(input)?;
        Ok((input, Self(inner)))
    }

    fn write<W: Write>(&self, f: &mut W) -> std::io::Result<()> {
        self.0.write(f)
    }
}
impl PPrintable for WindowsTime {
    fn pprint(&self, printer: &mut Printer) {
        let epoch = chrono::Utc.ymd(1601, 1, 1).and_hms(0, 0, 0);
        let ts = epoch + Duration::seconds(self.0 / 10_000_000);
        printer.print(&format!("{}", ts.format("%Y-%m-%dT%H:%M:%S%z")));
    }
}

impl PPrintable for f32 {
    fn pprint(&self, printer: &mut Printer) {
        printer.print(&format!("{self}"));
    }
}
impl PPrintable for f64 {
    fn pprint(&self, printer: &mut Printer) {
        printer.print(&format!("{self}"));
    }
}

impl PPrintable for u64 {
    fn pprint(&self, printer: &mut Printer) {
        printer.print(&format!("{self}"));
    }
}
impl PPrintable for String {
    fn pprint(&self, printer: &mut Printer) {
        printer.print(&format!("{self}"));
    }
}
impl PPrintable for bool {
    fn pprint(&self, printer: &mut Printer) {
        printer.print(&format!("{self}"));
    }
}
impl PPrintable for &[u8] {
    fn pprint(&self, printer: &mut Printer) {
        printer.print(&format!("{self:02x?}"));
    }
}
impl<const N: usize> PPrintable for [u8; N] {
    fn pprint(&self, printer: &mut Printer) {
        printer.print(&format!("{self:02x?}"));
    }
}

trait Loggy {
    fn width(self) -> u32;
}
impl Loggy for usize {
    fn width(self) -> u32 {
        if self >= 10000000 {
            8
        } else if self >= 1000000 {
            7
        } else if self >= 100000 {
            6
        } else if self >= 10000 {
            5
        } else if self >= 1000 {
            4
        } else if self >= 100 {
            3
        } else if self >= 10 {
            2
        } else {
            1
        }
    }
}

pub struct Printer<'a> {
    margin: usize,
    game: &'a SRGame,
    out: Box<dyn std::io::Write>,
    new_line: bool,
    compare_mode: bool,
}

impl<'a> Printer<'a> {
    pub fn with_filename(mut self, outfile: &Path) -> std::io::Result<Self> {
        self.out = Box::new(BufWriter::new(File::create(outfile)?));
        Ok(self)
    }
    pub fn compare_mode(&mut self, enabled: bool) {
        self.compare_mode = enabled;
    }
    pub fn new(game: &'a SRGame) -> Self {
        Printer {
            margin: 0,
            game,
            new_line: true,
            out: Box::new(BufWriter::new(std::io::stdout().lock())),
            compare_mode: false,
        }
    }
    pub fn game(&self) -> &SRGame {
        self.game
    }
    fn ensure_nl(&mut self) {
        if self.new_line {
            write!(self.out, "{0:1$}", ' ', self.margin);
            self.new_line = false;
        }
    }
    pub fn print(&mut self, text: &str) {
        self.ensure_nl();
        self.out.write_all(text.as_bytes());
    }
    pub fn nl(&mut self) {
        if !self.new_line {
            writeln!(self.out);
            self.new_line = true;
        }
    }
    pub fn list<P, F: FnMut(&mut Self, &P)>(&mut self, items: &[P], mut f: F) {
        if items.is_empty() {
            self.print("<empty>");
        } else {
            self.margin += 2;
            let width = items.len().width();
            for (i, item) in items.iter().enumerate() {
                self.nl();
                self.ensure_nl();
                if self.compare_mode {
                    write!(self.out, "- ");
                } else {
                    write!(self.out, "[{i:>0$}] ", width as usize);
                }
                f(self, item);
            }
            self.margin -= 2;
        }
        self.nl();
    }
    pub fn map<
        'data,
        K: 'data,
        V: 'data,
        F: Fn(&mut Self, &'data (K, V)),
        I: IntoIterator<Item = &'data (K, V)>,
    >(
        &mut self,
        items: I,
        f: F,
    ) {
        self.margin += 2;
        let mut any = false;
        for item in items {
            any = true;
            self.nl();
            self.print("- ");
            f(self, item);
        }
        if !any {
            self.print("<empty>");
        }
        self.margin -= 2;
        self.nl();
    }

    pub fn object<F>(&mut self, object: &str, f: F)
    where
        F: FnOnce(&mut Self),
    {
        self.ensure_nl();
        self.print(&format!("{object} {{"));
        self.nl();
        self.margin += 2;
        f(self);
        self.margin -= 2;
        self.nl();
        self.print("}");
    }

    pub fn field(&mut self, name: &str) -> &mut Self {
        self.nl();
        self.ensure_nl();
        write!(self.out, "{name}: ");
        self
    }
    pub fn ufield(&mut self, name: &str) -> &mut Self {
        self.nl();
        self.ensure_nl();
        write!(self.out, "?{name}?: ");
        self
    }
    pub fn value<D: PPrintable>(&mut self, value: D) {
        self.ensure_nl();
        value.pprint(self);
    }
}

impl<A, B> Parseable for (A, B)
where
    A: Parseable,
    B: Parseable,
{
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        simple_parse!(input, a, b);
        Ok((input, (a, b)))
    }

    fn write<W: Write>(&self, f: &mut W) -> std::io::Result<()> {
        self.0.write(f)?;
        self.1.write(f)
    }
}

pub trait Parseable: Sized {
    fn parse(input: &[u8]) -> IResult<&[u8], Self>;

    fn write<W: Write>(&self, f: &mut W) -> std::io::Result<()>;
}

impl Parseable for String {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        lpstring(input)
    }

    fn write<W: Write>(&self, f: &mut W) -> std::io::Result<()> {
        f.write_all(&[self.len() as u8])?;
        f.write_all(self.as_bytes())
    }
}

impl<K, V> Parseable for VecMap<K, V>
where
    K: Parseable,
    V: Parseable,
{
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let (mut input, len) = nom::number::complete::le_i32(input)?;
        let mut rv = Vec::with_capacity(len as usize);
        for _ in 0..len {
            let (input2, key) = K::parse(input)?;
            input = input2;
            let (input2, value) = V::parse(input)?;
            input = input2;
            rv.push((key, value));
            input = nom::bytes::complete::tag([0x00, 0x20])(input)?.0;
        }
        Ok((input, VecMap(rv)))
    }

    fn write<W: Write>(&self, f: &mut W) -> std::io::Result<()> {
        f.write_all(&(self.0.len() as i32).to_le_bytes())?;
        for (k, v) in self.0.iter() {
            k.write(f)?;
            v.write(f)?;
            f.write_all(&[0x00, 0x20])?;
        }
        Ok(())
    }
}

impl Parseable for bool {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let (input, byte) = nom::bytes::complete::take(1 as usize)(input)?;
        Ok((input, byte[0] == 1))
    }

    fn write<W: Write>(&self, f: &mut W) -> std::io::Result<()> {
        f.write_all(&[if *self { 0x01 } else { 0x00 }])
    }
}

impl<const N: usize> Parseable for [u8; N] {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let (input, rv) = nom::bytes::complete::take(N)(input)?;
        Ok((input, s2a(rv).unwrap()))
    }

    fn write<W: Write>(&self, f: &mut W) -> std::io::Result<()> {
        f.write_all(self)
    }
}

impl Parseable for f64 {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        nom::number::complete::le_f64(input)
    }

    fn write<W: Write>(&self, f: &mut W) -> std::io::Result<()> {
        f.write_all(&self.to_le_bytes())
    }
}
impl Parseable for i64 {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        nom::number::complete::le_i64(input)
    }

    fn write<W: Write>(&self, f: &mut W) -> std::io::Result<()> {
        f.write_all(&self.to_le_bytes())
    }
}

impl Parseable for i32 {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        nom::number::complete::le_i32(input)
    }

    fn write<W: Write>(&self, f: &mut W) -> std::io::Result<()> {
        f.write_all(&self.to_le_bytes())
    }
}

impl Parseable for f32 {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        nom::number::complete::le_f32(input)
    }

    fn write<W: Write>(&self, f: &mut W) -> std::io::Result<()> {
        f.write_all(&self.to_le_bytes())
    }
}

impl<T> Parseable for Vec<T>
where
    T: Parseable,
{
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let (mut input, len) = nom::number::complete::le_i32(input)?;
        let mut rv = Vec::with_capacity(len as usize);
        for _ in 0..len {
            let (input2, value) = T::parse(input)?;
            input = input2;
            rv.push(value);
        }
        Ok((input, rv))
    }
    fn write<W: Write>(&self, f: &mut W) -> std::io::Result<()> {
        f.write_all(&(self.len() as i32).to_le_bytes())?;
        for value in self {
            value.write(f)?;
        }
        Ok(())
    }
}

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SimpleInventory {
    pub id: ItemId,
    pub count: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PlantId(usize);
impl Default for PlantId {
    fn default() -> Self {
        Self(usize::MAX)
    }
}
impl PPrintable for PlantId {
    fn pprint(&self, printer: &mut Printer) {
        let s = if self.0 != usize::MAX {
            printer.game().patch_index.patches[self.0].clone()
        } else {
            "<no plant>".into()
        };
        printer.print(&s);
    }
}
impl Parseable for PlantId {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let (input, value) = i32::parse(input)?;
        Ok((input, Self(value as usize)))
    }

    fn write<W: Write>(&self, f: &mut W) -> std::io::Result<()> {
        let value = self.0 as i32;
        value.write(f)
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct Plot {
    pub plot_type: PlotType,
    pub plant_type: PlantId,
    pub name: String,
    pub plot_inventories: HashMap<InventoryType, Vec<SimpleInventory>>,
    pub upgrades: Vec<i32>,
    pub tracked_actors: Vec<(f32, f32)>,
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

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ItemId(pub usize);
impl Parseable for ItemId {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let (input, value) = i32::parse(input)?;
        Ok((input, Self(value as usize)))
    }

    fn write<W: Write>(&self, f: &mut W) -> std::io::Result<()> {
        let value = self.0 as i32;
        value.write(f)
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SceneGroupId(pub usize);
impl Parseable for SceneGroupId {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let (input, value) = i32::parse(input)?;
        Ok((input, Self(value as usize)))
    }

    fn write<W: Write>(&self, f: &mut W) -> std::io::Result<()> {
        let value = self.0 as i32;
        value.write(f)
    }
}
impl PPrintable for SceneGroupId {
    fn pprint(&self, printer: &mut Printer) {
        let s = if self.0 != usize::MAX {
            printer.game().scene_group_index.scene_groups[self.0].clone()
        } else {
            "<no zone>".to_string()
        };
        printer.print(&s)
    }
}

fn lpstring(input: &[u8]) -> IResult<&[u8], String> {
    let orig_input = input;
    let (input, n) = nom::number::complete::le_u8(input)?;
    let (input, s) = nom::bytes::complete::take(n as usize)(input)?;
    Ok(std::str::from_utf8(s)
        .map(|s| (input, s.to_owned()))
        .unwrap_or_else(|e| {
            panic!(
                "{:?} was not a valid string: {e} ({} bytes from end)",
                s,
                orig_input.len()
            )
        }))
}
