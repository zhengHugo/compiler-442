use crate::syntactic::symbol::{NonTerminal, Symbol};
use regex::Regex;
use std::fmt::{Display, Formatter};

pub struct Derivation {
    from: NonTerminal,
    pub to: Vec<Symbol>,
}

impl Derivation {
    pub fn new(derivation_string: &str) -> Derivation {
        // production[0] is the left-hand side of the production rule
        // production[1] is the right-hand side of the production rule
        let production: Vec<String> = Regex::new(r"→")
            .unwrap()
            .split(derivation_string)
            .map(|x| x.trim().to_string())
            .collect();
        let from_symbol = match Symbol::from_string(&*production[0]) {
            Symbol::NonTerminal(n) => n,
            Symbol::Terminal(_) => panic!("Unexpected symbol string"),
            Symbol::ActionSymbol(actionSymbol) => todo!(),
        };
        let right_symbols: Vec<Symbol> = Regex::new(r" ")
            .unwrap()
            .split(&*production[1])
            .map(|x| Symbol::from_string(x.trim()))
            .collect();

        Derivation {
            from: from_symbol,
            to: right_symbols,
        }
    }
}

impl Display for Derivation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} -> {}",
            self.from,
            self.to
                .iter()
                .map(|x| x.to_string())
                .reduce(|current, next| current + " " + &*next)
                .unwrap(),
        )
    }
}
