use crate::{Parser, ParserInput};

pub struct SeparatedBy<Item, Separator> {
    item_parser: Item,
    separator_parser: Separator,
}

impl<Item, Separator> SeparatedBy<Item, Separator> {
    pub fn new(item_parser: Item, separator_parser: Separator) -> Self {
        Self {
            item_parser,
            separator_parser,
        }
    }
}

impl<Item, Separator> Parser for SeparatedBy<Item, Separator>
where
    Item: Parser,
    Separator: Parser,
{
    type Output = Vec<Item::Output>;

    fn parse_impl<I, S, O>(&mut self, iter: &mut I, skipper: &mut S, mut output: O) -> bool
    where
        I: ParserInput,
        S: Parser,
        O: FnMut(Self::Output),
    {
        skipper.skip(iter);

        let save = iter.clone();

        let mut first = None;

        if self
            .item_parser
            .parse_impl(iter, skipper, |x| first = Some(x))
            && first.is_some()
        {
            // Safety: first is checked and contains some
            let mut rest = vec![unsafe { first.unwrap_unchecked() }];
            loop {
                let save = iter.clone();
                if !self.separator_parser.parse_impl(iter, skipper, |_| ()) {
                    *iter = save;
                    break;
                }
                if !self.item_parser.parse_impl(iter, skipper, |x| rest.push(x)) {
                    *iter = save;
                    break;
                }
            }
            output(rest);
            true
        } else {
            *iter = save;
            false
        }
    }
}
