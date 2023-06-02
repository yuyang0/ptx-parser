use nom::{
    branch::alt,
    Parser,
    sequence::preceded,
    IResult,
    combinator::opt
};

use super::{
    PtxFile,
    Function,
    Global,
    function::parse_function,
    global::parse_global,
    preamble::parse_preamble,
    comment::many1_comments_or_whitespace
};

#[derive(Debug)]
pub(crate) enum FunctionOrGlobal<'a> {
    Function(Function<'a>),
    Global(Global<'a>),
}

impl<'a> Iterator for PtxFile<'a> {
    type Item = IResult<&'a str, FunctionOrGlobal<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        let body = self.body?;
        Some(match alt((
            preceded(opt(many1_comments_or_whitespace), parse_function).map(FunctionOrGlobal::Function),
            preceded(opt(many1_comments_or_whitespace), parse_global).map(FunctionOrGlobal::Global),
        ))(body) {
            Ok((body, value)) => {
                self.body = Some(body);
                Ok((body, value))
            },
            err => {
                self.body = None;
                err
            },
        })
    }
}

impl<'a> TryFrom<&'a str> for PtxFile<'a> {
    type Error = nom::Err<nom::error::Error<&'a str>>;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let (body, preamble) = preceded(
            opt(many1_comments_or_whitespace), 
            parse_preamble
        )(value)?;
        Ok(PtxFile { preamble, body: Some(body) })
    }
}

#[cfg(feature = "std")]
#[cfg(test)]
mod test_iterator {
    use super::{PtxFile, FunctionOrGlobal};
    use crate::parser::{Function, Global};
    use crate::ptx_files::{_EXAMPLE_FILE, kernel, a, b, c, d};

    #[test]
    fn parse_example() {
        let ptx: PtxFile = _EXAMPLE_FILE.try_into().unwrap();
        println!("Preamble: {preamble:?}", preamble = ptx.preamble);
        for foo in ptx {
            println!("{foo:?}\n")
        }
    }

    #[test]
    fn parse_kernel() {
        let ptx: PtxFile = kernel::_PTX.try_into().unwrap();
        println!("Preamble: {preamble:?}", preamble = ptx.preamble);
        for foo in ptx {
            println!("{foo:?}\n")
        }
    }
}