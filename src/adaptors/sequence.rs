use crate::{Parser, ParserInput};

pub struct Sequence<First, Second> {
    first: First,
    second: Second,
}

impl<F, S> Sequence<F, S> {
    pub fn new(first: F, second: S) -> Self {
        Self { first, second }
    }
}

impl<First, Second> Parser for Sequence<First, Second>
where
    First: Parser,
    Second: Parser,
{
    type Output = (First::Output, Second::Output);

    fn parse_impl<I, S, O>(&mut self, iter: &mut I, skipper: &mut S, mut output: O) -> bool
    where
        I: ParserInput,
        S: Parser,
        O: FnMut(Self::Output),
    {
        skipper.skip(iter);
        let save = iter.clone();

        let mut first_out = None;

        if self
            .first
            .parse_impl(iter, skipper, |f| first_out = Some(f))
        {
            let mut second_out = None;
            if self
                .second
                .parse_impl(iter, skipper, |s| second_out = Some(s))
            {
                if let (Some(f), Some(s)) = (first_out, second_out) {
                    output((f, s));
                    return true;
                }
            }
        }

        *iter = save;
        false
    }
}

#[cfg(test)]
mod tests {
    use crate::adaptors::sequence::Sequence;
    use crate::chars::{AsciiWhitespace, Char};
    use crate::Parser;

    #[test]
    fn sequence() {
        let mut parser = Sequence::new(Char('a'), Char('b'));
        let mut output = None;
        let mut iter = [b'a', b'b', b'c'].into_iter();
        parser.parse_impl(&mut iter, &mut AsciiWhitespace, |x| output = Some(x));

        assert_eq!(output, Some(('a', 'b')));
        assert_eq!(iter.next(), Some(b'c'));
    }
}
