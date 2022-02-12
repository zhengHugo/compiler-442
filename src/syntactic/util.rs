use crate::syntactic::derivation::Derivation;
use crate::syntactic::symbol::{NonTerminal, Symbol, Terminal};
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn csv_to_hash_map() -> HashMap<(NonTerminal, Terminal), Derivation> {
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
                        if j > 1 {
                            // starting from the third colum
                            // first column is non-terminals
                            // second column is end of file, which will be handled separately
                            if let Symbol::Terminal(terminal) =
                                Symbol::from_string(cell).expect("Unexpected symbol string")
                            {
                                terminals.push(terminal);
                            }
                        }
                    }
                } else {
                    // starting from second line: build non-terminals
                    if let Symbol::NonTerminal(non_terminal) =
                        Symbol::from_string(&cells[0]).expect("Unexpected symbol string")
                    {
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
                        if j > 1 && !cell.eq("") {
                            table.insert(
                                (non_terminals[i - 1].clone(), terminals[j - 2].clone()),
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
