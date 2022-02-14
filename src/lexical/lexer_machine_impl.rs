use crate::lexical::token::{InvalidTokenType, TokenType, ValidTokenType};
use rust_fsm::StateMachineImpl;

#[derive(Debug)]
pub enum State {
    Start,

    // comment
    InlineCmt,
    BlockCmt,

    // atomic lexical elements
    Id2,
    Str2,
    Str3,
    Int12,
    Int13,
    Int21,
    Int22,
    Int23,
    Int31,
    Int32,
    Int33,
    Frac12,
    Frac13,
    Frac14,
    Frac15,

    // operators
    Eq,
    NotEq,
    Lt,
    Gt,
    Leq,
    Geq,
    Plus,
    Minus,
    Mult,
    Div,
    Assign,
    Or,
    And,
    Not,
    OpenPar,
    ClosePar,
    OpenCuBr,
    CloseCuBr,
    OpenSqBr,
    CloseSqBr,
    Semi,
    Comma,
    Dot,
    Colon,
    ColonColon,
    Arrow,

    // key words
    // if, integer, inherits, impl
    KwI,

    KwIf,

    KwIn,
    KwInt,
    KwInte,
    KwInteg,
    KwIntege,
    KwInteger,

    KwInh,
    KwInhe,
    KwInher,
    KwInheri,
    KwInherit,
    KwInherits,

    KwIm,
    KwImp,
    KwImpl,

    // then
    KwT,
    KwTh,
    KwThe,
    KwThen,

    // else
    KwE,
    KwEl,
    KwEls,
    KwElse,

    // float, func
    KwF,
    KwFl,
    KwFlo,
    KwFloa,
    KwFloat,

    KwFu,
    KwFun,
    KwFunc,

    // void, var
    KwV,
    KwVo,
    KwVoi,
    KwVoid,
    KwVa,
    KwVar,

    // public, private
    KwP,
    KwPu,
    KwPub,
    KwPubl,
    KwPubli,
    KwPublic,

    KwPr,
    KwPri,
    KwPriv,
    KwPriva,
    KwPrivat,
    KwPrivate,

    // struct, self
    KwS,
    KwSt,
    KwStr,
    KwStru,
    KwStruc,
    KwStruct,

    KwSe,
    KwSel,
    KwSelf,

    // while, write
    KwW,
    KwWh,
    KwWhi,
    KwWhil,
    KwWhile,

    KwWr,
    KwWri,
    KwWrit,
    KwWrite,

    // read, return
    KwR,
    KwRe,
    KwRea,
    KwRead,
    KwRet,
    KwRetu,
    KwRetur,
    KwReturn,

    // let
    KwL,
    KwLe,
    KwLet,
}
pub struct LexerStateMachineImpl {}

