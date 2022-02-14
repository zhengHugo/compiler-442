use crate::syntactic::derivation::Derivation;
use crate::syntactic::symbol::{NonTerminal, Symbol, Terminal};
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn read_parsing_table() -> HashMap<(NonTerminal, Terminal), Derivation> {
    // let mut table: HashMap<(NonTerminal, ValidTokenType), Derivation> = HashMap::new();
    let mut table = HashMap::new();
    let mut terminals: Vec<Terminal> = Vec::new();
    let mut non_terminals: Vec<NonTerminal> = Vec::new();
    if let Ok(lines) = read_lines("resource/syntax/LL(1) Parsing Table.csv") {
        // 1. build terminals and non_terminals
        for (i, line_result) in lines.enumerate() {
            if let Ok(line) = line_result {
                let cells = split_string(&*line, r",");
                if i == 0 {
                    // first line: build terminal vector starting from the third column
                    for (j, cell) in cells.iter().enumerate() {
                        if j > 0 {
                            // starting from the second colum
                            // first column is non-terminals
                            if let Symbol::Terminal(terminal) = Symbol::from_string(cell) {
                                terminals.push(terminal);
                            }
                        }
                    }
                } else {
                    // starting from second line: build non-terminals
                    if let Symbol::NonTerminal(non_terminal) = Symbol::from_string(&cells[0]) {
                        non_terminals.push(non_terminal);
                    }
                }
            }
        }
    }

    // separating table key building from table value filling to keep the rust compiler happy :-)
    if let Ok(lines) = read_lines("resource/syntax/LL(1) Parsing Table.csv") {
        for (i, line_result) in lines.enumerate() {
            if let Ok(line) = line_result {
                let cells = split_string(&*line, r",");
                if i > 0 {
                    for (j, cell) in cells.iter().enumerate() {
                        if j > 0 && !cell.eq("") {
                            table.insert(
                                (non_terminals[i - 1].clone(), terminals[j - 1].clone()),
                                Derivation::new(cell),
                            );
                        }
                    }
                }
            }
        }
    }
    table
}

pub fn read_first_follow_set_and_endable() -> (
    HashMap<NonTerminal, Vec<Terminal>>,
    HashMap<NonTerminal, Vec<Terminal>>,
    HashMap<NonTerminal, bool>,
) {
    let mut first_set = HashMap::new();
    let mut follow_set = HashMap::new();
    let mut endable = HashMap::new();
    if let Ok(lines) = read_lines("resource/syntax/fst_flw.csv") {
        for (i, line_result) in lines.enumerate() {
            if let Ok(line) = line_result {
                if i > 0 {
                    let cells = split_string(&*line, r",");
                    if let Symbol::NonTerminal(key) = Symbol::from_string(&cells[0]) {
                        let mut first_terminals: Vec<Terminal> = Regex::new(r" ")
                            .unwrap()
                            .split(&cells[1])
                            .map(|x| match Symbol::from_string(x.trim()) {
                                Symbol::NonTerminal(_) => {
                                    panic!("Unexpected nonterminal in set table")
                                }
                                Symbol::Terminal(terminal) => terminal,
                            })
                            .collect();
                        if cells[3].eq("yes") {
                            first_terminals.push(Terminal::EPSILON);
                        }
                        first_set.insert(key.clone(), first_terminals);

                        // build follow set
                        let follow_terminals: Vec<Terminal> = Regex::new(r" ")
                            .unwrap()
                            .split(&cells[2])
                            .filter(|x| !(*x).eq("∅"))
                            .map(|x| match Symbol::from_string(x.trim()) {
                                Symbol::NonTerminal(_) => {
                                    panic!("Unexpected nonterminal in set table")
                                }
                                Symbol::Terminal(terminal) => terminal,
                            })
                            .collect();
                        follow_set.insert(key.clone(), follow_terminals);

                        // build endable
                        if cells[4].eq("yes") {
                            endable.insert(key.clone(), true);
                        }
                    }

                    // build first set
                }
            }
        }
    }
    (first_set, follow_set, endable)
}
fn split_string(text: &str, regex: &str) -> Vec<String> {
    Regex::new(regex)
        .unwrap()
        .split(text)
        .map(|x| x.trim().to_string())
        .collect::<Vec<String>>()
}

fn read_lines<P>(file_name: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(file_name)?;
    Ok(io::BufReader::new(file).lines())
}
