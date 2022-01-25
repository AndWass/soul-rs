use crate::{Parser, ParserInput};
use crate::traits::EmptyParser;

pub struct AnyChar;

impl Parser for AnyChar {
    type Output = char;

    fn parse_impl<I, S, O>(&mut self, iter: &mut I, skipper: &mut S, mut output: O) -> bool
    where
        I: ParserInput,
        S: Parser,
        O: FnMut(Self::Output),
    {
        skipper.skip(iter);
        let save = iter.clone();
        match utf8_decode::decode(iter) {
            Some(Ok(ch)) => {
                output(ch);
                true
            }
            _ => {
                *iter = save;
                false
            }
        }
    }
}

pub struct Char(pub char);

impl Parser for Char {
    type Output = char;

    fn parse_impl<I, S, O>(&mut self, iter: &mut I, skipper: &mut S, mut output: O) -> bool
    where
        I: ParserInput,
        S: Parser,
        O: FnMut(Self::Output),
    {
        skipper.skip(iter);

        let save = iter.clone();
        match utf8_decode::decode(iter) {
            Some(Ok(ch)) if ch == self.0 => {
                output(ch);
                true
            }
            _ => {
                *iter = save;
                false
            }
        }
    }
}

macro_rules! char_parser {
    ($name:ident, $func:ident) => {
        pub struct $name;

        impl Parser for $name {
            type Output = char;

            fn parse_impl<I, S, O>(&mut self, iter: &mut I, skipper: &mut S, mut output: O) -> bool
            where
                I: ParserInput,
                S: Parser,
                O: FnMut(Self::Output),
            {
                skipper.skip(iter);
                let mut parsed_char: char = 0u8.into();
                let save = iter.clone();
                match AnyChar.parse_impl(iter, &mut EmptyParser, |ch| parsed_char = ch) {
                    true if char::$func(&parsed_char) => {
                        output(parsed_char);
                        true
                    },
                    _ => {
                        *iter = save;
                        false
                    }
                }
            }
        }
    }
}

char_parser!(AsciiWhitespace, is_ascii_whitespace);
char_parser!(AsciiAlphaNumeric, is_ascii_alphanumeric);
char_parser!(AsciiDigit, is_ascii_digit);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::EmptyParser;
    use crate::Parser;

    #[test]
    fn any_char() {
        let mut parser = AnyChar;
        let data = [b'a', b'b'];
        let mut iter = data.iter().copied();

        let mut res = 'c';
        assert!(parser.parse_impl(&mut iter, &mut EmptyParser, |ch| res = ch));
        assert_eq!(res, 'a');
        res = 'd';
        assert!(parser.parse_impl(&mut iter, &mut EmptyParser, |ch| res = ch));
        assert_eq!(res, 'b');
    }

    #[test]
    fn any_char_skipping() {
        let mut parser = AnyChar;
        let mut skipper = Char('b');

        let data = [b'b', b'b', b'a', b'b'];
        let mut iter = data.iter().copied();

        let mut res = 'c';
        assert!(parser.parse_impl(&mut iter, &mut skipper, |ch| res = ch));
        assert_eq!(res, 'a');
        res = 'd';
        assert!(!parser.parse_impl(&mut iter, &mut skipper, |ch| res = ch));
        assert_eq!(res, 'd');
    }

    #[test]
    fn char_() {
        let mut parser = Char('a');
        let data = [b'a', b'b'];
        let mut iter = data.iter().copied();

        let mut res = 'c';
        assert!(parser.parse_impl(&mut iter, &mut EmptyParser, |ch| res = ch));
        assert_eq!(res, 'a');
        res = 'd';
        assert!(!parser.parse_impl(&mut iter, &mut EmptyParser, |ch| res = ch));
        assert_eq!(res, 'd');
    }

    #[test]
    fn char_skipping() {
        let mut parser = Char('a');
        let mut skipper = Char('b');

        let data = [b'b', b'b', b'a', b'b'];
        let mut iter = data.iter().copied();

        let mut res = 'c';
        assert!(parser.parse_impl(&mut iter, &mut skipper, |ch| res = ch));
        assert_eq!(res, 'a');
        res = 'd';
        assert!(!parser.parse_impl(&mut iter, &mut skipper, |ch| res = ch));
        assert_eq!(res, 'd');
    }

    #[test]
    fn ascii_digit() {
        let data = [b' ', b'\n', b'1', b'b'];
        let mut iter = data.iter().copied();

        let mut res = 'c';
        assert!(AsciiDigit.parse_impl(&mut iter, &mut AsciiWhitespace, |ch| res = ch));
        assert_eq!(res, '1');
        res = 'd';
        assert!(!AsciiDigit.parse_impl(&mut iter, &mut EmptyParser, |ch| res = ch));
        assert_eq!(res, 'd');
        assert_eq!(iter.next(), Some(b'b'));
    }
}
