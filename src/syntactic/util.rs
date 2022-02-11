use crate::lexical::token::Token;
use crate::syntactic::non_terminal::NonTerminal;
use std::collections::HashMap;

fn csv_to_hash_map() {
    let mut table: HashMap<(NonTerminal, Token), Derivation> = HashMap::new();
}
