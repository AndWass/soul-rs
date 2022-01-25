use crate::chars::Char;
use crate::Parser;

pub trait IntoParser
{
    type ParserType: Parser;

    fn into_parser(self) -> Self::ParserType;
}

impl IntoParser for char {
    type ParserType = Char;

    #[inline(always)]
    fn into_parser(self) -> Self::ParserType {
        Char(self)
    }
}

impl<P: Parser> IntoParser for P
{
    type ParserType = Self;

    #[inline(always)]
    fn into_parser(self) -> Self::ParserType {
        self
    }
}
