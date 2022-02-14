use crate::lexical::token::{Token, TokenType, ValidTokenType};
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
    pub fn parse(&self, tokens: Vec<Token>) -> Result<Tree<SymbolOrToken>, SyntaxError> {
        let mut tree: Tree<SymbolOrToken> = Tree::new();
        let mut stack: Vec<NodeId> = Vec::new();
        let mut token_index: usize = 0;
        let mut current_node: NodeId;
        let start_node_id = tree.insert_node(
            None,
            SymbolOrToken::Symbol(Symbol::NonTerminal(NonTerminal::Start)),
        );
        // helper function to handle a derivation hit in the table
        stack.push(start_node_id);
        while !stack.is_empty() {
            // ignore comments
            if token_index < tokens.len()
                && (matches!(
                    tokens[token_index].token_type,
                    TokenType::ValidTokenType(ValidTokenType::BlockCmt)
                ) || matches!(
                    tokens[token_index].token_type,
                    TokenType::ValidTokenType(ValidTokenType::InlineCmt)
                ))
            {
                continue;
            }
            if let SymbolOrToken::Symbol(Symbol::Terminal(Terminal::ValidTokenType(
                top_token_type,
            ))) = tree.get_node_value(*stack.last().unwrap())
            {
                if let TokenType::ValidTokenType(lookahead_token_type) =
                    &tokens[token_index].token_type
                {
                    if top_token_type.eq(lookahead_token_type) {
                        println!("match {}", lookahead_token_type);
                        current_node = stack.pop().unwrap();
                        tree.insert_node(
                            Some(current_node),
                            SymbolOrToken::Token(tokens[token_index].clone()),
                        );
                        token_index += 1;
                    }
                } else {
                    self.skip_error(&tokens[token_index]);
                }
            } else if let SymbolOrToken::Symbol(Symbol::NonTerminal(nonterminal)) =
                tree.get_node_value(*stack.last().unwrap())
            {
                if token_index >= tokens.len() {
                    match self
                        .parsing_table
                        .get(&(nonterminal.clone(), Terminal::EOF))
                    {
                        None => {}
                        Some(derivation) => {
                            self.write_derivation(derivation);
                            current_node = stack.pop().unwrap();
                            Self::handle_derivation(
                                &mut stack,
                                current_node.clone(),
                                derivation,
                                &mut tree,
                            );
                        }
                    }
                } else if let TokenType::ValidTokenType(valid_token_type) =
                    &tokens[token_index].token_type
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
                            Self::handle_derivation(
                                &mut stack,
                                current_node.clone(),
                                derivation,
                                &mut tree,
                            );
                        }
                    }
                }
            }
        }
        Ok(tree)
    }

    fn handle_derivation(
        stack: &mut Vec<NodeId>,
        current_node: NodeId,
        derivation: &Derivation,
        tree: &mut Tree<SymbolOrToken>,
    ) {
        for symbol in derivation.to.iter().rev() {
            // insert node and push into stack
            match symbol {
                Symbol::Terminal(terminal) => {
                    if matches!(terminal, Terminal::EPSILON) {
                    } else {
                        let node_id = tree.insert_node(
                            Some(current_node),
                            SymbolOrToken::Symbol(Symbol::Terminal(terminal.clone())),
                        );
                        stack.push(node_id);
                    }
                }
                Symbol::NonTerminal(nonterminal) => {
                    let node_id = tree.insert_node(
                        Some(current_node),
                        SymbolOrToken::Symbol(Symbol::NonTerminal(nonterminal.clone())),
                    );
                    stack.push(node_id);
                }
            }
        }
    }

    fn skip_error(&self, token: &Token) {
        println!("Skip token: {:?}", token)
    }

    fn write_derivation(&self, derivation: &Derivation) {
        println!("{}", derivation)
    }
}

#[derive(Debug)]
pub struct SyntaxError {}

impl Display for SyntaxError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Error for SyntaxError {}

#[derive(PartialEq)]
pub enum SymbolOrToken {
    Symbol(Symbol),
    Token(Token),
}
