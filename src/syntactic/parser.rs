use crate::lexical::token::{Token, TokenType};
use crate::syntactic::derivation::Derivation;
use crate::syntactic::symbol::{NonTerminal, Symbol, Terminal};
use crate::syntactic::tree::{NodeId, Tree};
use crate::syntactic::util;
use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Display, Formatter};

pub struct Parser {
    parsing_table: HashMap<(NonTerminal, Terminal), Derivation>,
    first_set: HashMap<NonTerminal, Vec<Terminal>>,
    follow_set: HashMap<NonTerminal, Vec<Terminal>>,
    endable: HashMap<NonTerminal, bool>,
}

impl Parser {
    pub fn new() -> Self {
        let (first_set, follow_set, endable) = util::read_first_follow_set_and_endable();
        Self {
            parsing_table: util::read_parsing_table(),
            first_set,
            follow_set,
            endable,
        }
    }
    fn parse(&self, tokens: Vec<Token>) -> Result<Tree<TokenOrNonTerminal>, SyntaxError> {
        let mut tree: Tree<TokenOrNonTerminal> = Tree::new();
        let mut stack: Vec<NodeId> = Vec::new();
        let mut token_index: usize = 0;
        let mut current_node: NodeId;
        let start_node_id =
            tree.insert_node(None, TokenOrNonTerminal::NonTerminal(NonTerminal::Start));
        stack.push(start_node_id);
        while !stack.is_empty() {
            if token_index >= tokens.len() {
                // TODO: end of file
            } else if let TokenOrNonTerminal::Token(top_token) =
                tree.get_node_value(*stack.last().unwrap())
            {
                if tokens[token_index].eq(top_token) {
                    current_node = stack.pop().unwrap();
                    token_index += 1;
                } else {
                    self.skip_error(&tokens[token_index]);
                }
            } else if let TokenOrNonTerminal::NonTerminal(nonterminal) =
                tree.get_node_value(*stack.last().unwrap())
            {
                if let TokenType::ValidTokenType(valid_token_type) = &tokens[token_index].token_type
                {
                    // pushing new symbols into the stack
                    match self.parsing_table.get(&(
                        nonterminal.clone(),
                        Terminal::ValidTokenType(valid_token_type.clone()),
                    )) {
                        None => self.skip_error(&tokens[token_index]),
                        Some(derivation) => {
                            self.write_derivation(derivation);
                            current_node = stack.pop().unwrap();
                            // insert node
                            for symbol in derivation.to.iter().rev() {
                                // insert node and push into stack
                                match symbol {
                                    Symbol::Terminal(terminal) => {
                                        if matches!(terminal, Terminal::EPSILON) {
                                        } else {
                                            let node_id = tree.insert_node(
                                                Some(current_node),
                                                TokenOrNonTerminal::Token(
                                                    tokens[token_index].clone(),
                                                ),
                                            );
                                            stack.push(node_id);
                                        }
                                    }
                                    Symbol::NonTerminal(nonterminal) => {
                                        let node_id = tree.insert_node(
                                            Some(current_node),
                                            TokenOrNonTerminal::NonTerminal(nonterminal.clone()),
                                        );
                                        stack.push(node_id);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        Ok(tree)
    }

    fn skip_error(&self, token: &Token) {}

    fn write_derivation(&self, derivation: &Derivation) {}
}

#[derive(Debug)]
struct SyntaxError {}

impl Display for SyntaxError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Error for SyntaxError {}

#[derive(PartialEq)]
enum TokenOrNonTerminal {
    Token(Token),
    NonTerminal(NonTerminal),
}
