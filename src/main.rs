mod lexical;
mod syntactic;

use crate::syntactic::util::{read_first_follow_set_and_endable, read_parsing_table};
use lexical::lexer::Lexer;

fn main() {
    let mut lexer: Lexer = Lexer::new();
    read_parsing_table();
    read_first_follow_set_and_endable();
}
