use crate::lexical::token::ValidTokenType::{self, *};
use crate::syntactic::symbol::NonTerminal::*;
use crate::syntactic::symbol::Terminal::*;
use std::fmt::{Display, Formatter};

#[derive(PartialEq, Debug)]
pub enum Symbol {
    NonTerminal(NonTerminal),
    Terminal(Terminal),
}

impl Display for Symbol {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Symbol::NonTerminal(nonterminal) => write!(f, "{:?}", nonterminal),
            Symbol::Terminal(terminal) => write!(f, "{:?}", terminal),
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
        } else if symbol_string.chars().any(|c| c.is_uppercase()) {
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
        } else {
            panic!("Unexpected nonterminal symbol string {}", symbol_string)
        }
    }
}
