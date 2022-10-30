use std::io::Write;

use nom::{combinator::map_res, IResult, error::{VerboseError, ContextError}};

use crate::{
    primitives::{InGameTime, ItemId, PlantId, SceneGroupId, TimeSinceYear1, VecMap, WindowsTime},
    util::s2a,
};

use super::Parseable;

impl Parseable for InGameTime {
    fn parse(input: &[u8]) -> IResult<&[u8], Self, VerboseError<&[u8]>> {
        let (input, inner) = f64::parse(input)?;
        Ok((input, Self(inner)))
    }

    fn write<W: Write>(&self, f: &mut W) -> std::io::Result<()> {
        self.0.write(f)
    }
}

impl Parseable for TimeSinceYear1 {
    fn parse(input: &[u8]) -> IResult<&[u8], Self, VerboseError<&[u8]>> {
        let (input, inner) = i64::parse(input)?;
        Ok((input, Self(inner)))
    }

    fn write<W: Write>(&self, f: &mut W) -> std::io::Result<()> {
        self.0.write(f)
    }
}

impl Parseable for WindowsTime {
    fn parse(input: &[u8]) -> IResult<&[u8], Self, VerboseError<&[u8]>> {
        let (input, inner) = i64::parse(input)?;
        Ok((input, Self(inner)))
    }

    fn write<W: Write>(&self, f: &mut W) -> std::io::Result<()> {
        self.0.write(f)
    }
}

impl Parseable for String {
    fn parse(input: &[u8]) -> IResult<&[u8], Self, VerboseError<&[u8]>> {
        let (input, n) = nom::number::complete::le_u8(input)?;
        nom::combinator::map(
            map_res(nom::bytes::complete::take(n as usize), |s| {
                std::str::from_utf8(s)
            }),
            |s| {
                s.to_string()
            },
        )(input)
            .map_err(|e| e.map(|e| ContextError::add_context(input, "Invalid UTF-8", e)))
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
    fn parse(input: &[u8]) -> IResult<&[u8], Self, VerboseError<&[u8]>> {
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
    fn parse(input: &[u8]) -> IResult<&[u8], Self, VerboseError<&[u8]>> {
        let (input, byte) = nom::bytes::complete::take(1 as usize)(input)?;
        Ok((input, byte[0] == 1))
    }

    fn write<W: Write>(&self, f: &mut W) -> std::io::Result<()> {
        f.write_all(&[if *self { 0x01 } else { 0x00 }])
    }
}

impl<const N: usize> Parseable for [u8; N] {
    fn parse(input: &[u8]) -> IResult<&[u8], Self, VerboseError<&[u8]>> {
        let (input, rv) = nom::bytes::complete::take(N)(input)?;
        Ok((input, s2a(rv).unwrap()))
    }

    fn write<W: Write>(&self, f: &mut W) -> std::io::Result<()> {
        f.write_all(self)
    }
}

impl Parseable for f64 {
    fn parse(input: &[u8]) -> IResult<&[u8], Self, VerboseError<&[u8]>> {
        nom::number::complete::le_f64(input)
    }

    fn write<W: Write>(&self, f: &mut W) -> std::io::Result<()> {
        f.write_all(&self.to_le_bytes())
    }
}
impl Parseable for i64 {
    fn parse(input: &[u8]) -> IResult<&[u8], Self, VerboseError<&[u8]>> {
        nom::number::complete::le_i64(input)
    }

    fn write<W: Write>(&self, f: &mut W) -> std::io::Result<()> {
        f.write_all(&self.to_le_bytes())
    }
}

impl Parseable for i32 {
    fn parse(input: &[u8]) -> IResult<&[u8], Self, VerboseError<&[u8]>> {
        nom::number::complete::le_i32(input)
    }

    fn write<W: Write>(&self, f: &mut W) -> std::io::Result<()> {
        f.write_all(&self.to_le_bytes())
    }
}

impl Parseable for f32 {
    fn parse(input: &[u8]) -> IResult<&[u8], Self, VerboseError<&[u8]>> {
        nom::number::complete::le_f32(input)
    }

    fn write<W: Write>(&self, f: &mut W) -> std::io::Result<()> {
        f.write_all(&self.to_le_bytes())
    }
}

// This only shows up the once (that I'm aware of), but should make the code a lot cleaner
impl<T> Parseable for Option<T>
where
    T: Parseable,
{
    fn parse(input: &[u8]) -> IResult<&[u8], Self, VerboseError<&[u8]>> {
        let (input, has_value) = bool::parse(input)?;
        if has_value {
            T::parse(input).map(|(input, value)| (input, Some(value)))
        } else {
            Ok((input, None))
        }
    }

    fn write<W: Write>(&self, f: &mut W) -> std::io::Result<()> {
        match self {
            Some(value) => {
                true.write(f)?;
                value.write(f)
            }
            None => false.write(f),
        }
    }
}

impl<T> Parseable for Vec<T>
where
    T: Parseable,
{
    fn parse(input: &[u8]) -> IResult<&[u8], Self, VerboseError<&[u8]>> {
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

macro_rules! parse_newtype {
    ($name:ty, $file_type:ty) => {
        impl Parseable for $name {
            fn parse(input: &[u8]) -> IResult<&[u8], Self, VerboseError<&[u8]>> {
                let (input, value) = <$file_type as Parseable>::parse(input)?;
                Ok((input, Self(value as _)))
            }

            fn write<W: Write>(&self, f: &mut W) -> std::io::Result<()> {
                let value = self.0 as i32;
                value.write(f)
            }
        }
    };
}

parse_newtype!(SceneGroupId, i32);
parse_newtype!(ItemId, i32);
parse_newtype!(PlantId, i32);
