pub struct Node<T>
where
    T: PartialEq,
{
    value: T,
    parent: Option<NodeId>,
    children: Vec<NodeId>,
}

impl<T> Node<T>
where
    T: PartialEq,
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

pub struct Tree<T>
where
    T: PartialEq,
{
    arena: Arena<T>,
}

impl<T> Tree<T>
where
    T: PartialEq,
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
        self.arena.nodes.len()
    }

    pub fn get_node_value(&self, node_id: NodeId) -> &T {
        self.arena.nodes[node_id as usize].get_value()
    }

    pub fn size(&self) -> usize {
        self.arena.nodes.len()
    }
}

struct Arena<T>
where
    T: PartialEq,
{
    nodes: Vec<Node<T>>,
}

impl<T> Arena<T>
where
    T: PartialEq,
{
    fn new() -> Self {
        Self { nodes: vec![] }
    }
}

pub type NodeId = usize;
