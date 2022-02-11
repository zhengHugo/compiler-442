mod lexical;
mod syntactic;

use lexical::lexer::Lexer;
use lexical::lexer_machine_impl::LexerStateMachineImpl;
use lexical::lexer_machine_impl::State;
use lexical::lexical_error::LexicalError;
use lexical::token::TokenType;
use std::fs;
use std::fs::File;
use std::io::Write;

fn main() -> std::io::Result<()> {
    let input_files_prefix = ["sample", "lexnegativegrading", "lexpositivegrading"];
    for input_file_prefix in input_files_prefix {
        let source: String = fs::read_to_string(input_file_prefix.to_owned() + ".src")
            .expect("Something went wrong reading the file");
        let mut out_lex_tokens_file = File::create(input_file_prefix.to_owned() + ".outlextokens")?;
        let mut out_lex_errors_file = File::create(input_file_prefix.to_owned() + ".outlexerrors")?;
        let mut lexer: Lexer = Lexer::new();
        lexer.read_source(&source);
        while let Some(token) = lexer.next_token() {
            out_lex_tokens_file
                .write_all((token.clone().to_string() + "\n").as_bytes())
                .expect("Something went wrong writing the file");
            match token.token_type {
                TokenType::InvalidTokenType(invalid_token_type) => {
                    out_lex_errors_file
                        .write_all(
                            (format!(
                                "Lexical error: {}: \"{}\": line {}.\n",
                                invalid_token_type, token.lexeme, token.location.0
                            ))
                            .as_bytes(),
                        )
                        .expect("Something went wrong writing the file");
                }
                TokenType::ValidTokenType(_) => (),
            }
        }
    }
    Ok(())
}
