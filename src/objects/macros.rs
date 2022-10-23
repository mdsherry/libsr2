macro_rules! simple_obj {
    ($type_name:ty, $name:literal, $($field:ident),*) => {
        simple_obj!($type_name : 1, $name, $($field),*);
    };
    ($type_name:ty : $version:literal, $name:literal, $($field:ident),*) => {
        impl Obj for $type_name {
            const NAME: &'static str = $name;
            const VERSION: i32 = $version;

            fn parse_body(input: &[u8]) -> IResult<&[u8], Self> {
                $(
                    let (input, $field) = Parseable::parse(input)?;
                )*
                return Ok((input, Self { $($field),* }))
            }

            fn write_body<W: Write>(&self, f: &mut W) -> std::io::Result<()> {
                $(
                    self.$field.write(f)?;
                )*
                Ok(())
            }
        }
    }
}

macro_rules! simple_parse {
    ($input:ident, $($i:ident), *) => {
        $(
            let ($input, $i) = Parseable::parse($input)?;
        )*
    };
}

pub(crate) use simple_obj;
pub(crate) use simple_parse;
