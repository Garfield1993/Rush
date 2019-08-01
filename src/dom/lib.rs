extern crate dom;

mod node;
mod parser;

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(3, 3);
    }

    #[test]
    fn test_parse() {
        // This assert would fire and test will fail.
        // Please note, that private functions can be tested too!
        parser := new parser.Parser()
        assert_ne!(2, 3);
    }
}
