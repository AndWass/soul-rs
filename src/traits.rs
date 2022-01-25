pub mod parser_ext;
pub mod into_parser;

pub use parser_ext::ParserExt;
pub use into_parser::IntoParser;

mod sealed {
    pub trait Sealed {}

    impl<T: Iterator<Item = u8>> Sealed for T {}
}

pub trait ParserInput: Clone + Iterator<Item = u8> + sealed::Sealed {}

impl<T: Clone + Iterator<Item = u8>> ParserInput for T {}

pub trait Parser {
    type Output;

    /// Implements the parser logic
    ///
    /// A well-formed parser implementation should call `skipper.skip(iter)` before it
    /// attempts to perform the parsing logic.
    fn parse_impl<I, S, O>(&mut self, iter: &mut I, skipper: &mut S, output: O) -> bool
    where
        I: ParserInput,
        S: Parser,
        O: FnMut(Self::Output);

    fn skip<I: ParserInput>(&mut self, iter: &mut I) {
        while self.parse_impl(iter, &mut EmptyParser, |_| ()) {}
    }

    fn phrase_parse<I: ParserInput, O: FnMut(Self::Output)>(
        &mut self,
        iter: &mut I,
        output: O,
    ) -> bool {
        self.parse_impl(iter, &mut crate::parsers::chars::AsciiWhitespace, output)
    }

    fn parse_slice<S: Parser, O: FnMut(Self::Output)>(
        &mut self,
        data: &[u8],
        skipper: &mut S,
        output: O,
    ) -> bool {
        self.parse_impl(&mut data.iter().copied(), skipper, output)
    }
}

/// An empty parser that never succeeds in parsing anything
///
/// Main usage is as a skipper when you never want to skip anything.
pub struct EmptyParser;

impl Parser for EmptyParser {
    type Output = ();

    fn parse_impl<I, S, O>(&mut self, _iter: &mut I, _skipper: &mut S, _output: O) -> bool
    where
        I: ParserInput,
        S: Parser,
        O: FnMut(Self::Output),
    {
        false
    }
}
