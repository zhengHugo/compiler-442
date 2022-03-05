use crate::lexical::token::ValidTokenType::{self, *};
use crate::syntactic::symbol::NonTerminal::*;
use crate::syntactic::symbol::Terminal::*;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(PartialEq, Debug)]
pub enum Symbol {
    NonTerminal(NonTerminal),
    Terminal(Terminal),
    ActionSymbol(ActionSymbol),
}

impl Display for Symbol {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Symbol::NonTerminal(nonterminal) => write!(f, "{:?}", nonterminal),
            Symbol::Terminal(terminal) => write!(f, "{:?}", terminal),
            Symbol::ActionSymbol(action) => write!(f, "{:?}", action),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum Terminal {
    ValidTokenType(ValidTokenType),
    EPSILON,
    EOF,
}

impl Display for Terminal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Terminal::ValidTokenType(valid_token_type) => {
                write!(f, "{}", valid_token_type.to_string())
            }
            other => write!(f, "{:?}", other),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum NonTerminal {
    Start,
    AddOp,
    AParams,
    AParamsTail,
    ArithExpr,
    ArraySize,
    ArraySize2,
    AssignOp,
    Expr,
    Expr2,
    Factor,
    Factor2,
    FParams,
    FParamsTail,
    FuncBody,
    FuncDecl,
    FuncDef,
    FuncHead,
    IdNest,
    IdNest2,
    ImplDef,
    Index,
    MemberDecl,
    MultOp,
    OptStructDecl2,
    Prog,
    RelExpr,
    RelOp,
    ReptAParams1,
    ReptFParams3,
    ReptFParams4,
    ReptFParamsTail4,
    ReptFuncBody1,
    ReptFuncCallOrVar2,
    ReptIdNest1,
    ReptImplDef3,
    ReptIndices0,
    ReptOptStructDecl22,
    ReptProg0,
    ReptStatBlock1,
    ReptStructDecl4,
    ReptVarDecl4,
    ReptVariable,
    ReptVarOrFuncCall,
    ReturnType,
    RightRecArithExpr,
    RightRecTerm,
    Sign,
    StatBlock,
    Statement,
    StatementIdNest,
    StatementIdNest2,
    StatementIdNest3,
    StructDecl,
    StructOrImplOrFunc,
    Term,
    Type,
    VarDecl,
    VarDeclOrStat,
    Variable,
    Variable2,
    VarIdNest,
    VarIdNest2,

    Visibility,
}

impl Display for NonTerminal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Symbol {
    pub fn from_string(symbol_string: &str) -> Symbol {
        if symbol_string.contains("epsilon") {
            Symbol::Terminal(EPSILON)
        } else if symbol_string.contains("$") {
            Symbol::Terminal(EOF)
        } else if symbol_string.chars().any(|c| c.is_lowercase()) {
            // this is a terminal
            Symbol::Terminal(Terminal::ValidTokenType(match symbol_string {
                "private" => KwPrivate,
                "public" => KwPublic,
                "semi" => Semi,
                "colon" => Colon,
                "id" => Id,
                "let" => KwLet,
                "float" => KwFloat,
                "integer" => KwInteger,
                "rcurbr" => CloseCuBr,
                "lcurbr" => OpenCuBr,
                "struct" => KwStruct,
                "rpar" => ClosePar,
                "lpar" => OpenPar,
                "return" => KwReturn,
                "write" => KwWrite,
                "read" => KwRead,
                "while" => KwWhile,
                "else" => KwElse,
                "then" => KwThen,
                "if" => KwIf,
                "minus" => Minus,
                "plus" => Plus,
                "void" => KwVoid,
                "comma" => Comma,
                "geq" => Geq,
                "leq" => Leq,
                "gt" => Gt,
                "lt" => Lt,
                "neq" => NotEq,
                "eq" => Eq,
                "inherits" => KwInherits,
                "and" => And,
                "div" => Div,
                "mult" => Mult,
                "dot" => Dot,
                "rsqbr" => CloseSqBr,
                "lsqbr" => OpenSqBr,
                "impl" => KwImpl,
                "arrow" => Arrow,
                "func" => KwFunc,
                "not" => Not,
                "floatlit" => Float,
                "intlit" => Integer,
                "equal" => Assign,
                "or" => Or,
                bad_string => panic!("Unexpected terminal symbol string {}", bad_string),
            }))
        } else if symbol_string.len() > 2 {
            Symbol::NonTerminal(match symbol_string {
                "START" => Start,
                "ADDOP" => AddOp,
                "APARAMS" => AParams,
                "APARAMSTAIL" => AParamsTail,
                "ARITHEXPR" => ArithExpr,
                "ARRAYSIZE" => ArraySize,
                "ARRAYSIZE2" => ArraySize2,
                "ASSIGNOP" => AssignOp,
                "EXPR" => Expr,
                "EXPR2" => Expr2,
                "FACTOR" => Factor,
                "FACTOR2" => Factor2,
                "FPARAMS" => FParams,
                "FPARAMSTAIL" => FParamsTail,
                "FUNCBODY" => FuncBody,
                "FUNCDECL" => FuncDecl,
                "FUNCDEF" => FuncDef,
                "FUNCHEAD" => FuncHead,
                "IDNEST" => IdNest,
                "IDNEST2" => IdNest2,
                "IMPLDEF" => ImplDef,
                "INDICE" => Index,
                "MEMBERDECL" => MemberDecl,
                "MULTOP" => MultOp,
                "OPTSTRUCTDECL2" => OptStructDecl2,
                "PROG" => Prog,
                "RELEXPR" => RelExpr,
                "RELOP" => RelOp,
                "REPTAPARAMS1" => ReptAParams1,
                "REPTFPARAMS3" => ReptFParams3,
                "REPTFPARAMS4" => ReptFParams4,
                "REPTFPARAMSTAIL4" => ReptFParamsTail4,
                "REPTFUNCBODY1" => ReptFuncBody1,
                "REPTIDNEST1" => ReptIdNest1,
                "REPTIMPLDEF3" => ReptImplDef3,
                "REPTINDICES0" => ReptIndices0,
                "REPTOPTSTRUCTDECL22" => ReptOptStructDecl22,
                "REPTPROG0" => ReptProg0,
                "REPTSTATBLOCK1" => ReptStatBlock1,
                "REPTSTRUCTDECL4" => ReptStructDecl4,
                "REPTVARDECL4" => ReptVarDecl4,
                "REPTVARIABLE" => ReptVariable,
                "REPTVARORFUNCCALL" => ReptVarOrFuncCall,
                "RETURNTYPE" => ReturnType,
                "RIGHTRECARITHEXPR" => RightRecArithExpr,
                "RIGHTRECTERM" => RightRecTerm,
                "SIGN" => Sign,
                "STATBLOCK" => StatBlock,
                "STATEMENT" => Statement,
                "STATEMENTIDNEST" => StatementIdNest,
                "STATEMENTIDNEST2" => StatementIdNest2,
                "STATEMENTIDNEST3" => StatementIdNest3,
                "STRUCTDECL" => StructDecl,
                "STRUCTORIMPLORFUNC" => StructOrImplOrFunc,
                "TERM" => Term,
                "TYPE" => Type,
                "VARDECL" => VarDecl,
                "VARDECLORSTAT" => VarDeclOrStat,
                "VARIABLE" => Variable,
                "VARIABLE2" => Variable2,
                "VARIDNEST" => VarIdNest,
                "VARIDNEST2" => VarIdNest2,
                "VISIBILITY" => Visibility,
                bad_string => panic!("Unexpected nonterminal symbol string {}", bad_string),
            })
        } else if symbol_string.len() <= 2 {
            Symbol::ActionSymbol(ActionSymbol::from_str(symbol_string).unwrap())
        } else {
            panic!("Unexpected nonterminal symbol string {}", symbol_string)
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum ActionSymbol {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    A1,
    A2,
    A3,
    A4,
    A5,
    A6,
    A7,
    A8,
    A9,
    B1,
    B2,
    B3,
    B4,
    B5,
    B6,
    B7,
    B8,
}

impl FromStr for ActionSymbol {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(ActionSymbol::A),
            "B" => Ok(ActionSymbol::B),
            "C" => Ok(ActionSymbol::C),
            "D" => Ok(ActionSymbol::D),
            "E" => Ok(ActionSymbol::E),
            "F" => Ok(ActionSymbol::F),
            "G" => Ok(ActionSymbol::G),
            "H" => Ok(ActionSymbol::H),
            "I" => Ok(ActionSymbol::I),
            "J" => Ok(ActionSymbol::J),
            "K" => Ok(ActionSymbol::K),
            "L" => Ok(ActionSymbol::L),
            "M" => Ok(ActionSymbol::M),
            "N" => Ok(ActionSymbol::N),
            "O" => Ok(ActionSymbol::O),
            "P" => Ok(ActionSymbol::P),
            "Q" => Ok(ActionSymbol::Q),
            "R" => Ok(ActionSymbol::R),
            "S" => Ok(ActionSymbol::S),
            "T" => Ok(ActionSymbol::T),
            "U" => Ok(ActionSymbol::U),
            "V" => Ok(ActionSymbol::V),
            "W" => Ok(ActionSymbol::W),
            "X" => Ok(ActionSymbol::X),
            "Y" => Ok(ActionSymbol::Y),
            "Z" => Ok(ActionSymbol::Z),
            "A1" => Ok(ActionSymbol::A1),
            "A2" => Ok(ActionSymbol::A2),
            "A3" => Ok(ActionSymbol::A3),
            "A4" => Ok(ActionSymbol::A4),
            "A5" => Ok(ActionSymbol::A5),
            "A6" => Ok(ActionSymbol::A6),
            "A7" => Ok(ActionSymbol::A7),
            "A8" => Ok(ActionSymbol::A8),
            "A9" => Ok(ActionSymbol::A9),
            "B1" => Ok(ActionSymbol::B1),
            "B2" => Ok(ActionSymbol::B2),
            "B3" => Ok(ActionSymbol::B3),
            "B4" => Ok(ActionSymbol::B4),
            "B5" => Ok(ActionSymbol::B5),
            "B6" => Ok(ActionSymbol::B6),
            "B7" => Ok(ActionSymbol::B7),
            "B8" => Ok(ActionSymbol::B8),
            _ => Err(()),
        }
    }
}
