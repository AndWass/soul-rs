pub mod traits;
pub mod parsers;
pub mod adaptors;

pub use traits::ParserInput;
pub use traits::Parser;
pub use traits::ParserExt;

pub use parsers::chars;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
