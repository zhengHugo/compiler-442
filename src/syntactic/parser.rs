use crate::lexical::token::{Token, TokenType, ValidTokenType};
use crate::semantic::concept::{CompositeConcept, Concept};
use crate::syntactic::derivation::Derivation;
use crate::syntactic::symbol::{ActionSymbol, NonTerminal, Symbol, Terminal};
use crate::syntactic::tree::{NodeId, Tree};
use crate::syntactic::util;
use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::Result as IOResult;
use std::io::Write;

pub struct Parser {
    parsing_table: HashMap<(NonTerminal, Terminal), Derivation>,
    first_set: HashMap<NonTerminal, Vec<Terminal>>,
    follow_set: HashMap<NonTerminal, Vec<Terminal>>,
}

impl Parser {
    pub fn new() -> Self {
        let (first_set, follow_set, _) = util::read_first_follow_set_and_endable();
        Self {
            parsing_table: util::read_parsing_table(),
            first_set,
            follow_set,
        }
    }
    pub fn parse(&mut self, tokens: Vec<Token>) -> Result<Tree<SymbolOrToken>, SyntaxError> {
        // debug only
        let mut error_file = File::create("resource/syntax/outsyntaxerrors").unwrap();
        let mut derivation_file = File::create("resource/syntax/outderivations").unwrap();

        let mut parsing_tree: Tree<SymbolOrToken> = Tree::new();
        let mut parsing_stack: Vec<NodeId> = Vec::new();
        let mut ast: Tree<Concept> = Tree::new();
        let mut semantic_stack: Vec<NodeId> = Vec::new();
        let mut token_index: usize = 0;
        let mut current_node: NodeId;
        let mut outstanding_dot: bool = false;

        let start_node_id = parsing_tree.insert_node(
            None,
            SymbolOrToken::Symbol(Symbol::NonTerminal(NonTerminal::Start)),
        );
        // helper function to handle a derivation hit in the table
        parsing_stack.push(start_node_id);
        while !parsing_stack.is_empty() {
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
                token_index += 1;
                continue;
            }

            match parsing_tree.get_node_value(*parsing_stack.last().unwrap()) {
                SymbolOrToken::Symbol(symbol) => {
                    match symbol {
                        Symbol::Terminal(terminal) => {
                            if let Terminal::ValidTokenType(top_token_type) = terminal {
                                // parsing stack top is validTokenType: try to match token
                                if let TokenType::ValidTokenType(lookahead_token_type) =
                                    &tokens[token_index].token_type
                                {
                                    if top_token_type.eq(lookahead_token_type) {
                                        // match token
                                        self.write_match(
                                            &mut derivation_file,
                                            lookahead_token_type,
                                        );
                                        current_node = parsing_stack.pop().unwrap();
                                        parsing_tree.insert_node(
                                            Some(current_node),
                                            SymbolOrToken::Token(tokens[token_index].clone()),
                                        );
                                        token_index += 1;
                                    } else {
                                        self.skip_error(
                                            &tokens,
                                            &mut token_index,
                                            &mut parsing_stack,
                                            &parsing_tree,
                                            &mut error_file,
                                        );
                                    }
                                } else {
                                    // token_index += 1;
                                    self.skip_error(
                                        &tokens,
                                        &mut token_index,
                                        &mut parsing_stack,
                                        &parsing_tree,
                                        &mut error_file,
                                    );
                                }
                            }
                        }
                        Symbol::NonTerminal(nonterminal) => {
                            // parsing stack top is nonterminal: query parsing table
                            if token_index >= tokens.len() {
                                // end of token stream: try EOF
                                match self
                                    .parsing_table
                                    .get(&(nonterminal.clone(), Terminal::EOF))
                                {
                                    None => {
                                        self.skip_error(
                                            &tokens,
                                            &mut token_index,
                                            &mut parsing_stack,
                                            &parsing_tree,
                                            &mut error_file,
                                        );
                                    }
                                    Some(derivation) => {
                                        self.write_derivation(&mut derivation_file, derivation);
                                        current_node = parsing_stack.pop().unwrap();
                                        Self::handle_derivation(
                                            &mut parsing_stack,
                                            current_node.clone(),
                                            derivation,
                                            &mut parsing_tree,
                                        );
                                    }
                                }
                            } else if let TokenType::ValidTokenType(valid_token_type) =
                                &tokens[token_index].token_type
                            {
                                // get new derivation: push new symbols into the stack
                                match self.parsing_table.get(&(
                                    nonterminal.clone(),
                                    Terminal::ValidTokenType(valid_token_type.clone()),
                                )) {
                                    None => {
                                        // token_index += 1;
                                        self.skip_error(
                                            &tokens,
                                            &mut token_index,
                                            &mut parsing_stack,
                                            &parsing_tree,
                                            &mut error_file,
                                        );
                                    }
                                    Some(derivation) => {
                                        self.write_derivation(&mut derivation_file, derivation);
                                        current_node = parsing_stack.pop().unwrap();
                                        // insert node
                                        Self::handle_derivation(
                                            &mut parsing_stack,
                                            current_node.clone(),
                                            derivation,
                                            &mut parsing_tree,
                                        );
                                    }
                                }
                            }
                        }
                        Symbol::ActionSymbol(action_symbol) => {
                            current_node = parsing_stack.pop().unwrap();
                            Self::perform_semantic_action(
                                &token_index,
                                &tokens,
                                action_symbol,
                                &mut semantic_stack,
                                &mut ast,
                                &mut outstanding_dot,
                            )
                        }
                    }
                }
                SymbolOrToken::Token(_) => panic!("Token appear on the parsing stack"),
            }
        }
        println!("{}", ast);
        Ok(parsing_tree)
    }

    fn handle_derivation(
        parsing_stack: &mut Vec<NodeId>,
        current_node: NodeId,
        derivation: &Derivation,
        parsing_tree: &mut Tree<SymbolOrToken>,
    ) {
        for symbol in derivation.to.iter().rev() {
            // insert node and push into stack
            match symbol {
                Symbol::Terminal(terminal) => {
                    if matches!(terminal, Terminal::EPSILON) {
                    } else {
                        let node_id = parsing_tree.insert_node(
                            Some(current_node),
                            SymbolOrToken::Symbol(Symbol::Terminal(terminal.clone())),
                        );
                        parsing_stack.push(node_id);
                    }
                }
                Symbol::NonTerminal(nonterminal) => {
                    let node_id = parsing_tree.insert_node(
                        Some(current_node),
                        SymbolOrToken::Symbol(Symbol::NonTerminal(nonterminal.clone())),
                    );
                    parsing_stack.push(node_id);
                }
                Symbol::ActionSymbol(action_symbol) => {
                    let node_id = parsing_tree.insert_node(
                        Some(current_node),
                        SymbolOrToken::Symbol(Symbol::ActionSymbol(action_symbol.clone())),
                    );
                    parsing_stack.push(node_id);
                }
            }
        }
    }

    fn perform_semantic_action(
        token_index: &usize,
        tokens: &Vec<Token>,
        action_symbol: &ActionSymbol,
        semantic_stack: &mut Vec<NodeId>,
        ast: &mut Tree<Concept>,
        outstanding_dot: &mut bool,
    ) {
        println!("perform action {:?}", action_symbol);
        match action_symbol {
            ActionSymbol::A // id, floatLit, intLit
            | ActionSymbol::K // relOp
            | ActionSymbol::N // addOp
            | ActionSymbol::P // multOp
            | ActionSymbol::A5 // void
            | ActionSymbol::B6 => { // visibility
                let concept = Concept::from_terminal_token(tokens[token_index - 1].clone()).unwrap();
                let concept_node_id = ast.insert_node(None, concept);
                semantic_stack.push(concept_node_id);
            }
            ActionSymbol::B => { // dot
                let operand1 = semantic_stack.pop().unwrap();
                let operand2 = semantic_stack.pop().unwrap();
                let dot_concept = Concept::CompositeConcept(CompositeConcept::Dot);
                let dot_node_id = ast.insert_node(None, dot_concept);
                ast.move_node_under_prepend(operand1, Some(dot_node_id));
                ast.move_node_under_prepend(operand2, Some(dot_node_id));
                semantic_stack.push(dot_node_id);
            }
            ActionSymbol::C => { // indexList
                let index_list_concept = Concept::CompositeConcept(CompositeConcept::IndexList);
                let index_list_node_id = ast.insert_node(None, index_list_concept);
                while !ast.get_node_value(*semantic_stack.last().unwrap()).is_epsilon() {
                    let index_item_node_id = semantic_stack.pop().unwrap();
                    ast.move_node_under_prepend(index_item_node_id, Some(index_list_node_id));
                }
                // pop epsilon
                semantic_stack.pop();
                semantic_stack.push(index_list_node_id);
            }
            ActionSymbol::D => { // var
                let sub_concept1_id = semantic_stack.pop().unwrap();
                let sub_concept2_id = semantic_stack.pop().unwrap();
                let var_concept = Concept::CompositeConcept(CompositeConcept::Var);
                let var_concept_id = ast.insert_node(None, var_concept);
                ast.move_node_under_prepend(sub_concept1_id, Some(var_concept_id));
                ast.move_node_under_prepend(sub_concept2_id, Some(var_concept_id));
                semantic_stack.push(var_concept_id);
            }
            ActionSymbol::E => { // push epsilon
                let epsilon = Concept::create_epsilon();
                let epsilon_node_id = ast.insert_node(None, epsilon);
                semantic_stack.push(epsilon_node_id);
            }
            ActionSymbol::F => { // create dot if OSD
                if *outstanding_dot {
                    let operand1 = semantic_stack.pop().unwrap();
                    let operand2 = semantic_stack.pop().unwrap();
                    let dot_concept = Concept::CompositeConcept(CompositeConcept::Dot);
                    let dot_node_id = ast.insert_node(None, dot_concept);
                    ast.move_node_under_prepend(operand1, Some(dot_node_id));
                    ast.move_node_under_prepend(operand2, Some(dot_node_id));
                    semantic_stack.push(dot_node_id);
                    *outstanding_dot = false;
                }
            }
            ActionSymbol::G => { // set osd to true
                *outstanding_dot = true;
            }
            ActionSymbol::H => { // assign
                let sub_concept1_id = semantic_stack.pop().unwrap();
                let sub_concept2_id = semantic_stack.pop().unwrap();
                let assign_concept = Concept::CompositeConcept(CompositeConcept::Assign);
                let assign_concept_id = ast.insert_node(None, assign_concept);
                ast.move_node_under_prepend(sub_concept1_id, Some(assign_concept_id));
                ast.move_node_under_prepend(sub_concept2_id, Some(assign_concept_id));
                semantic_stack.push(assign_concept_id);
            }
            ActionSymbol::I => {}
            ActionSymbol::J => { // funcCall
                let sub_concept1_id = semantic_stack.pop().unwrap();
                let sub_concept2_id = semantic_stack.pop().unwrap();
                let func_call_concept = Concept::CompositeConcept(CompositeConcept::FuncCall);
                let func_call_concept_id = ast.insert_node(None, func_call_concept);
                ast.move_node_under_prepend(sub_concept1_id, Some(func_call_concept_id));
                ast.move_node_under_prepend(sub_concept2_id, Some(func_call_concept_id));
                semantic_stack.push(func_call_concept_id);
            }
            ActionSymbol::L => { // relExpr
                let sub_concept1_id = semantic_stack.pop().unwrap();
                let sub_concept2_id = semantic_stack.pop().unwrap();
                let sub_concept3_id = semantic_stack.pop().unwrap();
                let expr_id = ast.insert_node(None, Concept::CompositeConcept(CompositeConcept::RelExpr));
                ast.move_node_under_prepend(sub_concept1_id, Some(expr_id));
                ast.move_node_under_prepend(sub_concept2_id, Some(expr_id));
                ast.move_node_under_prepend(sub_concept3_id, Some(expr_id));
                semantic_stack.push(expr_id);
            }
            ActionSymbol::M => { // addExpr
                let sub_concept1_id = semantic_stack.pop().unwrap();
                let sub_concept2_id = semantic_stack.pop().unwrap();
                let sub_concept3_id = semantic_stack.pop().unwrap();
                let expr_id = ast.insert_node(None, Concept::CompositeConcept(CompositeConcept::AddExpr));
                ast.move_node_under_prepend(sub_concept1_id, Some(expr_id));
                ast.move_node_under_prepend(sub_concept2_id, Some(expr_id));
                ast.move_node_under_prepend(sub_concept3_id, Some(expr_id));
                semantic_stack.push(expr_id);
            }
            ActionSymbol::O => { // multExpr
                let sub_concept1_id = semantic_stack.pop().unwrap();
                let sub_concept2_id = semantic_stack.pop().unwrap();
                let sub_concept3_id = semantic_stack.pop().unwrap();
                let expr_id = ast.insert_node(None, Concept::CompositeConcept(CompositeConcept::MultExpr));
                ast.move_node_under_prepend(sub_concept1_id, Some(expr_id));
                ast.move_node_under_prepend(sub_concept2_id, Some(expr_id));
                ast.move_node_under_prepend(sub_concept3_id, Some(expr_id));
                semantic_stack.push(expr_id);
            }
            ActionSymbol::Q => { // notExpr
                let sub_concept_id = semantic_stack.pop().unwrap();
                let expr_id = ast.insert_node(None, Concept::CompositeConcept(CompositeConcept::NotExpr));
                ast.move_node_under_prepend(sub_concept_id, Some(expr_id));
                semantic_stack.push(expr_id);
            }
            ActionSymbol::R => { // signed expr
                let sub_concept1_id = semantic_stack.pop().unwrap();
                let sub_concept2_id = semantic_stack.pop().unwrap();
                let signed_expr = Concept::CompositeConcept(CompositeConcept::SignedExpr);
                let signed_expr_id = ast.insert_node(None, signed_expr);
                ast.move_node_under_prepend(sub_concept1_id, Some(signed_expr_id));
                ast.move_node_under_prepend(sub_concept2_id, Some(signed_expr_id));
                semantic_stack.push(signed_expr_id);
            }
            ActionSymbol::S => { // sign
                let concept = Concept::create_sign(tokens[token_index - 1].clone()).unwrap();
                let concept_node_id = ast.insert_node(None, concept);
                semantic_stack.push(concept_node_id);
            }
            ActionSymbol::T => { // if then else
                let sub_concept1_id = semantic_stack.pop().unwrap();
                let sub_concept2_id = semantic_stack.pop().unwrap();
                let sub_concept3_id = semantic_stack.pop().unwrap();
                let expr_id = ast.insert_node(None, Concept::CompositeConcept(CompositeConcept::IfThenElse));
                ast.move_node_under_prepend(sub_concept1_id, Some(expr_id));
                ast.move_node_under_prepend(sub_concept2_id, Some(expr_id));
                ast.move_node_under_prepend(sub_concept3_id, Some(expr_id));
                semantic_stack.push(expr_id);
            }
            ActionSymbol::U => { // read
                let sub_concept_id = semantic_stack.pop().unwrap();
                let expr_id = ast.insert_node(None, Concept::CompositeConcept(CompositeConcept::Read));
                ast.move_node_under_prepend(sub_concept_id, Some(expr_id));
                semantic_stack.push(expr_id);
            }
            ActionSymbol::V => { // return 
                let sub_concept_id = semantic_stack.pop().unwrap();
                let expr_id = ast.insert_node(None, Concept::CompositeConcept(CompositeConcept::Return));
                ast.move_node_under_prepend(sub_concept_id, Some(expr_id));
                semantic_stack.push(expr_id);
            }
            ActionSymbol::W => { // while
                let sub_concept1_id = semantic_stack.pop().unwrap();
                let sub_concept2_id = semantic_stack.pop().unwrap();
                let while_concept = Concept::CompositeConcept(CompositeConcept::While);
                let while_concept_id = ast.insert_node(None, while_concept);
                ast.move_node_under_prepend(sub_concept1_id, Some(while_concept_id));
                ast.move_node_under_prepend(sub_concept2_id, Some(while_concept_id));
                semantic_stack.push(while_concept_id);
            }
            ActionSymbol::X => { // statBlock
                let stmt_block_concept = Concept::CompositeConcept(CompositeConcept::StmtBlock);
                let stmt_block_concept_id = ast.insert_node(None, stmt_block_concept);
                while !ast.get_node_value(*semantic_stack.last().unwrap()).is_epsilon() {
                    let stmt_item_id = semantic_stack.pop().unwrap();
                    ast.move_node_under_prepend(stmt_item_id, Some(stmt_block_concept_id));
                }
                // pop epsilon
                semantic_stack.pop();
                semantic_stack.push(stmt_block_concept_id);
            }
            ActionSymbol::Y => { // write
                let sub_concept_id = semantic_stack.pop().unwrap();
                let write_id = ast.insert_node(None, Concept::CompositeConcept(CompositeConcept::Write));
                ast.move_node_under_prepend(sub_concept_id, Some(write_id));
                semantic_stack.push(write_id);
            }
            ActionSymbol::Z => { // aParams
                let a_params_concept = Concept::CompositeConcept(CompositeConcept::AParams);
                let a_params_concept_id = ast.insert_node(None, a_params_concept);
                while !ast.get_node_value(*semantic_stack.last().unwrap()).is_epsilon() {
                    let aparam = semantic_stack.pop().unwrap();
                    ast.move_node_under_prepend(aparam, Some(a_params_concept_id));
                }
                // pop epsilon
                semantic_stack.pop();
                semantic_stack.push(a_params_concept_id);
            }
            ActionSymbol::A1 => { // arraySizes
                let array_sizes_concept = Concept::CompositeConcept(CompositeConcept::ArraySizes);
                let array_sizes_concept_id = ast.insert_node(None, array_sizes_concept);
                while !ast.get_node_value(*semantic_stack.last().unwrap()).is_epsilon() {
                    let array_size_id = semantic_stack.pop().unwrap();
                    ast.move_node_under_prepend(array_size_id, Some(array_sizes_concept_id));
                }
                // pop epsilon
                semantic_stack.pop();
                semantic_stack.push(array_sizes_concept_id);
            }
            ActionSymbol::A2 => { // fParam
                let sub_concept1_id = semantic_stack.pop().unwrap();
                let sub_concept2_id = semantic_stack.pop().unwrap();
                let sub_concept3_id = semantic_stack.pop().unwrap();
                let f_param_id = ast.insert_node(None, Concept::CompositeConcept(CompositeConcept::FParam));
                ast.move_node_under_prepend(sub_concept1_id, Some(f_param_id));
                ast.move_node_under_prepend(sub_concept2_id, Some(f_param_id));
                ast.move_node_under_prepend(sub_concept3_id, Some(f_param_id));
                semantic_stack.push(f_param_id);
            }
            ActionSymbol::A3 => { // type
                let sub_concept_id = semantic_stack.pop().unwrap();
                let type_id = ast.insert_node(None, Concept::CompositeConcept(CompositeConcept::Type));
                ast.move_node_under_prepend(sub_concept_id, Some(type_id));
                semantic_stack.push(type_id);
            }
            ActionSymbol::A4 => { // fParams
                let f_params_concept = Concept::CompositeConcept(CompositeConcept::FParams);
                let f_params_concept_id = ast.insert_node(None, f_params_concept);
                while !ast.get_node_value(*semantic_stack.last().unwrap()).is_epsilon() {
                    let f_param_id = semantic_stack.pop().unwrap();
                    ast.move_node_under_prepend(f_param_id, Some(f_params_concept_id));
                }
                // pop epsilon
                semantic_stack.pop();
                semantic_stack.push(f_params_concept_id);
            }
            ActionSymbol::A6 => { // funcDef
                let sub_concept1_id = semantic_stack.pop().unwrap();
                let sub_concept2_id = semantic_stack.pop().unwrap();
                let sub_concept3_id = semantic_stack.pop().unwrap();
                let sub_concept4_id = semantic_stack.pop().unwrap();
                let func_def_id = ast.insert_node(None, Concept::CompositeConcept(CompositeConcept::FuncDef));
                ast.move_node_under_prepend(sub_concept1_id, Some(func_def_id));
                ast.move_node_under_prepend(sub_concept2_id, Some(func_def_id));
                ast.move_node_under_prepend(sub_concept3_id, Some(func_def_id));
                ast.move_node_under_prepend(sub_concept4_id, Some(func_def_id));
                semantic_stack.push(func_def_id);
            }
            ActionSymbol::A7 => { // varDecl
                let sub_concept1_id = semantic_stack.pop().unwrap();
                let sub_concept2_id = semantic_stack.pop().unwrap();
                let sub_concept3_id = semantic_stack.pop().unwrap();
                let var_decl_id = ast.insert_node(None, Concept::CompositeConcept(CompositeConcept::VarDecl));
                ast.move_node_under_prepend(sub_concept1_id, Some(var_decl_id));
                ast.move_node_under_prepend(sub_concept2_id, Some(var_decl_id));
                ast.move_node_under_prepend(sub_concept3_id, Some(var_decl_id));
                semantic_stack.push(var_decl_id);
            }
            ActionSymbol::A8 => { // funcBody
                let func_body_id = ast.insert_node(None, Concept::CompositeConcept(CompositeConcept::FuncBody));
                while !ast.get_node_value(*semantic_stack.last().unwrap()).is_epsilon() {
                    let item_id = semantic_stack.pop().unwrap();
                    ast.move_node_under_prepend(item_id, Some(func_body_id));
                }
                // pop epsilon
                semantic_stack.pop();
                semantic_stack.push(func_body_id);
            }
            ActionSymbol::A9 => { // funcDecl
                let sub_concept1_id = semantic_stack.pop().unwrap();
                let sub_concept2_id = semantic_stack.pop().unwrap();
                let sub_concept3_id = semantic_stack.pop().unwrap();
                let func_decl_id = ast.insert_node(None, Concept::CompositeConcept(CompositeConcept::FuncDecl));
                ast.move_node_under_prepend(sub_concept1_id, Some(func_decl_id));
                ast.move_node_under_prepend(sub_concept2_id, Some(func_decl_id));
                ast.move_node_under_prepend(sub_concept3_id, Some(func_decl_id));
                semantic_stack.push(func_decl_id);
            }
            ActionSymbol::B1 => { // funcDefList
                let func_def_list_id = ast.insert_node(None, Concept::CompositeConcept(CompositeConcept::FuncDefList));
                while !ast.get_node_value(*semantic_stack.last().unwrap()).is_epsilon() {
                    let func_def_id = semantic_stack.pop().unwrap();
                    ast.move_node_under_prepend(func_def_id, Some(func_def_list_id));
                }
                // pop epsilon
                semantic_stack.pop();
                semantic_stack.push(func_def_list_id);
            }
            ActionSymbol::B2 => { // implDef
                let sub_concept1_id = semantic_stack.pop().unwrap();
                let sub_concept2_id = semantic_stack.pop().unwrap();
                let impl_def_id = ast.insert_node(None, Concept::CompositeConcept(CompositeConcept::ImplDef));
                ast.move_node_under_prepend(sub_concept1_id, Some(impl_def_id));
                ast.move_node_under_prepend(sub_concept2_id, Some(impl_def_id));
                semantic_stack.push(impl_def_id);
            }
            ActionSymbol::B3 => { // structDecl
                let sub_concept1_id = semantic_stack.pop().unwrap();
                let sub_concept2_id = semantic_stack.pop().unwrap();
                let sub_concept3_id = semantic_stack.pop().unwrap();
                let struct_decl_id = ast.insert_node(None, Concept::CompositeConcept(CompositeConcept::StructDecl));
                ast.move_node_under_prepend(sub_concept1_id, Some(struct_decl_id));
                ast.move_node_under_prepend(sub_concept2_id, Some(struct_decl_id));
                ast.move_node_under_prepend(sub_concept3_id, Some(struct_decl_id));
                semantic_stack.push(struct_decl_id);
            }
            ActionSymbol::B4 => { // inheritsList
                let inherits_list_id = ast.insert_node(None, Concept::CompositeConcept(CompositeConcept::InheritsList));
                while !ast.get_node_value(*semantic_stack.last().unwrap()).is_epsilon() {
                    let inherits_item_id = semantic_stack.pop().unwrap();
                    ast.move_node_under_prepend(inherits_item_id, Some(inherits_list_id));
                }
                // pop epsilon
                semantic_stack.pop();
                semantic_stack.push(inherits_list_id);
            }
            ActionSymbol::B5 => { // structMemberDeclList
                let struct_mem_decl_list_id = ast.insert_node(None, Concept::CompositeConcept(CompositeConcept::StructMemberDeclList));
                while !ast.get_node_value(*semantic_stack.last().unwrap()).is_epsilon() {
                    let struct_mem_decl_id = semantic_stack.pop().unwrap();
                    ast.move_node_under_prepend(struct_mem_decl_id, Some(struct_mem_decl_list_id));
                }
                // pop epsilon
                semantic_stack.pop();
                semantic_stack.push(struct_mem_decl_list_id);
            }
            ActionSymbol::B7 => { // structMemberDecl
                let sub_concept1_id = semantic_stack.pop().unwrap();
                let sub_concept2_id = semantic_stack.pop().unwrap();
                let struct_mem_decl_id = ast.insert_node(None, Concept::CompositeConcept(CompositeConcept::StructMemberDecl));
                ast.move_node_under_prepend(sub_concept1_id, Some(struct_mem_decl_id));
                ast.move_node_under_prepend(sub_concept2_id, Some(struct_mem_decl_id));
                semantic_stack.push(struct_mem_decl_id);
            }
            ActionSymbol::B8 => { // prog
                let prog_id = ast.insert_node(None, Concept::CompositeConcept(CompositeConcept::Prog));
                while !ast.get_node_value(*semantic_stack.last().unwrap()).is_epsilon() {
                    let item_id = semantic_stack.pop().unwrap();
                    ast.move_node_under_prepend(item_id, Some(prog_id));
                }
                // pop epsilon
                semantic_stack.pop();
                semantic_stack.push(prog_id);
            }
        }
    }

    fn skip_error(
        &self,
        tokens: &Vec<Token>,
        token_index: &mut usize,
        stack: &mut Vec<NodeId>,
        tree: &Tree<SymbolOrToken>,
        error_file: &mut File,
    ) {
        error_file.write_all(
            format!(
                "Syntax error at line {}: unexpected token {}\n",
                tokens[*token_index].location.0,
                tokens[*token_index].get_valid_token_type().unwrap()
            )
            .as_ref(),
        );
        match tree.get_node_value(*stack.last().unwrap()) {
            SymbolOrToken::Symbol(Symbol::Terminal(top)) => {
                // terminal on the stack top
                error_file.write_all(format!("Expect token {}\n", top).as_ref());
                stack.pop();
            }
            SymbolOrToken::Symbol(Symbol::NonTerminal(top)) => {
                let mut lookahead_token_type = tokens[*token_index].get_valid_token_type().unwrap();
                if self
                    .follow_set
                    .get(top)
                    .unwrap()
                    .contains(&Terminal::ValidTokenType(lookahead_token_type))
                {
                    error_file.write_all(format!("Expect nonterminal {}\n", top).as_ref());
                    stack.pop();
                } else {
                    while (!self
                        .first_set
                        .get(top)
                        .unwrap()
                        .contains(&Terminal::ValidTokenType(lookahead_token_type)))
                        || (self
                            .first_set
                            .get(top)
                            .unwrap()
                            .contains(&Terminal::EPSILON)
                            && !self
                                .follow_set
                                .get(top)
                                .unwrap()
                                .contains(&Terminal::ValidTokenType(lookahead_token_type)))
                    {
                        error_file
                            .write_all(format!("Skip token: {}\n", lookahead_token_type).as_ref());
                        *token_index += 1;
                        lookahead_token_type = tokens[*token_index].get_valid_token_type().unwrap();
                    }
                }
            }
            _ => panic!("Trying to skip a matched token. Should not reach here!"),
        }
    }

    fn write_derivation(
        &self,
        derivation_file: &mut File,
        derivation: &Derivation,
    ) -> IOResult<()> {
        // derivation_file.write_all(format!("{}\n", derivation).as_ref())
        println!("{}", derivation);
        Ok(())
    }

    fn write_match(&self, derivation_file: &mut File, lookahead: &ValidTokenType) -> IOResult<()> {
        // derivation_file.write_all(format!("match {}\n", lookahead).as_ref())
        println!("match {}", lookahead);
        Ok(())
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

impl SymbolOrToken {
    fn get_token(&self) -> Token {
        match self {
            SymbolOrToken::Symbol(_) => panic!("Called SymbolOrToken::get_token on a Symbol"),
            SymbolOrToken::Token(token) => token.clone(),
        }
    }
}

impl Display for SymbolOrToken {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                SymbolOrToken::Symbol(symbol) => symbol.to_string(),
                SymbolOrToken::Token(token) => token.to_string(),
            }
        )
    }
}
