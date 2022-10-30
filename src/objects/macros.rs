macro_rules! simple_obj {
    ($type_name:ty, $name:literal, $($field:ident),*) => {
        simple_obj!($type_name : 1, $name, $($field),*);
    };
    ($type_name:ty : $version:literal, $name:literal, $($field:ident),*) => {
        impl $crate::objects::Obj for $type_name {
            const NAME: &'static str = $name;
            const VERSION: i32 = $version;

            fn parse_body(input: &[u8]) -> nom::IResult<&[u8], Self, nom::error::VerboseError<&[u8]>> {
                $(
                    let (input, $field) = $crate::parsers::Parseable::parse(input)
                        .map_err(|e| e.map(|e| nom::error::ContextError::add_context(input, concat!("Parsing field '", stringify!($field), "'"), e)))?;
                )*
                return Ok((input, Self { $($field),* }))
            }

            fn write_body<W: std::io::Write>(&self, f: &mut W) -> std::io::Result<()> {
                $(
                    $crate::parsers::Parseable::write(&self.$field, f)?;
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
