mod lexical;
mod syntactic;

use lexical::lexer::Lexer;
use syntactic::util::csv_to_hash_map;

fn main() {
    let mut lexer: Lexer = Lexer::new();
    csv_to_hash_map();
}