impl LexerStateMachineImpl {
    /// Return a token type based on the current machine state
    pub(crate) fn state_to_token_type(
        state: &<LexerStateMachineImpl as StateMachineImpl>::State,
    ) -> TokenType {
        let valid_token_type = match state {
            // operators
            State::Eq => Some(ValidTokenType::Eq),
            State::NotEq => Some(ValidTokenType::NotEq),
            State::Lt => Some(ValidTokenType::Lt),
            State::Gt => Some(ValidTokenType::Gt),
            State::Leq => Some(ValidTokenType::Leq),
            State::Geq => Some(ValidTokenType::Geq),
            State::Plus => Some(ValidTokenType::Plus),
            State::Minus => Some(ValidTokenType::Minus),
            State::Mult => Some(ValidTokenType::Mult),
            State::Div => Some(ValidTokenType::Div),
            State::Assign => Some(ValidTokenType::Assign),
            State::Or => Some(ValidTokenType::Or),
            State::And => Some(ValidTokenType::And),
            State::Not => Some(ValidTokenType::Not),
            State::OpenPar => Some(ValidTokenType::OpenPar),
            State::ClosePar => Some(ValidTokenType::ClosePar),
            State::OpenCuBr => Some(ValidTokenType::OpenCuBr),
            State::CloseCuBr => Some(ValidTokenType::CloseCuBr),
            State::OpenSqBr => Some(ValidTokenType::OpenSqBr),
            State::CloseSqBr => Some(ValidTokenType::CloseSqBr),
            State::Semi => Some(ValidTokenType::Semi),
            State::Comma => Some(ValidTokenType::Comma),
            State::Dot => Some(ValidTokenType::Dot),
            State::Colon => Some(ValidTokenType::Colon),
            State::ColonColon => Some(ValidTokenType::ColonColon),
            State::Arrow => Some(ValidTokenType::Arrow),

            // keywords
            State::KwIf => Some(ValidTokenType::KwIf),
            State::KwThen => Some(ValidTokenType::KwThen),
            State::KwElse => Some(ValidTokenType::KwElse),
            State::KwInteger => Some(ValidTokenType::KwInteger),
            State::KwFloat => Some(ValidTokenType::KwFloat),
            State::KwVoid => Some(ValidTokenType::KwVoid),
            State::KwPublic => Some(ValidTokenType::KwPublic),
            State::KwPrivate => Some(ValidTokenType::KwPrivate),
            State::KwFunc => Some(ValidTokenType::KwFunc),
            State::KwVar => Some(ValidTokenType::KwVar),
            State::KwStruct => Some(ValidTokenType::KwStruct),
            State::KwWhile => Some(ValidTokenType::KwWhile),
            State::KwRead => Some(ValidTokenType::KwRead),
            State::KwWrite => Some(ValidTokenType::KwWrite),
            State::KwReturn => Some(ValidTokenType::KwReturn),
            State::KwSelf => Some(ValidTokenType::KwSelf),
            State::KwInherits => Some(ValidTokenType::KwInherits),
            State::KwLet => Some(ValidTokenType::KwLet),
            State::KwImpl => Some(ValidTokenType::KwImpl),

            // atomic elements
            State::Id2 => Some(ValidTokenType::Id),
            State::Str3 => Some(ValidTokenType::Str),
            State::Int12 | State::Int13 => Some(ValidTokenType::Integer),
            State::Frac13
            | State::Frac14
            | State::Int22
            | State::Int23
            | State::Int32
            | State::Int33 => Some(ValidTokenType::Float),

            // comments
            State::InlineCmt => Some(ValidTokenType::InlineCmt),
            State::BlockCmt => Some(ValidTokenType::BlockCmt),
            _ => Some(ValidTokenType::Id),
        };

        let invalid_token_type = match state {
            // invalid token types
            State::Frac12 | State::Frac15 | State::Int21 | State::Int31 => {
                Some(InvalidTokenType::InvalidNumber)
            }
            State::Str2 => Some(InvalidTokenType::InvalidStr),
            State::Start => Some(InvalidTokenType::InvalidChar),
            _ => None,
        };

        return if invalid_token_type.is_some() {
            TokenType::InvalidTokenType(invalid_token_type.expect("Invalid token type not found!"))
        } else {
            TokenType::ValidTokenType(valid_token_type.expect("Valid token type not exist!"))
        };
    }
}

impl StateMachineImpl for LexerStateMachineImpl {
    type Input = char;
    type State = State;
    type Output = TokenType;

    const INITIAL_STATE: Self::State = State::Start;

