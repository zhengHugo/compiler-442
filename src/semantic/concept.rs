use crate::lexical::token::{Token, TokenType, ValidTokenType};
use std::fmt::{Display, Formatter};

#[derive(PartialEq)]
pub enum Concept {
    AtomicConcept(AtomicConcept),
    CompositeConcept(CompositeConcept),
}

impl Concept {
    pub(crate) fn from_terminal_token(token: Token) -> Result<Self, ()> {
        match token.token_type {
            TokenType::ValidTokenType(valid_token_type) => match valid_token_type {
                ValidTokenType::Id => Ok(Concept::AtomicConcept(AtomicConcept {
                    atomic_concept_type: AtomicConceptType::Id,
                    value: token.lexeme,
                })),
                ValidTokenType::Float => Ok(Concept::AtomicConcept(AtomicConcept {
                    atomic_concept_type: AtomicConceptType::FloatLit,
                    value: token.lexeme,
                })),
                ValidTokenType::Integer => Ok(Concept::AtomicConcept(AtomicConcept {
                    atomic_concept_type: AtomicConceptType::IntLit,
                    value: token.lexeme,
                })),
                ValidTokenType::Eq
                | ValidTokenType::Geq
                | ValidTokenType::Gt
                | ValidTokenType::Leq
                | ValidTokenType::Lt
                | ValidTokenType::NotEq => Ok(Concept::AtomicConcept(AtomicConcept {
                    atomic_concept_type: AtomicConceptType::RelOp,
                    value: token.lexeme,
                })),
                ValidTokenType::Plus | ValidTokenType::Minus | ValidTokenType::Or => {
                    Ok(Concept::AtomicConcept(AtomicConcept {
                        atomic_concept_type: AtomicConceptType::AddOp,
                        value: token.lexeme,
                    }))
                }
                ValidTokenType::Mult | ValidTokenType::Div | ValidTokenType::And => {
                    Ok(Concept::AtomicConcept(AtomicConcept {
                        atomic_concept_type: AtomicConceptType::MultiOp,
                        value: token.lexeme,
                    }))
                }
                ValidTokenType::KwVoid => Ok(Concept::AtomicConcept(AtomicConcept {
                    atomic_concept_type: AtomicConceptType::Void,
                    value: token.lexeme,
                })),
                ValidTokenType::KwFloat => Ok(Concept::AtomicConcept(AtomicConcept {
                    atomic_concept_type: AtomicConceptType::Float,
                    value: token.lexeme,
                })),
                ValidTokenType::KwInteger => Ok(Concept::AtomicConcept(AtomicConcept {
                    atomic_concept_type: AtomicConceptType::Integer,
                    value: token.lexeme,
                })),
                ValidTokenType::KwPublic | ValidTokenType::KwPrivate => {
                    Ok(Concept::AtomicConcept(AtomicConcept {
                        atomic_concept_type: AtomicConceptType::Visibility,
                        value: token.lexeme,
                    }))
                }
                _ => Err(()),
            },
            TokenType::InvalidTokenType(_) => Err(()),
        }
    }

    pub fn create_sign(token: Token) -> Result<Self, ()> {
        match token.token_type {
            TokenType::ValidTokenType(valid_token_type) => match valid_token_type {
                ValidTokenType::Plus | ValidTokenType::Minus => {
                    Ok(Concept::AtomicConcept(AtomicConcept {
                        atomic_concept_type: AtomicConceptType::Sign,
                        value: token.lexeme,
                    }))
                }
                _ => Err(()),
            },
            _ => Err(()),
        }
    }

    pub fn create_epsilon() -> Self {
        Concept::AtomicConcept(AtomicConcept {
            atomic_concept_type: AtomicConceptType::Epsilon,
            value: "".parse().unwrap(),
        })
    }

    pub fn is_epsilon(&self) -> bool {
        match self {
            Concept::AtomicConcept(atomic_concept) => matches!(
                atomic_concept.atomic_concept_type,
                AtomicConceptType::Epsilon
            ),
            Concept::CompositeConcept(_) => false,
        }
    }
}

impl Display for Concept {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Concept::AtomicConcept(ac) => ac.to_string(),
                Concept::CompositeConcept(cc) => cc.to_string(),
            }
        )
    }
}
#[derive(PartialEq)]
pub struct AtomicConcept {
    pub(crate) atomic_concept_type: AtomicConceptType,
    value: String,
}

impl Display for AtomicConcept {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.atomic_concept_type)
    }
}

#[derive(PartialEq, Debug)]
pub enum AtomicConceptType {
    Id,
    FloatLit,
    Float,
    IntLit,
    Integer,
    Void,
    RelOp,
    MultiOp,
    AddOp,
    Sign,
    Visibility,
    Epsilon,
}

#[derive(PartialEq, Debug)]
pub enum CompositeConcept {
    Dot,
    IndexList,
    Var,
    Assign,
    FuncCall,
    RelExpr,
    AddExpr,
    MultExpr,
    NotExpr,
    SignedExpr,
    IfThenElse,
    Read,
    Return,
    While,
    StmtBlock,
    Write,
    AParams,
    ArraySizes,
    FParam,
    Type,
    FParams,
    FuncDef,
    VarDecl,
    FuncBody,
    FuncDecl,
    FuncDefList,
    ImplDef,
    StructDecl,
    InheritsList,
    StructMemberDeclList,
    StructMemberDecl,
    Prog,
}

impl Display for CompositeConcept {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
