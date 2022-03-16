use crate::semantic::concept::Concept;
use crate::syntactic::tree::{NodeId, Tree};

pub trait Visitable {
    fn visit(&self);
}