    fn transition(state: &Self::State, input: &Self::Input) -> Option<Self::State> {
        match (state, input) {
            // keywords
            (State::Start, 'i') => Some(State::KwI),

            (State::KwI, 'f') => Some(State::KwIf),

            (State::KwI, 'n') => Some(State::KwIn),

            (State::KwIn, 't') => Some(State::KwInt),
            (State::KwInt, 'e') => Some(State::KwInte),
            (State::KwInte, 'g') => Some(State::KwInteg),
            (State::KwInteg, 'e') => Some(State::KwIntege),
            (State::KwIntege, 'r') => Some(State::KwInteger),

            (State::KwIn, 'h') => Some(State::KwInh),
            (State::KwInh, 'e') => Some(State::KwInhe),
            (State::KwInhe, 'r') => Some(State::KwInher),
            (State::KwInher, 'i') => Some(State::KwInheri),
            (State::KwInheri, 't') => Some(State::KwInherit),
            (State::KwInherit, 's') => Some(State::KwInherits),

            (State::KwI, 'm') => Some(State::KwIm),
            (State::KwIm, 'p') => Some(State::KwImp),
            (State::KwImp, 'l') => Some(State::KwImpl),

            // then
            (State::Start, 't') => Some(State::KwT),
            (State::KwT, 'h') => Some(State::KwTh),
            (State::KwTh, 'e') => Some(State::KwThe),
            (State::KwThe, 'n') => Some(State::KwThen),

            // else
            (State::Start, 'e') => Some(State::KwE),
            (State::KwE, 'l') => Some(State::KwEl),
            (State::KwEl, 's') => Some(State::KwEls),
            (State::KwEls, 'e') => Some(State::KwElse),

            // float, func
            (State::Start, 'f') => Some(State::KwF),
            (State::KwF, 'l') => Some(State::KwFl),
            (State::KwFl, 'o') => Some(State::KwFlo),
            (State::KwFlo, 'a') => Some(State::KwFloa),
            (State::KwFloa, 't') => Some(State::KwFloat),

            (State::KwF, 'u') => Some(State::KwFu),
            (State::KwFu, 'n') => Some(State::KwFun),
            (State::KwFun, 'c') => Some(State::KwFunc),

            // void, var
            (State::Start, 'v') => Some(State::KwV),
            (State::KwV, 'o') => Some(State::KwVo),
            (State::KwVo, 'i') => Some(State::KwVoi),
            (State::KwVoi, 'd') => Some(State::KwVoid),

            (State::KwV, 'a') => Some(State::KwVa),
            (State::KwVa, 'r') => Some(State::KwVar),

            // public, private
            (State::Start, 'p') => Some(State::KwP),
            (State::KwP, 'u') => Some(State::KwPu),
            (State::KwPu, 'b') => Some(State::KwPub),
            (State::KwPub, 'l') => Some(State::KwPubl),
            (State::KwPubl, 'i') => Some(State::KwPubli),
            (State::KwPubli, 'c') => Some(State::KwPublic),

            (State::KwP, 'r') => Some(State::KwPr),
            (State::KwPr, 'i') => Some(State::KwPri),
            (State::KwPri, 'v') => Some(State::KwPriv),
            (State::KwPriv, 'a') => Some(State::KwPriva),
            (State::KwPriva, 't') => Some(State::KwPrivat),
            (State::KwPrivat, 'e') => Some(State::KwPrivate),

            // struct, self
            (State::Start, 's') => Some(State::KwS),
            (State::KwS, 't') => Some(State::KwSt),
            (State::KwSt, 'r') => Some(State::KwStr),
            (State::KwStr, 'u') => Some(State::KwStru),
            (State::KwStru, 'c') => Some(State::KwStruc),
            (State::KwStruc, 't') => Some(State::KwStruct),

            (State::KwS, 'e') => Some(State::KwSe),
            (State::KwSe, 'l') => Some(State::KwSel),
            (State::KwSel, 'f') => Some(State::KwSelf),

            // while, write
            (State::Start, 'w') => Some(State::KwW),
            (State::KwW, 'h') => Some(State::KwWh),
            (State::KwWh, 'i') => Some(State::KwWhi),
            (State::KwWhi, 'l') => Some(State::KwWhil),
            (State::KwWhil, 'e') => Some(State::KwWhile),

            (State::KwW, 'r') => Some(State::KwWr),
            (State::KwWr, 'i') => Some(State::KwWri),
            (State::KwWri, 't') => Some(State::KwWrit),
            (State::KwWrit, 'e') => Some(State::KwWrite),

            // read, return
            (State::Start, 'r') => Some(State::KwR),
            (State::KwR, 'e') => Some(State::KwRe),
            (State::KwRe, 'a') => Some(State::KwRea),
            (State::KwRea, 'd') => Some(State::KwRead),

            (State::KwRe, 't') => Some(State::KwRet),
            (State::KwRet, 'u') => Some(State::KwRetu),
            (State::KwRetu, 'r') => Some(State::KwRetur),
            (State::KwRetur, 'n') => Some(State::KwReturn),

            // let
            (State::Start, 'l') => Some(State::KwL),
            (State::KwL, 'e') => Some(State::KwLe),
            (State::KwLe, 't') => Some(State::KwLet),

            // keywords to id
            (
                State::KwI
                | State::KwIf
                | State::KwIn
                | State::KwInt
                | State::KwInte
                | State::KwInteg
                | State::KwIntege
                | State::KwInteger
                | State::KwInh
                | State::KwInhe
                | State::KwInher
                | State::KwInheri
                | State::KwInherit
                | State::KwInherits
                | State::KwIm
                | State::KwImp
                | State::KwImpl
                | State::KwT
                | State::KwTh
                | State::KwThe
                | State::KwThen
                | State::KwE
                | State::KwEl
                | State::KwEls
                | State::KwElse
                | State::KwF
                | State::KwFl
                | State::KwFlo
                | State::KwFloa
                | State::KwFloat
                | State::KwFu
                | State::KwFun
                | State::KwFunc
                | State::KwV
                | State::KwVo
                | State::KwVoi
                | State::KwVoid
                | State::KwVa
                | State::KwVar
                | State::KwP
                | State::KwPu
                | State::KwPub
                | State::KwPubl
                | State::KwPubli
                | State::KwPublic
                | State::KwPr
                | State::KwPri
                | State::KwPriv
                | State::KwPriva
                | State::KwPrivat
                | State::KwPrivate
                | State::KwS
                | State::KwSt
                | State::KwStr
                | State::KwStru
                | State::KwStruc
                | State::KwStruct
                | State::KwSe
                | State::KwSel
                | State::KwSelf
                | State::KwW
                | State::KwWh
                | State::KwWhi
                | State::KwWhil
                | State::KwWhile
                | State::KwWr
                | State::KwWri
                | State::KwWrit
                | State::KwWrite
                | State::KwR
                | State::KwRe
                | State::KwRea
                | State::KwRead
                | State::KwRet
                | State::KwRetu
                | State::KwRetur
                | State::KwReturn
                | State::KwL
                | State::KwLe
                | State::KwLet,
                'A'..='Z' | 'a'..='z' | '0'..='9' | '_',
            ) => Some(State::Id2),

            // atomic lexical elements
            (State::Start, '0') => Some(State::Int12),
            (State::Start, '1'..='9') => Some(State::Int13),
            (State::Start, 'A'..='Z' | 'a'..='z') => Some(State::Id2),
            (State::Start, '"') => Some(State::Str2),
            (State::Id2, 'A'..='Z' | 'a'..='z' | '0'..='9' | '_') => Some(State::Id2),
            (State::Str2, 'A'..='Z' | 'a'..='z' | '0'..='9' | '_' | ' ') => Some(State::Str2),
            (State::Str2, '"') => Some(State::Str3),
            (State::Int12 | State::Int13, '.') => Some(State::Frac12),
            (State::Int13, '0'..='9') => Some(State::Int13),
            (State::Frac12, '0') => Some(State::Frac14),
            (State::Frac12, '1'..='9') => Some(State::Frac13),
            (State::Frac13, '0') => Some(State::Frac15),
            (State::Frac13, '1'..='9') => Some(State::Frac13),
            (State::Frac14, '0') => Some(State::Frac15),
            (State::Frac14, '1'..='9') => Some(State::Frac14),
            (State::Frac15, '0') => Some(State::Frac15),
            (State::Frac15, '1'..='9') => Some(State::Frac14),
            (State::Frac13 | State::Frac14, 'e') => Some(State::Int21),
            (State::Int21, '0') => Some(State::Int22),
            (State::Int21, '+' | '-') => Some(State::Int31),
            (State::Int21, '1'..='9') => Some(State::Int23),
            (State::Int23, '0'..='9') => Some(State::Int23),
            (State::Int31, '0') => Some(State::Int32),
            (State::Int31, '1'..='9') => Some(State::Int33),
            (State::Int33, '0'..='9') => Some(State::Int33),

            // operators
            (State::Start, '=') => Some(State::Assign),
            (State::Assign, '=') => Some(State::Eq),
            (State::Start, '<') => Some(State::Lt),
            (State::Lt, '>') => Some(State::NotEq),
            (State::Lt, '=') => Some(State::Leq),
            (State::Start, '>') => Some(State::Gt),
            (State::Gt, '=') => Some(State::Geq),
            (State::Start, '+') => Some(State::Plus),
            (State::Start, '-') => Some(State::Minus),
            (State::Minus, '>') => Some(State::Arrow),
            (State::Start, '*') => Some(State::Mult),
            (State::Start, '/') => Some(State::Div),
            (State::Start, '|') => Some(State::Or),
            (State::Start, '&') => Some(State::And),
            (State::Start, '!') => Some(State::Not),
            (State::Start, '(') => Some(State::OpenPar),
            (State::Start, ')') => Some(State::ClosePar),
            (State::Start, '{') => Some(State::OpenCuBr),
            (State::Start, '}') => Some(State::CloseCuBr),
            (State::Start, '[') => Some(State::OpenSqBr),
            (State::Start, ']') => Some(State::CloseSqBr),
            (State::Start, ';') => Some(State::Semi),
            (State::Start, ',') => Some(State::Comma),
            (State::Start, '.') => Some(State::Dot),
            (State::Start, ':') => Some(State::Colon),
            (State::Colon, ':') => Some(State::ColonColon),

            (State::Div, '/') => Some(State::InlineCmt),
            (State::Div, '*') => Some(State::BlockCmt),

            (State::InlineCmt, _) => Some(State::InlineCmt),

            _ => None,
        }
    }

    fn output(state: &Self::State, input: &Self::Input) -> Option<Self::Output> {
        let next_state = Self::transition(state, input).expect("Unhandled transition error");
        Some(LexerStateMachineImpl::state_to_token_type(&next_state))
    }
}
