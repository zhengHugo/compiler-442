use std::fmt::{Display, Formatter};

struct Node<T>
where
    T: PartialEq + Display,
{
    value: T,
    parent: Option<NodeId>,
    children: Vec<NodeId>,
}

impl<T> Node<T>
where
    T: PartialEq + Display,
{
    fn new(value: T) -> Self {
        Self {
            value,
            parent: None,
            children: vec![],
        }
    }

    pub fn get_value(&self) -> &T {
        &self.value
    }
}

impl<T> Display for Node<T>
where
    T: PartialEq + Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

pub struct Tree<T>
where
    T: PartialEq + Display,
{
    arena: Arena<T>,
}

impl<T> Tree<T>
where
    T: PartialEq + Display,
{
    pub fn new() -> Self {
        Self {
            arena: Arena::new(),
        }
    }

    pub fn insert_node(&mut self, parent: Option<NodeId>, value: T) -> NodeId {
        self.arena.nodes.push(Node {
            value,
            parent,
            children: vec![],
        });
        self.arena.nodes.len() - 1
    }

    /// move a node child under another node parent.
    /// Child is prepended to the children list of parent
    pub fn move_node_under_prepend(&mut self, child: NodeId, parent: Option<NodeId>) {
        // remove from old parent
        if let Some(old_parent_id) = self.arena.nodes[child as usize].parent {
            let pos = self.arena.nodes[old_parent_id]
                .children
                .iter()
                .position(|id| *id == old_parent_id)
                .unwrap();
            self.arena.nodes[old_parent_id].children.remove(pos);
        }
        // add pointer from child to parent
        self.arena.nodes[child as usize].parent = parent;
        // add pointer from parent to child
        if parent.is_some() {
            self.arena.nodes[parent.unwrap() as usize]
                .children
                .insert(0, child)
        }
    }

    pub fn get_node_value(&self, node_id: NodeId) -> &T {
        self.arena.nodes[node_id as usize].get_value()
    }

    pub fn size(&self) -> usize {
        self.arena.nodes.len()
    }

    pub fn get_root(&self) -> NodeId {
        let mut id = self.arena.nodes.len() - 1;
        while self.arena.nodes[id].parent.is_some() {
            id = self.arena.nodes[id].parent.unwrap();
        }
        id as NodeId
    }

    pub fn get_children(&self, parent_id: NodeId) -> Vec<NodeId> {
        self.arena.nodes[parent_id].children.clone()
    }

    fn to_string_from_node(&self, from: &NodeId, depth: usize) -> String {
        let mut result = String::from("");
        result.push_str(&*"| ".repeat(depth));
        result.push_str(&self.arena.nodes[*from as usize].to_string());
        result.push_str(&*"\n");
        for child in self.arena.nodes[*from as usize].children.iter() {
            result.push_str(&*self.to_string_from_node(child, depth + 1))
        }
        result
    }
}

impl<T> Display for Tree<T>
where
    T: PartialEq + Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string_from_node(&self.get_root(), 0))
    }
}

struct Arena<T>
where
    T: PartialEq + Display,
{
    nodes: Vec<Node<T>>,
}

impl<T> Arena<T>
where
    T: PartialEq + Display,
{
    fn new() -> Self {
        Self { nodes: vec![] }
    }
}

pub type NodeId = usize;
