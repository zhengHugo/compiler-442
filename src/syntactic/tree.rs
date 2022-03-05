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
        self.arena.nodes.len() - 1
    }

    pub fn move_node_under(&mut self, child: NodeId, parent: Option<NodeId>) {
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
                .push(child);
        }
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
